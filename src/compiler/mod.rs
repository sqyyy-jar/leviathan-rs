use phf::{phf_map, Map};

use crate::parser::{BareModule, Node};

use self::{
    collecting::{CollectedModule, CollectedModuleData},
    error::{Error, Result},
    intermediary::IntermediaryModule,
    mod_type::assembly::Assembly,
};

pub mod collecting;
pub mod error;
pub mod intermediary;
pub mod mod_type;

pub const MODULE_TYPES: Map<&'static str, &dyn ModuleType> = phf_map! {
    "assembly" => &Assembly,
};

pub trait ModuleType {
    fn collect(&self, module: BareModule) -> Result<CollectedModule>;

    fn gen_intermediary(&self, module: CollectedModule) -> Result<IntermediaryModule>;
}

#[derive(Debug)]
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
    pub fn include(&mut self, module: BareModule) -> Result<()> {
        let State::LayoutCollecting { modules } = &mut self.state else {
            return Err(Error::InvalidOperation);
        };
        if module.root.is_empty() {
            return Err(Error::EmptyModule { name: module.name });
        }
        let Node::Node { span: _, sub_nodes: mod_sub_nodes } = &module.root[0] else {
            panic!("Invalid AST");
        };
        if mod_sub_nodes.len() != 2 {
            return Err(Error::EmptyModule { name: module.name });
        }
        let keyword_node = &mod_sub_nodes[0];
        let ident_node = &mod_sub_nodes[1];
        let Node::Ident { span: keyword_span } = keyword_node else {
            return Err(Error::InvalidModuleDeclaration { span: keyword_node.span() });
        };
        let keyword = &module.src[keyword_span.clone()];
        if keyword != "mod" {
            return Err(Error::InvalidModuleDeclaration {
                span: keyword_span.clone(),
            });
        }
        let Node::Ident { span: ident_span } = ident_node else {
            return Err(Error::InvalidModuleDeclaration { span: ident_node.span() });
        };
        let ident = &module.src[ident_span.clone()];
        let Some(mod_type) = MODULE_TYPES.get(ident) else {
            return Err(Error::UnknownModuleType { span: ident_span.clone() });
        };
        modules.push(mod_type.collect(module)?);
        Ok(())
    }

    pub fn gen_intermediary(&mut self) -> Result<()> {
        let State::LayoutCollecting { .. } = &mut self.state else {
            return Err(Error::InvalidOperation);
        };
        let State::LayoutCollecting { modules } = std::mem::replace(
            &mut self.state,
            State::Intermediary {
                modules: Vec::with_capacity(0),
            },
        ) else {
            unreachable!();
        };
        let mut new_modules = Vec::with_capacity(modules.len());
        for module in modules {
            new_modules.push(match &module.data {
                CollectedModuleData::Assembly(_) => Assembly.gen_intermediary(module)?,
            });
        }
        self.state = State::Intermediary {
            modules: new_modules,
        };
        Ok(())
    }
}

#[derive(Debug)]
pub enum State {
    LayoutCollecting { modules: Vec<CollectedModule> },
    Intermediary { modules: Vec<IntermediaryModule> },
    DependencyFiltered {},
    BinaryAssembled {},
}
