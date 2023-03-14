use phf::{phf_map, Map};

use crate::{
    compiler::{
        collecting::{CollectedFunction, CollectedModule, CollectedModuleData},
        error::{Error, Result},
        intermediary::{
            Insn, IntermediaryFunction, IntermediaryModule, IntermediaryStatic,
            IntermediaryStaticValue, Reg,
        },
        ModuleType,
    },
    parser::{BareModule, Node},
    util::source::Span,
};

type StaticFunc = fn(name: String, span: Span, nodes: Vec<Node>) -> Result<IntermediaryStatic>;

const STATIC_FUNCS: Map<&'static str, StaticFunc> = phf_map! {
    "buffer" => static_buffer,
};

fn static_buffer(name: String, span: Span, mut nodes: Vec<Node>) -> Result<IntermediaryStatic> {
    if nodes.len() != 2 {
        return Err(Error::InvalidCallSignature { span });
    }
    let size = match nodes.pop().unwrap() {
        Node::Int { span, value } => {
            if value < 1 {
                return Err(Error::NotInSizeRangeFrom { span, range: 1.. });
            }
            value as usize
        }
        Node::UInt { span, value } => {
            if value < 1 {
                return Err(Error::NotInSizeRangeFrom { span, range: 1.. });
            }
            value as usize
        }
        _ => return Err(Error::InvalidCallSignature { span }),
    };
    Ok(IntermediaryStatic {
        name,
        value: IntermediaryStaticValue::Buffer { size },
    })
}

pub struct Assembly;

impl ModuleType for Assembly {
    fn collect(&self, BareModule { src, root, .. }: BareModule) -> Result<CollectedModule> {
        let mut nodes = root.into_iter();
        nodes.next().unwrap();
        let mut statics = Vec::with_capacity(0);
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
                "static" => {
                    if sub_nodes.len() != 3 {
                        return Err(Error::InvalidStatement { span });
                    }
                    let Node::Ident { span: name_span } = &sub_nodes[1] else {
                        return Err(Error::UnexpectedToken {
                            span: sub_nodes[1].span(),
                        });
                    };
                    let name = &src[name_span.clone()];
                    for static_ in &statics {
                        let AssemblyCollectedStatic {
                            name: static_name, ..
                        } = static_;
                        if name == static_name {
                            return Err(Error::DuplicateName {
                                span: name_span.clone(),
                            });
                        }
                    }
                    statics.push(AssemblyCollectedStatic {
                        name: name.to_string(),
                        expr: sub_nodes.pop().unwrap(),
                    });
                }
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
            data: CollectedModuleData::Assembly { statics, scopes },
        })
    }

    fn gen_intermediary(
        &self,
        CollectedModule { src, funcs, data }: CollectedModule,
    ) -> Result<IntermediaryModule> {
        let CollectedModuleData::Assembly { statics, scopes } = data else {
            panic!("Invalid module data");
        };
        let mut ir_statics = Vec::with_capacity(statics.len());
        for static_ in statics {
            ir_statics.push(gen_static_intermediary(&src, static_)?);
        }
        let mut ir_funcs = Vec::with_capacity(scopes.len());
        for scope in scopes {
            ir_funcs.push(gen_scope_intermediary(&src, scope)?);
        }
        Ok(IntermediaryModule {
            src,
            funcs,
            statics: ir_statics,
            ir_funcs,
        })
    }
}

#[derive(Debug)]
pub struct AssemblyCollectedStatic {
    pub name: String,
    pub expr: Node,
}

#[derive(Debug)]
pub struct AssemblyCollectedScope {
    pub func_index: usize,
    pub expr: Node,
}

fn gen_static_intermediary(
    src: &str,
    AssemblyCollectedStatic { name, expr }: AssemblyCollectedStatic,
) -> Result<IntermediaryStatic> {
    match expr {
        Node::Ident { span } => Err(Error::UnexpectedToken { span }),
        Node::Int { value, .. } => Ok(IntermediaryStatic {
            name,
            value: IntermediaryStaticValue::Int(value),
        }),
        Node::UInt { value, .. } => Ok(IntermediaryStatic {
            name,
            value: IntermediaryStaticValue::UInt(value),
        }),
        Node::Float { value, .. } => Ok(IntermediaryStatic {
            name,
            value: IntermediaryStaticValue::Float(value),
        }),
        Node::String { value, .. } => Ok(IntermediaryStatic {
            name,
            value: IntermediaryStaticValue::String(value),
        }),
        Node::Node { span, sub_nodes } => {
            if sub_nodes.is_empty() {
                return Err(Error::EmptyNode { span });
            }
            let Node::Ident { span: spa } = &sub_nodes[0] else {
                return Err(Error::UnexpectedToken { span });
            };
            let keyword = &src[spa.clone()];
            let Some(static_func) = STATIC_FUNCS.get(keyword) else {
                return Err(Error::UnknownStaticFunc { span: spa.clone() });
            };
            (*static_func)(name, span, sub_nodes)
        }
    }
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
            Ok(IntermediaryFunction {
                func_index,
                ir,
                deps: Vec::with_capacity(0),
            })
        }
        Node::String { .. } => todo!(),
        Node::Node { .. } => todo!(),
    }
}
