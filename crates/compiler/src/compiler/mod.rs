use std::{
    collections::HashMap,
    fmt::Debug,
    io::{Seek, Write},
    mem::{self},
};

use phf::{phf_map, Map};

use crate::parser::{BareModule, BracketType, Node};

use self::{
    error::{Error, Result},
    intermediary::{Insn, IntermediaryStaticValue, Reg},
    mod_type::{assembly::AssemblyLanguage, code::CodeLanguage},
};

pub mod error;
pub mod intermediary;
pub mod mod_type;

pub const DIALECTS: Map<&str, fn() -> Box<dyn Dialect>> = phf_map! {
    "assembly" => || Box::<AssemblyLanguage>::default(),
    "code" => || Box::<CodeLanguage>::default(),
};

pub trait Dialect {
    fn collect(
        &mut self,
        task: &mut CompileTask,
        module_index: usize,
        module: UncollectedModule,
        main: bool,
    ) -> Result<()>;

    fn compile_module(&mut self, task: &mut CompileTask, module_index: usize) -> Result<()>;
}

#[derive(Debug)]
pub struct CompileTask {
    pub module_indices: HashMap<String, usize>,
    pub modules: Vec<Module_>,
    pub status: Status,
    pub main: Option<(usize, usize)>,
}

impl Default for CompileTask {
    fn default() -> Self {
        Self {
            module_indices: HashMap::with_capacity(0),
            modules: Vec::with_capacity(0),
            status: Status::Open,
            main: None,
        }
    }
}

impl CompileTask {
    pub fn include(
        &mut self,
        BareModule {
            name,
            file,
            src,
            root,
        }: BareModule,
        main: bool,
    ) -> Result<()> {
        if self.status != Status::Open || main && self.main.is_some() {
            return Err(Error::InvalidOperation);
        }
        self.status = Status::Invalid;
        if self.module_indices.contains_key(&name) {
            return Err(Error::DuplicateModule { file, name });
        }
        if root.is_empty() {
            return Err(Error::EmptyModule { file, name });
        }
        let Node::Node {
            span: mod_decl_span,
            type_: BracketType::Round,
            sub_nodes: mod_sub_nodes,
        } = &root[0] else
        {
            panic!("Invalid AST");
        };
        if mod_sub_nodes.len() != 2 {
            return Err(Error::InvalidModuleDeclaration {
                file,
                src,
                span: mod_decl_span.clone(),
            });
        }
        let keyword_node = &mod_sub_nodes[0];
        let ident_node = &mod_sub_nodes[1];
        let Node::Ident { span: keyword_span } = keyword_node else {
            return Err(Error::InvalidModuleDeclaration {
                file,
                src,
                span: keyword_node.span(),
            });
        };
        let keyword = &src[keyword_span.clone()];
        if keyword != "mod" {
            return Err(Error::InvalidModuleDeclaration {
                file,
                src,
                span: keyword_span.clone(),
            });
        }
        let Node::Ident { span: ident_span } = ident_node else {
            return Err(Error::InvalidModuleDeclaration {
                file,
                src,
                span: ident_node.span(),
            });
        };
        let ident = &src[ident_span.clone()];
        let Some(dialect_supplier) = DIALECTS.get(ident) else {
            return Err(Error::UnknownModuleDialect { file, src, span: ident_span.clone() });
        };
        let dialect = dialect_supplier();
        let module_index = self.modules.len();
        self.modules.push(Module_::new(file, src, dialect));
        self.module_indices.insert(name, module_index);
        let mut dialect = self.modules[module_index].take_dialect();
        dialect.collect(self, module_index, UncollectedModule { root }, main)?;
        self.modules[module_index].dialect = Some(dialect);
        self.status = Status::Open;
        Ok(())
    }

    pub fn compile(&mut self) -> Result<()> {
        if self.status != Status::Open {
            return Err(Error::InvalidOperation);
        }
        self.status = Status::Invalid;
        for i in 0..self.modules.len() {
            let mut dialect = self.modules[i].take_dialect();
            dialect.compile_module(self, i)?;
            self.modules[i].dialect = Some(dialect);
        }
        self.status = Status::Compiled;
        Ok(())
    }

    pub fn filter(&mut self) -> Result<()> {
        if self.status != Status::Compiled {
            return Err(Error::InvalidOperation);
        }
        self.status = Status::Invalid;
        assert!(self.main.is_some());
        let _main = self.main.unwrap();
        for _module in &mut self.modules {
            // module.used = true;
            // for static_ in &mut module.statics {
            //     static_.used = true;
            // }
            // for func in &mut module.funcs {
            //     func.used = true;
            // }
        }
        // todo!();
        self.status = Status::Filtered;
        Ok(())
    }

