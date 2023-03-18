pub mod insns;
pub mod macros;
pub mod static_funcs;

use crate::{
    compiler::{
        error::{Error, Result},
        intermediary::{Insn, IntermediaryStaticValue, Reg},
        mod_type::assembly::{insns::INSN_MACROS, macros::MACROS},
        CompileTask, Func, FuncData, ModuleType, Static, StaticData, Type, UncollectedModule,
    },
    parser::Node,
    util::source::Span,
};

use self::static_funcs::STATIC_FUNCS;

pub struct Assembly;

impl ModuleType for Assembly {
    fn collect(
        &self,
        task: &mut CompileTask,
        module_index: usize,
        UncollectedModule { root }: UncollectedModule,
    ) -> Result<()> {
        let module = &mut task.modules[module_index];
        let mut nodes = root.into_iter();
        nodes.next().unwrap();
        for node in nodes {
            let Node::Node { span, sub_nodes } = node else {
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
            let keyword = &module.src[keyword_span.clone()];
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
                    let name = &module.src[name_span.clone()];
                    if module.static_indices.contains_key(name) {
                        return Err(Error::DuplicateName {
                            src: None,
                            span: name_span.clone(),
                        });
                    }
                    module.statics.push(Static {
                        data: StaticData::Collected { nodes: sub_nodes },
                        used: false,
                    });
                    module
                        .static_indices
                        .insert(name.to_string(), module.statics.len() - 1);
                }
                "-label" | "+label" => {
                    let public = !keyword.starts_with('+');
                    if sub_nodes.len() != 3 {
                        return Err(Error::InvalidStatement { src: None, span });
                    }
                    let Node::Ident { span: name_span } = &sub_nodes[1] else {
                        return Err(Error::UnexpectedToken {
                            src: None,
                            span: sub_nodes[1].span(),
                        });
                    };
                    let name = &module.src[name_span.clone()];
                    if module.func_indices.contains_key(name) {
                        return Err(Error::DuplicateName {
                            src: None,
                            span: name_span.clone(),
                        });
                    }
                    module.funcs.push(Func {
                        public,
                        params: vec![(None, Type::Unknown)],
                        return_: Type::Unknown,
                        data: FuncData::Collected { nodes: sub_nodes },
                        used: false,
                    });
                    module
                        .func_indices
                        .insert(name.to_string(), module.funcs.len() - 1);
                }
                _ => {
                    return Err(Error::InvalidKeyword {
                        src: None,
                        span: keyword_span.clone(),
                    })
                }
            }
        }
        Ok(())
    }

    fn gen_intermediary(&self, task: &mut CompileTask, module_index: usize) -> Result<()> {
        let statics_len = task.modules[module_index].statics.len();
        let funcs_len = task.modules[module_index].funcs.len();
        for static_index in 0..statics_len {
            gen_static_intermediary(task, module_index, static_index)?;
        }
        for func_index in 0..funcs_len {
            gen_scope_intermediary(task, module_index, func_index)?;
        }
        Ok(())
    }
}

fn gen_static_intermediary(
    task: &mut CompileTask,
    module_index: usize,
    static_index: usize,
) -> Result<()> {
    let module = &mut task.modules[module_index];
    let Static { data, used: _ } = &mut module.statics[static_index];
    let StaticData::Collected { nodes } = data else {unreachable!()};
    match nodes.pop().unwrap() {
        Node::Ident { span } => Err(Error::UnexpectedToken { src: None, span }),
        Node::Int { value, .. } => {
            *data = StaticData::Intermediary {
                value: IntermediaryStaticValue::Int(value),
            };
            Ok(())
        }
        Node::UInt { value, .. } => {
            *data = StaticData::Intermediary {
                value: IntermediaryStaticValue::UInt(value),
            };
            Ok(())
        }
        Node::Float { value, .. } => {
            *data = StaticData::Intermediary {
                value: IntermediaryStaticValue::Float(value),
            };
            Ok(())
        }
        Node::String { value, .. } => {
            *data = StaticData::Intermediary {
                value: IntermediaryStaticValue::String(value),
            };
            Ok(())
        }
        Node::Node { span, sub_nodes } => {
            if sub_nodes.is_empty() {
                return Err(Error::EmptyNode { src: None, span });
            }
            let Node::Ident { span } = &sub_nodes[0] else {
                return Err(Error::UnexpectedToken {
                    src: None,
                    span
                });
            };
            let keyword = &module.src[span.clone()];
            let Some(static_func) = STATIC_FUNCS.get(keyword) else {
                return Err(Error::UnknownStaticFunc { src: None, span: span.clone() });
            };
            *data = StaticData::Intermediary {
                value: (*static_func)(span.clone(), sub_nodes)?,
            };
            Ok(())
        }
    }
}

