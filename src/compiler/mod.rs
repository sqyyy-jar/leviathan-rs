use std::collections::HashMap;

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
    fn collect(&self, src: &str, module: UncollectedModule) -> Result<CollectedModule>;

    fn gen_intermediary(&self, src: &str, module: CollectedModule) -> Result<IntermediaryModule>;
}

#[derive(Debug)]
pub struct CompileTask {
    pub state: State,
}

impl Default for CompileTask {
    fn default() -> Self {
        Self {
            state: State::LayoutCollecting {
                module_lookup: HashMap::with_capacity(0),
                modules: Vec::with_capacity(0),
            },
        }
    }
}

impl CompileTask {
    pub fn include(&mut self, BareModule { name, src, root }: BareModule) -> Result<()> {
        let State::LayoutCollecting { module_lookup, modules } = &mut self.state else {
            return Err(Error::InvalidOperation);
        };
        if module_lookup.contains_key(&name) {
            return Err(Error::DuplicateModule { name: Some(name) });
        }
        if root.is_empty() {
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
            return Err(Error::InvalidModuleDeclaration {
                src: Some(src),
                span: mod_decl_span.clone(),
            });
        }
        let keyword_node = &mod_sub_nodes[0];
        let ident_node = &mod_sub_nodes[1];
        let Node::Ident { span: keyword_span } = keyword_node else {
            return Err(Error::InvalidModuleDeclaration {
                src: Some(src),
                span: keyword_node.span(),
            });
        };
        let keyword = &src[keyword_span.clone()];
        if keyword != "mod" {
            return Err(Error::InvalidModuleDeclaration {
                src: Some(src),
                span: keyword_span.clone(),
            });
        }
        let Node::Ident { span: ident_span } = ident_node else {
            return Err(Error::InvalidModuleDeclaration {
                src: Some(src),
                span: ident_node.span(),
            });
        };
        let ident = &src[ident_span.clone()];
        let Some(mod_type) = MODULE_TYPES.get(ident) else {
            return Err(Error::UnknownModuleType { src: Some(src), span: ident_span.clone() });
        };
        let module = mod_type.collect(&src, UncollectedModule { root });
        if let Err(err) = module {
            return Err(err.complete(src));
        }
        let module = module.unwrap();
        modules.push(ModuleState { src, module });
        module_lookup.insert(name, modules.len() - 1);
        Ok(())
    }

    pub fn gen_intermediary(&mut self) -> Result<()> {
        let State::LayoutCollecting { .. } = &mut self.state else {
            return Err(Error::InvalidOperation);
        };
        let State::LayoutCollecting { module_lookup, modules } = std::mem::replace(
            &mut self.state,
            State::Intermediary {
                module_lookup: HashMap::with_capacity(0),
                modules: Vec::with_capacity(0),
            },
        ) else {
            unreachable!();
        };
        let mut new_modules = Vec::with_capacity(modules.len());
        for ModuleState { src, module } in modules {
            let module = match &module.data {
                CollectedModuleData::Assembly { .. } => Assembly.gen_intermediary(&src, module),
                _ => unimplemented!(),
            };
            if let Err(err) = module {
                return Err(err.complete(src));
            }
            new_modules.push(ModuleState {
                src,
                module: module.unwrap(),
            });
        }
        self.state = State::Intermediary {
            module_lookup,
            modules: new_modules,
        };
        Ok(())
    }
}

#[derive(Debug)]
pub enum State {
    LayoutCollecting {
        module_lookup: HashMap<String, usize>,
        modules: Vec<ModuleState<CollectedModule>>,
    },
    Intermediary {
        module_lookup: HashMap<String, usize>,
        modules: Vec<ModuleState<IntermediaryModule>>,
    },
    DependencyFiltered {},
    BinaryAssembled {},
}

#[derive(Debug)]
pub struct ModuleState<T: Sized> {
    pub src: String,
    pub module: T,
}

#[derive(Debug)]
pub struct UncollectedModule {
    pub root: Vec<Node>,
}