    pub fn assemble(&mut self, _out: &mut (impl Write + Seek)) -> Result<()> {
        todo!();
        // if self.status != Status::Compiled && self.status != Status::Filtered {
        //     return Err(Error::InvalidOperation);
        // }
        // self.status = Status::Invalid;
        // assert!(self.main.is_some());
        // let main = self.main.unwrap();
        // // Magic
        // out.write_all(b"\0urb")?;
        // // Flags
        // out.write_u32::<LittleEndian>(EXECUTABLE)?;
        // // Entrypoint offset
        // out.write_u64::<LittleEndian>(0)?;
        // let mut ptr = 0;
        // let mut post_procs = Vec::with_capacity(0);
        // let mut modules = HashMap::with_capacity(0);
        // for (i, module) in self.modules.iter().enumerate() {
        //     if !module.used {
        //         continue;
        //     }
        //     let mut statics = HashMap::with_capacity(0);
        //     let mut funcs = HashMap::with_capacity(0);
        //     for (i, static_) in module.statics.iter().enumerate() {
        //         if !static_.used {
        //             continue;
        //         }
        //         statics.insert(i, ptr);
        //         let StaticData::Intermediary { value } = &static_.data else {unreachable!()};
        //         match value {
        //             IntermediaryStaticValue::Int(value) => {
        //                 out.write_i64::<LittleEndian>(*value)?;
        //                 ptr += 8;
        //             }
        //             IntermediaryStaticValue::UInt(value) => {
        //                 out.write_u64::<LittleEndian>(*value)?;
        //                 ptr += 8;
        //             }
        //             IntermediaryStaticValue::Float(value) => {
        //                 out.write_f64::<LittleEndian>(*value)?;
        //                 ptr += 8;
        //             }
        //             IntermediaryStaticValue::String(value) => {
        //                 out.write_all(value.as_bytes())?;
        //                 out.write_u8(0)?;
        //                 ptr += value.len() + 1;
        //                 for _ in 0..alignment(value.len() + 1, 4) {
        //                     out.write_u8(0)?;
        //                     ptr += 1;
        //                 }
        //             }
        //             IntermediaryStaticValue::Buffer { size } => {
        //                 for _ in 0..align(*size, 4) {
        //                     out.write_u8(0)?;
        //                     ptr += 1;
        //                 }
        //             }
        //         }
        //     }
        //     for (j, func) in module.funcs.iter().enumerate() {
        //         if !func.used {
        //             continue;
        //         }
        //         funcs.insert(j, ptr);
        //         let FuncData::Intermediary { ir } = &func.data else {unreachable!()};
        //         let mut points = HashMap::with_capacity(0);
        //         let mut inner_post_procs = Vec::with_capacity(0);
        //         for insn in ir {
        //             match insn {
        //                 Insn::Raw(opc) => {
        //                     out.write_u32::<LittleEndian>(*opc)?;
        //                     ptr += 4;
        //                 }
        //                 Insn::LdStaticAbsAddr { dst, index } => {
        //                     if i != 0 || *index != 0 {
        //                         post_procs.push(PostProc::LdStaticAbsAddr {
        //                             ptr,
        //                             dst: *dst,
        //                             module_index: i,
        //                             static_index: *index,
        //                         });
        //                     }
        //                     out.write_u32::<LittleEndian>(L4_LDBO | *dst as u32)?;
        //                     ptr += 4;
        //                     if i != 0 || *index != 0 {
        //                         out.write_u32::<LittleEndian>(0)?;
        //                         ptr += 4;
        //                     }
        //                 }
        //                 Insn::LoadStatic { dst, index } => {
        //                     post_procs.push(PostProc::LdStaticValue {
        //                         ptr,
        //                         dst: *dst,
        //                         module_index: i,
        //                         static_index: *index,
        //                     });
        //                     out.write_u32::<LittleEndian>(0)?;
        //                     ptr += 4;
        //                 }
        //                 Insn::BranchLabelLinked {
        //                     module_index,
        //                     func_index,
        //                 } => {
        //                     post_procs.push(PostProc::BrLabelLinked {
        //                         ptr,
        //                         module_index: *module_index,
        //                         func_index: *func_index,
        //                     });
        //                     out.write_u32::<LittleEndian>(0)?;
        //                     ptr += 4;
        //                 }
        //                 Insn::Ret => {
        //                     out.write_u32::<LittleEndian>(L5_RET)?;
        //                     ptr += 4;
        //                 }
        //                 Insn::CreatePoint { pos } => {
        //                     points.insert(*pos, ptr);
        //                 }
        //                 Insn::BranchPoint { pos } => {
        //                     inner_post_procs.push(InnerPostProc::Branch { ptr, pos: *pos });
        //                     out.write_u32::<LittleEndian>(0)?;
        //                     ptr += 4;
        //                 }
        //                 Insn::BranchPointIfEq { pos, reg } => {
        //                     inner_post_procs.push(InnerPostProc::BranchPointIfEq {
        //                         ptr,
        //                         pos: *pos,
        //                         reg: *reg,
        //                     });
        //                     out.write_u32::<LittleEndian>(0)?;
        //                     ptr += 4;
        //                 }
        //                 Insn::BranchPointIfNeq { pos, reg } => {
        //                     inner_post_procs.push(InnerPostProc::BranchPointIfNeq {
        //                         ptr,
        //                         pos: *pos,
        //                         reg: *reg,
        //                     });
        //                     out.write_u32::<LittleEndian>(0)?;
        //                     ptr += 4;
        //                 }
        //                 Insn::BranchPointIfLt { pos, reg } => {
        //                     inner_post_procs.push(InnerPostProc::BranchPointIfLt {
        //                         ptr,
        //                         pos: *pos,
        //                         reg: *reg,
        //                     });
        //                     out.write_u32::<LittleEndian>(0)?;
        //                     ptr += 4;
        //                 }
        //                 Insn::BranchPointIfGt { pos, reg } => {
        //                     inner_post_procs.push(InnerPostProc::BranchPointIfGt {
        //                         ptr,
        //                         pos: *pos,
        //                         reg: *reg,
        //                     });
        //                     out.write_u32::<LittleEndian>(0)?;
        //                     ptr += 4;
        //                 }
        //                 Insn::BranchPointIfLeq { pos, reg } => {
        //                     inner_post_procs.push(InnerPostProc::BranchPointIfLeq {
        //                         ptr,
        //                         pos: *pos,
        //                         reg: *reg,
        //                     });
        //                     out.write_u32::<LittleEndian>(0)?;
        //                     ptr += 4;
        //                 }
        //                 Insn::BranchPointIfGeq { pos, reg } => {
        //                     inner_post_procs.push(InnerPostProc::BranchPointIfGeq {
        //                         ptr,
        //                         pos: *pos,
        //                         reg: *reg,
        //                     });
        //                     out.write_u32::<LittleEndian>(0)?;
        //                     ptr += 4;
        //                 }
        //                 Insn::BranchPointIfNz { pos, reg } => {
        //                     inner_post_procs.push(InnerPostProc::BranchPointIfNz {
        //                         ptr,
        //                         pos: *pos,
        //                         reg: *reg,
        //                     });
        //                     out.write_u32::<LittleEndian>(0)?;
        //                     ptr += 4;
        //                 }
        //                 Insn::BranchPointIfZr { pos, reg } => {
        //                     inner_post_procs.push(InnerPostProc::BranchPointIfZr {
        //                         ptr,
        //                         pos: *pos,
        //                         reg: *reg,
        //                     });
        //                     out.write_u32::<LittleEndian>(0)?;
        //                     ptr += 4;
        //                 }
        //             }
        //         }
        //         let saved_ptr = ptr;
        //         for inner_post_proc in inner_post_procs {
        //             match inner_post_proc {
        //                 InnerPostProc::Branch { ptr, pos } => {
        //                     out.seek(SeekFrom::Start(16 + ptr as u64))?;
        //                     let func_ptr = points[&pos] as isize;
        //                     let offset = (func_ptr - ptr as isize) / 4;
        //                     out.write_u32::<LittleEndian>(
        //                         L0_BRANCH | (offset as u32 & ((1 << 27) - 1)),
        //                     )?;
        //                 }
        //                 InnerPostProc::BranchPointIfEq { ptr, pos, reg } => {
        //                     out.seek(SeekFrom::Start(16 + ptr as u64))?;
        //                     let func_ptr = points[&pos] as isize;
        //                     let offset = (func_ptr - ptr as isize) / 4;
        //                     out.write_u32::<LittleEndian>(
        //                         L0_BRANCH_EQ | offset as u32 & ((1 << 22) - 1) | (reg as u32) << 22,
        //                     )?;
        //                 }
        //                 InnerPostProc::BranchPointIfNeq { ptr, pos, reg } => {
        //                     out.seek(SeekFrom::Start(16 + ptr as u64))?;
        //                     let func_ptr = points[&pos] as isize;
        //                     let offset = (func_ptr - ptr as isize) / 4;
        //                     out.write_u32::<LittleEndian>(
        //                         L0_BRANCH_NE | offset as u32 & ((1 << 22) - 1) | (reg as u32) << 22,
        //                     )?;
        //                 }
        //                 InnerPostProc::BranchPointIfLt { ptr, pos, reg } => {
        //                     out.seek(SeekFrom::Start(16 + ptr as u64))?;
        //                     let func_ptr = points[&pos] as isize;
        //                     let offset = (func_ptr - ptr as isize) / 4;
        //                     out.write_u32::<LittleEndian>(
        //                         L0_BRANCH_LT | offset as u32 & ((1 << 22) - 1) | (reg as u32) << 22,
        //                     )?;
        //                 }
        //                 InnerPostProc::BranchPointIfGt { ptr, pos, reg } => {
        //                     out.seek(SeekFrom::Start(16 + ptr as u64))?;
        //                     let func_ptr = points[&pos] as isize;
        //                     let offset = (func_ptr - ptr as isize) / 4;
        //                     out.write_u32::<LittleEndian>(
        //                         L0_BRANCH_GT | offset as u32 & ((1 << 22) - 1) | (reg as u32) << 22,
        //                     )?;
        //                 }
        //                 InnerPostProc::BranchPointIfLeq { ptr, pos, reg } => {
        //                     out.seek(SeekFrom::Start(16 + ptr as u64))?;
        //                     let func_ptr = points[&pos] as isize;
        //                     let offset = (func_ptr - ptr as isize) / 4;
        //                     out.write_u32::<LittleEndian>(
        //                         L0_BRANCH_LE | offset as u32 & ((1 << 22) - 1) | (reg as u32) << 22,
        //                     )?;
        //                 }
        //                 InnerPostProc::BranchPointIfGeq { ptr, pos, reg } => {
        //                     out.seek(SeekFrom::Start(16 + ptr as u64))?;
        //                     let func_ptr = points[&pos] as isize;
        //                     let offset = (func_ptr - ptr as isize) / 4;
        //                     out.write_u32::<LittleEndian>(
        //                         L0_BRANCH_GE | offset as u32 & ((1 << 22) - 1) | (reg as u32) << 22,
        //                     )?;
        //                 }
        //                 InnerPostProc::BranchPointIfNz { ptr, pos, reg } => {
        //                     out.seek(SeekFrom::Start(16 + ptr as u64))?;
        //                     let func_ptr = points[&pos] as isize;
        //                     let offset = (func_ptr - ptr as isize) / 4;
        //                     out.write_u32::<LittleEndian>(
        //                         L0_BRANCH_NZ | offset as u32 & ((1 << 22) - 1) | (reg as u32) << 22,
        //                     )?;
        //                 }
        //                 InnerPostProc::BranchPointIfZr { ptr, pos, reg } => {
        //                     out.seek(SeekFrom::Start(16 + ptr as u64))?;
        //                     let func_ptr = points[&pos] as isize;
        //                     let offset = (func_ptr - ptr as isize) / 4;
        //                     out.write_u32::<LittleEndian>(
        //                         L0_BRANCH_ZR | offset as u32 & ((1 << 22) - 1) | (reg as u32) << 22,
        //                     )?;
        //                 }
        //             }
        //         }
        //         out.seek(SeekFrom::Start(16 + saved_ptr as u64))?;
        //     }
        //     modules.insert(i, (statics, funcs));
        // }
        // let main_offset = modules[&main.0].1[&main.1];
        // out.seek(SeekFrom::Start(8))?;
        // out.write_u64::<LittleEndian>(main_offset as u64)?;
        // for post_proc in post_procs {
        //     match post_proc {
        //         PostProc::LdStaticAbsAddr {
        //             ptr,
        //             dst,
        //             module_index,
        //             static_index,
        //         } => {
        //             out.seek(SeekFrom::Start(16 + 4 + ptr as u64))?;
        //             let static_ptr = modules[&module_index].0[&static_index];
        //             out.write_u32::<LittleEndian>(
        //                 L0_ADD
        //                     | dst as u32
        //                     | (dst as u32) << 5
        //                     | (static_ptr as u32 & ((1 << 17) - 1)) << 10,
        //             )?;
        //         }
        //         PostProc::LdStaticValue {
        //             ptr,
        //             dst,
        //             module_index,
        //             static_index,
        //         } => {
        //             out.seek(SeekFrom::Start(16 + ptr as u64))?;
        //             let StaticData::Intermediary { value } =
        //                 &self.modules[module_index].statics[static_index].data else
        //             {
        //                 unreachable!()
        //             };
        //             match value {
        //                 IntermediaryStaticValue::Int(_)
        //                 | IntermediaryStaticValue::UInt(_)
        //                 | IntermediaryStaticValue::Float(_) => {
        //                     let static_ptr = modules[&module_index].0[&static_index] as isize;
        //                     let offset = (static_ptr - ptr as isize) / 4;
        //                     out.write_u32::<LittleEndian>(
        //                         L0_LDR | dst as u32 | (offset as u32 & ((1 << 22) - 1)) << 5,
        //                     )?;
        //                 }
        //                 IntermediaryStaticValue::String(_) => todo!("Load string"),
        //                 IntermediaryStaticValue::Buffer { size: _ } => todo!("Load buffer"),
        //             }
        //         }
        //         PostProc::BrLabelLinked {
        //             ptr,
        //             module_index,
        //             func_index,
        //         } => {
        //             out.seek(SeekFrom::Start(16 + ptr as u64))?;
        //             let func_ptr = modules[&module_index].1[&func_index] as isize;
        //             let offset = (func_ptr - ptr as isize) / 4;
        //             out.write_u32::<LittleEndian>(L0_BRANCH_L | (offset as u32 & ((1 << 27) - 1)))?;
        //         }
        //     }
        // }
        // self.status = Status::Complete;
        // Ok(())
    }
}

