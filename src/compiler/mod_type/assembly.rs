use phf::{phf_map, Map};

use crate::{
    compiler::{
        collecting::{CollectedFunction, CollectedModule, CollectedModuleData},
        error::{Error, Result},
        intermediary::{
            Insn, IntermediaryFunction, IntermediaryModule, IntermediaryStatic,
            IntermediaryStaticValue, Reg,
        },
        ModuleType, UncollectedModule,
    },
    parser::Node,
    util::source::Span,
};

type StaticFunc<'a> = fn(name: String, span: Span, nodes: Vec<Node>) -> Result<IntermediaryStatic>;

const STATIC_FUNCS: Map<&'static str, StaticFunc> = phf_map! {
    "buffer" => static_buffer,
};

fn static_buffer(name: String, span: Span, mut nodes: Vec<Node>) -> Result<IntermediaryStatic> {
    if nodes.len() != 2 {
        return Err(Error::InvalidCallSignature { src: None, span });
    }
    let size = match nodes.pop().unwrap() {
        Node::Int { span, value } => {
            if value < 1 {
                return Err(Error::NotInSizeRangeFrom {
                    src: None,
                    span,
                    range: 1..,
                });
            }
            value as usize
        }
        Node::UInt { span, value } => {
            if value < 1 {
                return Err(Error::NotInSizeRangeFrom {
                    src: None,
                    span,
                    range: 1..,
                });
            }
            value as usize
        }
        _ => return Err(Error::InvalidCallSignature { src: None, span }),
    };
    Ok(IntermediaryStatic {
        name: Some(name),
        value: IntermediaryStaticValue::Buffer { size },
    })
}

pub struct Assembly;

impl ModuleType for Assembly {
    fn collect(
        &self,
        src: &str,
        UncollectedModule { root }: UncollectedModule,
    ) -> Result<CollectedModule> {
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
                return Err(Error::EmptyNode { src: None, span });
            }
            let Node::Ident { span: keyword_span } = &sub_nodes[0] else {
                return Err(Error::UnexpectedToken {
                    src: None,
                    span: sub_nodes[0].span(),
                });
            };
            let keyword = &src[keyword_span.clone()];
            match keyword {
                "static" => {
                    if sub_nodes.len() != 3 {
                        return Err(Error::InvalidStatement { src: None, span });
                    }
                    let Node::Ident { span: name_span } = &sub_nodes[1] else {
                        return Err(Error::UnexpectedToken {
                            src: None,
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
                                src: None,
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
                        return Err(Error::InvalidStatement { src: None, span });
                    }
                    let Node::Ident { span: name_span } = &sub_nodes[1] else {
                        return Err(Error::UnexpectedToken {
                            src: None,
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
                                src: None,
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
                        src: None,
                        span: keyword_span.clone(),
                    })
                }
            }
        }
        Ok(CollectedModule {
            funcs,
            data: CollectedModuleData::Assembly { statics, scopes },
        })
    }

    fn gen_intermediary(
        &self,
        src: &str,
        CollectedModule { funcs, data }: CollectedModule,
    ) -> Result<IntermediaryModule> {
        let CollectedModuleData::Assembly { statics, scopes } = data else {
            panic!("Invalid module data");
        };
        let mut ir_statics = Vec::with_capacity(statics.len());
        for static_ in statics {
            ir_statics.push(gen_static_intermediary(src, static_)?);
        }
        let mut ir_funcs = Vec::with_capacity(scopes.len());
        for scope in scopes {
            ir_funcs.push(gen_scope_intermediary(src, &mut ir_statics, scope)?);
        }
        Ok(IntermediaryModule {
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
        Node::Ident { span } => Err(Error::UnexpectedToken { src: None, span }),
        Node::Int { value, .. } => Ok(IntermediaryStatic {
            name: Some(name),
            value: IntermediaryStaticValue::Int(value),
        }),
        Node::UInt { value, .. } => Ok(IntermediaryStatic {
            name: Some(name),
            value: IntermediaryStaticValue::UInt(value),
        }),
        Node::Float { value, .. } => Ok(IntermediaryStatic {
            name: Some(name),
            value: IntermediaryStaticValue::Float(value),
        }),
        Node::String { value, .. } => Ok(IntermediaryStatic {
            name: Some(name),
            value: IntermediaryStaticValue::String(value),
        }),
        Node::Node { span, sub_nodes } => {
            if sub_nodes.is_empty() {
                return Err(Error::EmptyNode { src: None, span });
            }
            let Node::Ident { span: spa } = &sub_nodes[0] else {
                return Err(Error::UnexpectedToken { src: None, span });
            };
            let keyword = &src[spa.clone()];
            let Some(static_func) = STATIC_FUNCS.get(keyword) else {
                return Err(Error::UnknownStaticFunc { src: None, span: spa.clone() });
            };
            (*static_func)(name, span, sub_nodes)
        }
    }
}

fn gen_scope_intermediary(
    src: &str,
    ir_statics: &mut Vec<IntermediaryStatic>,
    AssemblyCollectedScope { func_index, expr }: AssemblyCollectedScope,
) -> Result<IntermediaryFunction> {
    let mut ir = Vec::with_capacity(2);
    match expr {
        Node::Ident { span } => {
            let name = &src[span.clone()];
            for (index, ir_static) in ir_statics.iter().enumerate() {
                let Some(static_name) = &ir_static.name else {
                    continue;
                };
                if name != static_name {
                    continue;
                }
                match &ir_static.value {
                    IntermediaryStaticValue::Int(_)
                    | IntermediaryStaticValue::UInt(_)
                    | IntermediaryStaticValue::Float(_) => {
                        ir.push(Insn::LdStaticAbsAddr {
                            dst: Reg::R0,
                            index,
                        });
                    }
                    IntermediaryStaticValue::String(value) => {
                        ir.push(Insn::LdStaticAbsAddr {
                            dst: Reg::R0,
                            index,
                        });
                        ir.push(Insn::LdcUInt {
                            dst: Reg::R1,
                            value: value.len() as u64,
                        });
                    }
                    IntermediaryStaticValue::Buffer { size } => {
                        ir.push(Insn::LdStaticAbsAddr {
                            dst: Reg::R0,
                            index,
                        });
                        ir.push(Insn::LdcUInt {
                            dst: Reg::R1,
                            value: *size as u64,
                        });
                    }
                }
                ir.push(Insn::Ret);
                return Ok(IntermediaryFunction {
                    func_index,
                    ir,
                    deps: Vec::with_capacity(0),
                });
            }
            Err(Error::UnknownStaticVariable { src: None, span })
        }
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
        Node::String { value, .. } => {
            let len = value.len();
            ir_statics.push(IntermediaryStatic {
                name: None,
                value: IntermediaryStaticValue::String(value),
            });
            ir.push(Insn::LdStaticAbsAddr {
                dst: Reg::R0,
                index: ir_statics.len() - 1,
            });
            ir.push(Insn::LdcUInt {
                dst: Reg::R1,
                value: len as u64,
            });
            ir.push(Insn::Ret);
            Ok(IntermediaryFunction {
                func_index,
                ir,
                deps: Vec::with_capacity(0),
            })
        }
        Node::Node { span, sub_nodes } => {
            if sub_nodes.is_empty() {
                return Err(Error::EmptyNode { src: None, span });
            }
            todo!("Add function call/macro expansion");
            // Ok(IntermediaryFunction {
            //     func_index,
            //     ir,
            //     deps: Vec::with_capacity(0),
            // })
        }
    }
}
