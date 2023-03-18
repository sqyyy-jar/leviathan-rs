use std::{
    collections::HashMap,
    fmt::Debug,
    io::{Seek, Write},
};

use phf::{phf_map, Map};

use crate::parser::{BareModule, Node};

use self::{
    error::{Error, Result},
    intermediary::{Insn, IntermediaryStaticValue},
    mod_type::assembly::Assembly,
};

pub mod error;
pub mod intermediary;
pub mod mod_type;

pub const MODULE_TYPES: Map<&'static str, &dyn ModuleType> = phf_map! {
    "assembly" => &Assembly,
};

pub trait ModuleType {
    fn collect(
        &self,
        task: &mut CompileTask,
        module_index: usize,
        module: UncollectedModule,
        main: bool,
    ) -> Result<()>;

    fn gen_intermediary(&self, task: &mut CompileTask, module_index: usize) -> Result<()>;
}

#[derive(Debug)]
pub struct CompileTask {
    pub module_indices: HashMap<String, usize>,
    pub modules: Vec<Module>,
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
        let Some(mod_type) = MODULE_TYPES.get(ident) else {
            return Err(Error::UnknownModuleType { file, src, span: ident_span.clone() });
        };
        let module_index = self.modules.len();
        self.modules.push(Module::new(file, src, *mod_type));
        self.module_indices.insert(name, self.modules.len() - 1);
        mod_type.collect(self, module_index, UncollectedModule { root }, main)?;
        Ok(())
    }

    pub fn compile(&mut self) -> Result<()> {
        if self.status != Status::Open {
            return Err(Error::InvalidOperation);
        }
        self.status = Status::Invalid;
        for i in 0..self.modules.len() {
            let m = &mut self.modules[i];
            let type_ = m.type_;
            type_.gen_intermediary(self, i)?;
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
        // todo!();
        self.status = Status::Filtered;
        Ok(())
    }

    pub fn assemble(&mut self, _out: &mut (impl Write + Seek)) -> Result<()> {
        if self.status != Status::Compiled && self.status != Status::Filtered {
            return Err(Error::InvalidOperation);
        }
        self.status = Status::Invalid;
        assert!(self.main.is_some());
        let _main = self.main.unwrap();
        // todo!();
        self.status = Status::Complete;
        Ok(())
    }
}

pub struct Module {
    pub file: String,
    pub src: String,
    pub func_indices: HashMap<String, usize>,
    pub funcs: Vec<Func>,
    pub static_indices: HashMap<String, usize>,
    pub statics: Vec<Static>,
    pub type_: &'static dyn ModuleType,
    pub used: bool,
}

impl Debug for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Module")
            .field("src", &self.src)
            .field("func_indices", &self.func_indices)
            .field("funcs", &self.funcs)
            .field("static_indices", &self.static_indices)
            .field("statics", &self.statics)
            .field("used", &self.used)
            .finish()
    }
}

impl Module {
    pub fn new(file: String, src: String, type_: &'static dyn ModuleType) -> Self {
        Self {
            file,
            src,
            func_indices: HashMap::with_capacity(0),
            funcs: Vec::with_capacity(0),
            static_indices: HashMap::with_capacity(0),
            statics: Vec::with_capacity(0),
            type_,
            used: false,
        }
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
    Collected { nodes: Vec<Node> },
    Intermediary { ir: Vec<Insn> },
}

#[derive(Debug)]
pub enum StaticData {
    Collected { nodes: Vec<Node> },
    Intermediary { value: IntermediaryStaticValue },
}

#[derive(Debug)]
pub enum Type {
    None,
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