pub struct Module_ {
    pub file: String,
    pub src: String,
    pub dialect: Option<Box<dyn Dialect>>,
}

impl Debug for Module_ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Module")
            .field("file", &self.file)
            .field("src", &self.src)
            .finish()
    }
}

impl Module_ {
    pub fn new(file: String, src: String, dialect: Box<dyn Dialect>) -> Self {
        Self {
            file,
            src,
            dialect: Some(dialect),
        }
    }

    pub fn take_file(&mut self) -> String {
        mem::replace(&mut self.file, String::with_capacity(0))
    }

    pub fn take_src(&mut self) -> String {
        mem::replace(&mut self.src, String::with_capacity(0))
    }

    pub fn take_dialect(&mut self) -> Box<dyn Dialect> {
        self.dialect.take().unwrap()
    }
}

#[derive(Debug)]
pub struct Func {
    pub public: bool,
    pub params: Vec<(Option<String>, Type)>,
    pub return_: Type,
    pub data: FuncData,
    pub used: bool,
}

#[derive(Debug)]
pub struct Static {
    pub data: StaticData,
    pub used: bool,
}

#[derive(Debug)]
pub enum FuncData {
    Collected { node: Node },
    Intermediary { ir: Vec<Insn> },
}

#[derive(Debug)]
pub enum StaticData {
    Collected { node: Node },
    Intermediary { value: IntermediaryStaticValue },
}

