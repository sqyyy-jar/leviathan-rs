use crate::{
    compiler::{
        collecting::{CollectedModule, CollectedModuleData, CollectedModuleFunction},
        error::{Error, Result},
        intermediary::IntermediaryModule,
        ModuleType,
    },
    parser::{BareModule, Node},
};

pub struct Assembly;

impl ModuleType for Assembly {
    fn collect(&self, BareModule { src, root, .. }: BareModule) -> Result<CollectedModule> {
        let mut nodes = root.into_iter();
        nodes.next().unwrap();
        let mut exported_funcs = Vec::<CollectedModuleFunction>::with_capacity(0);
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
                        let AssemblyCollectedScope { export_index, .. } = func;
                        if name == exported_funcs[*export_index].name {
                            return Err(Error::DuplicateName {
                                span: name_span.clone(),
                            });
                        }
                    }
                    exported_funcs.push(CollectedModuleFunction {
                        name: name.to_string(),
                        public,
                    });
                    scopes.push(AssemblyCollectedScope {
                        export_index: exported_funcs.len() - 1,
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
            src,
            exported_funcs,
            data: CollectedModuleData::Assembly(AssemblyCollectedModuleData { scopes }),
        })
    }

    fn gen_intermediary(&self, _module: CollectedModule) -> Result<IntermediaryModule> {
        todo!()
    }
}

#[derive(Debug)]
pub struct AssemblyCollectedModuleData {
    pub scopes: Vec<AssemblyCollectedScope>,
}

#[derive(Debug)]
pub struct AssemblyCollectedScope {
    pub export_index: usize,
    pub expr: Node,
}