fn gen_scope_intermediary(
    task: &mut CompileTask,
    module_index: usize,
    func_index: usize,
) -> Result<()> {
    let module = &mut task.modules[module_index];
    let Func {
        public: _,
        params: _,
        return_: _,
        data,
        used: _,
    } = &mut module.funcs[func_index];
    let FuncData::Collected { nodes } = data else {unreachable!()};
    let mut ir = Vec::with_capacity(0);
    let expr = nodes.pop().unwrap();
    match expr {
        Node::Ident { span } => {
            let name = &module.src[span.clone()];
            let Some(static_index) = module.static_indices.get(name) else {
                return Err(Error::UnknownStaticVariable { src: None, span });
            };
            let static_ = &mut module.statics[*static_index];
            let StaticData::Intermediary { value } = &static_.data else {unreachable!()};
            match value {
                IntermediaryStaticValue::Int(_)
                | IntermediaryStaticValue::UInt(_)
                | IntermediaryStaticValue::Float(_) => {
                    ir.push(Insn::LdStaticValue {
                        dst: Reg::R0,
                        index: *static_index,
                    });
                }
                IntermediaryStaticValue::String(value) => {
                    ir.push(Insn::LdStaticAbsAddr {
                        dst: Reg::R0,
                        index: *static_index,
                    });
                    ir.push(Insn::LdcUInt {
                        dst: Reg::R1,
                        value: value.len() as u64,
                    });
                }
                IntermediaryStaticValue::Buffer { size } => {
                    ir.push(Insn::LdStaticAbsAddr {
                        dst: Reg::R0,
                        index: *static_index,
                    });
                    ir.push(Insn::LdcUInt {
                        dst: Reg::R1,
                        value: *size as u64,
                    });
                }
            }
            ir.push(Insn::Ret);
            *data = FuncData::Intermediary { ir };
            Ok(())
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
            *data = FuncData::Intermediary { ir };
            Ok(())
        }
        Node::String { value, .. } => {
            let len = value.len();
            module.statics.push(Static {
                data: StaticData::Intermediary {
                    value: IntermediaryStaticValue::String(value),
                },
                used: true,
            });
            ir.push(Insn::LdStaticAbsAddr {
                dst: Reg::R0,
                index: module.statics.len() - 1,
            });
            ir.push(Insn::LdcUInt {
                dst: Reg::R1,
                value: len as u64,
            });
            ir.push(Insn::Ret);
            *data = FuncData::Intermediary { ir };
            Ok(())
        }
        Node::Node { span, sub_nodes } => {
            gen_scope_node_intermediary(task, module_index, &mut ir, sub_nodes, span, 0)?;
            task.modules[module_index].funcs[func_index].data = FuncData::Intermediary { ir };
            Ok(())
        }
    }
}

fn gen_scope_node_intermediary(
    task: &mut CompileTask,
    module_index: usize,
    ir: &mut Vec<Insn>,
    sub_nodes: Vec<Node>,
    span: Span,
    depth: usize,
) -> Result<()> {
    let module = &mut task.modules[module_index];
    if sub_nodes.is_empty() {
        if depth == 0 {
            ir.push(Insn::Ret);
        }
        return Ok(());
    }
    let Node::Ident { span: name_span } = &sub_nodes[0] else {
        return Err(Error::UnexpectedToken {
            src: None,
            span: sub_nodes[0].span(),
        });
    };
    let name = &module.src[name_span.clone()];
    match name {
        "do" => {
            let mut sub_nodes = sub_nodes.into_iter().peekable();
            sub_nodes.next().unwrap();
            while let Some(node) = sub_nodes.next() {
                match node {
                    Node::Ident { span } => {
                        if sub_nodes.peek().is_some() {
                            return Err(Error::UnexpectedToken { src: None, span });
                        }
                        todo!()
                    }
                    node @ Node::Int { .. }
                    | node @ Node::UInt { .. }
                    | node @ Node::Float { .. } => {
                        if sub_nodes.peek().is_some() {
                            return Err(Error::UnexpectedToken {
                                src: None,
                                span: node.span(),
                            });
                        }
                        match node {
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
                    }
                    Node::String { span, value } => {
                        if sub_nodes.peek().is_some() {
                            return Err(Error::UnexpectedToken { src: None, span });
                        }
                        let len = value.len();
                        task.modules[module_index].statics.push(Static {
                            data: StaticData::Intermediary {
                                value: IntermediaryStaticValue::String(value),
                            },
                            used: true,
                        });
                        ir.push(Insn::LdStaticAbsAddr {
                            dst: Reg::R0,
                            index: task.modules[module_index].statics.len() - 1,
                        });
                        ir.push(Insn::LdcUInt {
                            dst: Reg::R1,
                            value: len as u64,
                        });
                    }
                    Node::Node { span, sub_nodes } => {
                        gen_scope_node_intermediary(
                            task,
                            module_index,
                            ir,
                            sub_nodes,
                            span,
                            depth + 1,
                        )?;
                    }
                }
            }
            if depth == 0 {
                ir.push(Insn::Ret);
            }
        }
        "if" => {
            todo!()
        }
        "while" => {
            todo!()
        }
        _ => {
            if let Some(macro_) = MACROS.get(name) {
                (*macro_)(task, module_index, ir, span, sub_nodes)?;
                if depth == 0 {
                    ir.push(Insn::Ret);
                }
                return Ok(());
            };
            if let Some(insns) = INSN_MACROS.get(name) {
                let insn = insns::find(insns, &module.src, span.clone(), &sub_nodes)?;
                if let Some(insn) = insn {
                    insn.gen(&module.src, ir, sub_nodes)?;
                    if depth == 0 {
                        ir.push(Insn::Ret);
                    }
                    return Ok(());
                }
            }
            if sub_nodes.len() == 1 {
                if let Some(func_index) = module.func_indices.get(name) {
                    ir.push(Insn::BrLabelLinked { index: *func_index });
                    if depth == 0 {
                        ir.push(Insn::Ret);
                    }
                    return Ok(());
                }
            }
            return Err(Error::UnknownFunc { src: None, span });
        }
    }
    Ok(())
}