#[derive(Debug)]
pub enum Type {
    Unit,
    Int,
    UInt,
    Float,
    String,
    Unknown,
}

#[derive(Debug, PartialEq)]
pub enum Status {
    Open,
    Collected,
    Compiled,
    Filtered,
    Complete,
    Invalid,
}

#[derive(Debug)]
pub struct UncollectedModule {
    pub root: Vec<Node>,
}

pub enum PostProc {
    LdStaticAbsAddr {
        ptr: usize,
        dst: Reg,
        module_index: usize,
        static_index: usize,
    },
    LdStaticValue {
        ptr: usize,
        dst: Reg,
        module_index: usize,
        static_index: usize,
    },
    BrLabelLinked {
        ptr: usize,
        module_index: usize,
        func_index: usize,
    },
}

pub enum InnerPostProc {
    Branch { ptr: usize, pos: usize },
    BranchPointIfEq { ptr: usize, pos: usize, reg: Reg },
    BranchPointIfNeq { ptr: usize, pos: usize, reg: Reg },
    BranchPointIfLt { ptr: usize, pos: usize, reg: Reg },
    BranchPointIfGt { ptr: usize, pos: usize, reg: Reg },
    BranchPointIfLeq { ptr: usize, pos: usize, reg: Reg },
    BranchPointIfGeq { ptr: usize, pos: usize, reg: Reg },
    BranchPointIfNz { ptr: usize, pos: usize, reg: Reg },
    BranchPointIfZr { ptr: usize, pos: usize, reg: Reg },
}
