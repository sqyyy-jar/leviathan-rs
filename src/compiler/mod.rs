use phf::{phf_map, Map};

use crate::parser::Node;

use self::{
    collecting::CollectedModule,
    error::{Error, Result},
    mod_type::assembly::Assembly,
};

pub mod collecting;
pub mod error;
pub mod mod_type;

pub const MODULE_TYPES: Map<&'static str, &dyn ModuleType> = phf_map! {
    "assembly" => &Assembly,
};

pub trait ModuleType {
    fn collect(&self, name: String, root: Vec<Node>) -> Result<CollectedModule>;
}

pub struct CompileTask {
    pub state: State,
}

impl Default for CompileTask {
    fn default() -> Self {
        Self {
            state: State::LayoutCollecting {
                modules: Vec::with_capacity(0),
            },
        }
    }
}

impl CompileTask {
    pub fn include(
        &mut self,
        source: &str,
        BareModule { name, mut root }: BareModule,
    ) -> Result<()> {
        let State::LayoutCollecting { modules } = &mut self.state else {
            return Err(Error::InvalidOperation);
        };
        if root.is_empty() {
            return Err(Error::EmptyModule { name });
        }
        let Node::Node { span: _, sub_nodes: mod_sub_nodes } = &mut root[0] else {
            panic!("Invalid AST");
        };
        if mod_sub_nodes.len() != 2 {
            return Err(Error::EmptyModule { name });
        }
        let ident_node = mod_sub_nodes.pop().unwrap();
        let keyword_node = mod_sub_nodes.pop().unwrap();
        let Node::Ident { span: keyword_span } = keyword_node else {
            return Err(Error::InvalidModuleDeclaration { span: keyword_node.span() });
        };
        let keyword = &source[keyword_span.clone()];
        if keyword != "mod" {
            return Err(Error::InvalidModuleDeclaration { span: keyword_span });
        }
        let Node::Ident { span: ident_span } = ident_node else {
            return Err(Error::InvalidModuleDeclaration { span: ident_node.span() });
        };
        let ident = &source[ident_span.clone()];
        let Some(mod_type) = MODULE_TYPES.get(ident) else {
            return Err(Error::UnknownModuleType { span: ident_span });
        };
        modules.push(mod_type.collect(name, root)?);
        Ok(())
    }
}

pub enum State {
    LayoutCollecting { modules: Vec<CollectedModule> },
    IntermediaryGeneration {},
    DependencyFiltering {},
    BinaryAssembling {},
}

pub struct BareModule {
    pub name: String,
    pub root: Vec<Node>,
}
