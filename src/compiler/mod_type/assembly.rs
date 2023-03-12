use crate::{
    compiler::{
        collecting::{CollectedModule, CollectedModuleData, CollectedModuleFunctionExport},
        error::{Error, Result},
        ModuleType,
    },
    parser::{BareModule, Node},
};

pub struct Assembly;

impl ModuleType for Assembly {
    fn collect(&self, BareModule { name, src, root }: BareModule) -> Result<CollectedModule> {
        let mut nodes = root.into_iter();
        nodes.next().unwrap();
        let mut exported_funcs = Vec::<CollectedModuleFunctionExport>::with_capacity(0);
        let mut scopes = Vec::with_capacity(0);
        for node in nodes {
            let Node::Node { span, mut sub_nodes } = node else {
                panic!("Invalid AST");
            };
            if sub_nodes.is_empty() {
                return Err(Error::EmptyNode { span });
            }
            let Node::Ident { span: keyword_span } = &sub_nodes[0] else {
                return Err(Error::UnexpectedToken {
                    span: sub_nodes[0].span(),
                });
            };
            let keyword = &src[keyword_span.clone()];
            match keyword {
                "scope" | "+scope" => {
                    let public = keyword.starts_with('+');
                    if sub_nodes.len() != 3 {
                        return Err(Error::InvalidStatement { span });
                    }
                    let Node::Ident { span: name_span } = &sub_nodes[1] else {
                        return Err(Error::UnexpectedToken {
                            span: sub_nodes[1].span(),
                        });
                    };
                    let name = &src[name_span.clone()];
                    for func in &scopes {
                        match func {
                            AssemblyCollectedScope::Public { export_index, .. } => {
                                if name == exported_funcs[*export_index].name {
                                    return Err(Error::DuplicateName {
                                        span: name_span.clone(),
                                    });
                                }
                            }
                            AssemblyCollectedScope::Private {
                                name: func_name, ..
                            } => {
                                if name == func_name {
                                    return Err(Error::DuplicateName {
                                        span: name_span.clone(),
                                    });
                                }
                            }
                        }
                    }
                    if public {
                        exported_funcs.push(CollectedModuleFunctionExport {
                            name: name.to_string(),
                        });
                        scopes.push(AssemblyCollectedScope::Public {
                            export_index: exported_funcs.len() - 1,
                            expr: sub_nodes.pop().unwrap(),
                        });
                        continue;
                    }
                    scopes.push(AssemblyCollectedScope::Private {
                        name: name.to_string(),
                        expr: sub_nodes.pop().unwrap(),
                    });
                }
                _ => {
                    return Err(Error::InvalidKeyword {
                        span: keyword_span.clone(),
                    })
                }
            }
        }
        Ok(CollectedModule {
            name,
            src,
            exported_funcs,
            data: CollectedModuleData::Assembly(AssemblyCollectedModuleData { scopes }),
        })
    }
}

#[derive(Debug)]
pub struct AssemblyCollectedModuleData {
    pub scopes: Vec<AssemblyCollectedScope>,
}

#[derive(Debug)]
pub enum AssemblyCollectedScope {
    Public { export_index: usize, expr: Node },
    Private { name: String, expr: Node },
}
