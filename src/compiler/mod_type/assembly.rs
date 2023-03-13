use crate::{
    compiler::{
        collecting::{CollectedFunction, CollectedModule, CollectedModuleData},
        error::{Error, Result},
        intermediary::{Insn, IntermediaryFunction, IntermediaryModule, Reg},
        ModuleType,
    },
    parser::{BareModule, Node},
};

pub struct Assembly;

impl ModuleType for Assembly {
    fn collect(&self, BareModule { src, root, .. }: BareModule) -> Result<CollectedModule> {
        let mut nodes = root.into_iter();
        nodes.next().unwrap();
        let mut funcs = Vec::<CollectedFunction>::with_capacity(0);
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
                        let AssemblyCollectedScope {
                            func_index: export_index,
                            ..
                        } = func;
                        if name == funcs[*export_index].name {
                            return Err(Error::DuplicateName {
                                span: name_span.clone(),
                            });
                        }
                    }
                    funcs.push(CollectedFunction {
                        name: name.to_string(),
                        public,
                    });
                    scopes.push(AssemblyCollectedScope {
                        func_index: funcs.len() - 1,
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
            funcs,
            data: CollectedModuleData::Assembly { scopes },
        })
    }

    fn gen_intermediary(
        &self,
        CollectedModule { src, funcs, data }: CollectedModule,
    ) -> Result<IntermediaryModule> {
        let CollectedModuleData::Assembly { scopes } = data else {
            panic!("Invalid module data");
        };
        let mut ir_funcs = Vec::with_capacity(scopes.len());
        for scope in scopes {
            ir_funcs.push(gen_scope_intermediary(&src, scope)?);
        }
        Ok(IntermediaryModule {
            src,
            funcs,
            ir_funcs,
        })
    }
}

#[derive(Debug)]
pub struct AssemblyCollectedScope {
    pub func_index: usize,
    pub expr: Node,
}

fn gen_scope_intermediary(
    _src: &str,
    AssemblyCollectedScope { func_index, expr }: AssemblyCollectedScope,
) -> Result<IntermediaryFunction> {
    let mut ir = Vec::with_capacity(2);
    match expr {
        Node::Ident { .. } => todo!(),
        Node::Int { .. } | Node::UInt { .. } | Node::Float { .. } => {
            match expr {
                Node::Int { value, .. } => ir.push(Insn::LdcInt {
                    dst: Reg::R0,
                    value,
                }),
                Node::UInt { value, .. } => ir.push(Insn::LdcUInt {
                    dst: Reg::R0,
                    value,
                }),
                Node::Float { value, .. } => ir.push(Insn::LdcFloat {
                    dst: Reg::R0,
                    value,
                }),
                _ => unreachable!(),
            }
            ir.push(Insn::Ret);
            return Ok(IntermediaryFunction {
                func_index,
                ir,
                deps: Vec::with_capacity(0),
            });
        }
        Node::String { .. } => todo!(),
        Node::Node { .. } => todo!(),
    }
    todo!()
}
