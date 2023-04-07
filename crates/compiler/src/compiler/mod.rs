use std::{
    collections::HashMap,
    fmt::Debug,
    io::{Seek, Write},
    mem,
};

use leviathan_ir::{
    binary::{Binary, BinaryModule},
    layers::Coord,
};
use phf::{phf_map, Map};

use crate::parser::{BareModule, BracketType, Node};

use self::{
    dialect::{assembly::AssemblyLanguage, code::CodeLanguage},
    error::{Error, Result},
    intermediary::{Insn, Reg},
};

pub mod dialect;
pub mod error;
pub mod intermediary;

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

    fn compile_module(
        &mut self,
        task: &mut CompileTask,
        module_index: usize,
    ) -> Result<BinaryModule>;

    fn lookup_callable(&self, name: &str) -> Option<usize>;
}

#[derive(Debug)]
pub struct CompileTask {
    pub module_indices: HashMap<String, usize>,
    pub modules: Vec<Module>,
    pub status: Status,
    pub main: Option<Coord>,
    pub binary: Binary,
}

impl Default for CompileTask {
    fn default() -> Self {
        Self {
            module_indices: HashMap::with_capacity(0),
            modules: Vec::with_capacity(0),
            status: Status::Open,
            main: None,
            binary: Binary::default(),
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
        self.modules.push(Module::new(file, src, dialect));
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
            let binary_module = dialect.compile_module(self, i)?;
            self.binary.modules.insert(i, binary_module);
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
        self.status = Status::Filtered;
        Ok(())
    }

    pub fn assemble(&mut self, out: &mut (impl Write + Seek)) -> Result<()> {
        if self.status != Status::Compiled && self.status != Status::Filtered {
            return Err(Error::InvalidOperation);
        }
        self.status = Status::Invalid;
        assert!(self.main.is_some());
        self.binary.assemble(out, self.main.unwrap())?;
        Ok(())
    }
}

pub struct Module {
    pub file: String,
    pub src: String,
    pub dialect: Option<Box<dyn Dialect>>,
}

impl Debug for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Module")
            .field("file", &self.file)
            .field("src", &self.src)
            .finish()
    }
}

impl Module {
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

#[derive(Debug, Default)]
pub struct StaticData {
    pub node: Node,
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
