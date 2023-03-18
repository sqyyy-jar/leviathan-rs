use std::{collections::HashMap, fmt::Debug, mem};

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
    ) -> Result<()>;

    fn gen_intermediary(&self, task: &mut CompileTask, module_index: usize) -> Result<()>;
}

#[derive(Debug)]
pub struct CollectPacket<'a> {
    pub src: &'a str,
    pub module: UncollectedModule,
}

#[derive(Debug)]
pub struct CompileTask {
    pub module_indices: HashMap<String, usize>,
    pub modules: Vec<Module>,
    pub status: Status,
}

impl Default for CompileTask {
    fn default() -> Self {
        Self {
            module_indices: HashMap::with_capacity(0),
            modules: Vec::with_capacity(0),
            status: Status::Open,
        }
    }
}

impl CompileTask {
    pub fn include(&mut self, BareModule { name, src, root }: BareModule) -> Result<()> {
        if self.status != Status::Open {
            return Err(Error::InvalidOperation);
        }
        if self.module_indices.contains_key(&name) {
            self.status = Status::Failed;
            return Err(Error::DuplicateModule { name: Some(name) });
        }
        if root.is_empty() {
            self.status = Status::Failed;
            return Err(Error::EmptyModule { name: Some(name) });
        }
        let Node::Node {
            span: mod_decl_span,
            sub_nodes: mod_sub_nodes,
        } = &root[0] else
        {
            panic!("Invalid AST");
        };
        if mod_sub_nodes.len() != 2 {
            self.status = Status::Failed;
            return Err(Error::InvalidModuleDeclaration {
                src: Some(src),
                span: mod_decl_span.clone(),
            });
        }
        let keyword_node = &mod_sub_nodes[0];
        let ident_node = &mod_sub_nodes[1];
        let Node::Ident { span: keyword_span } = keyword_node else {
            self.status = Status::Failed;
            return Err(Error::InvalidModuleDeclaration {
                src: Some(src),
                span: keyword_node.span(),
            });
        };
        let keyword = &src[keyword_span.clone()];
        if keyword != "mod" {
            self.status = Status::Failed;
            return Err(Error::InvalidModuleDeclaration {
                src: Some(src),
                span: keyword_span.clone(),
            });
        }
        let Node::Ident { span: ident_span } = ident_node else {
            self.status = Status::Failed;
            return Err(Error::InvalidModuleDeclaration {
                src: Some(src),
                span: ident_node.span(),
            });
        };
        let ident = &src[ident_span.clone()];
        let Some(mod_type) = MODULE_TYPES.get(ident) else {
            self.status = Status::Failed;
            return Err(Error::UnknownModuleType { src: Some(src), span: ident_span.clone() });
        };
        let module_index = self.modules.len();
        self.modules.push(Module::new(src, *mod_type));
        self.module_indices.insert(name, self.modules.len() - 1);
        let module = mod_type.collect(self, module_index, UncollectedModule { root });
        if let Err(err) = module {
            self.status = Status::Failed;
            return Err(err.complete(self.modules.pop().unwrap().src));
        }
        Ok(())
    }

    pub fn gen_intermediary(&mut self) -> Result<()> {
        if self.status != Status::Open {
            return Err(Error::InvalidOperation);
        }
        self.status = Status::Collected;
        for i in 0..self.modules.len() {
            let m = &mut self.modules[i];
            let type_ = m.type_;
            let res = type_.gen_intermediary(self, i);
            if let Err(err) = res {
                self.status = Status::Failed;
                return Err(err.complete(mem::take(&mut self.modules[i].src)));
            }
        }
        Ok(())
    }
}

pub struct Module {
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
    pub fn new(src: String, type_: &'static dyn ModuleType) -> Self {
        Self {
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
    Failed,
}

#[derive(Debug)]
pub struct UncollectedModule {
    pub root: Vec<Node>,
}
