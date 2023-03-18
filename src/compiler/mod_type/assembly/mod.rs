pub mod insns;
pub mod macros;
pub mod static_funcs;

use std::mem;

use urban_common::opcodes::{L0_MOV, L0_MOVS};

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
        main: bool,
    ) -> Result<()> {
        let module = &mut task.modules[module_index];
        let mut nodes = root.into_iter();
        nodes.next().unwrap();
        for node in nodes {
            let Node::Node { span, sub_nodes } = node else {
                panic!("Invalid AST");
            };
            if sub_nodes.is_empty() {
                return Err(Error::EmptyNode {
                    file: mem::take(&mut module.file),
                    src: mem::take(&mut module.src),
                    span,
                });
            }
            let Node::Ident { span: keyword_span } = &sub_nodes[0] else {
                return Err(Error::UnexpectedToken {
                    file: mem::take(&mut module.file),
                    src: mem::take(&mut module.src),
                    span: sub_nodes[0].span(),
                });
            };
            let keyword = &module.src[keyword_span.clone()];
            match keyword {
                "static" => {
                    if sub_nodes.len() != 3 {
                        return Err(Error::InvalidStatement {
                            file: mem::take(&mut module.file),
                            src: mem::take(&mut module.src),
                            span,
                        });
                    }
                    let Node::Ident { span: name_span } = &sub_nodes[1] else {
                        return Err(Error::UnexpectedToken {
                            file: mem::take(&mut module.file),
                            src: mem::take(&mut module.src),
                            span: sub_nodes[1].span(),
                        });
                    };
                    let name = &module.src[name_span.clone()];
                    if module.static_indices.contains_key(name) {
                        return Err(Error::DuplicateName {
                            file: mem::take(&mut module.file),
                            src: mem::take(&mut module.src),
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
                    let public = keyword.starts_with('+');
                    if sub_nodes.len() != 3 {
                        return Err(Error::InvalidStatement {
                            file: mem::take(&mut module.file),
                            src: mem::take(&mut module.src),
                            span,
                        });
                    }
                    let Node::Ident { span: name_span } = &sub_nodes[1] else {
                        return Err(Error::UnexpectedToken {
                            file: mem::take(&mut module.file),
                            src: mem::take(&mut module.src),
                            span: sub_nodes[1].span(),
                        });
                    };
                    let name = &module.src[name_span.clone()];
                    if module.func_indices.contains_key(name) {
                        return Err(Error::DuplicateName {
                            file: mem::take(&mut module.file),
                            src: mem::take(&mut module.src),
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
                    if main && name == "main" {
                        task.main = Some((module_index, module.funcs.len() - 1));
                    }
                }
                _ => {
                    return Err(Error::InvalidKeyword {
                        file: mem::take(&mut module.file),
                        src: mem::take(&mut module.src),
                        span: keyword_span.clone(),
                    })
                }
            }
        }
        if main && task.main.is_none() {
            return Err(Error::NoMainFound {
                file: mem::take(&mut module.file),
            });
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
        Node::Ident { span } => Err(Error::UnexpectedToken {
            file: mem::take(&mut module.file),
            src: mem::take(&mut module.src),
            span,
        }),
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
                return Err(Error::EmptyNode {
                    file: mem::take(&mut module.file),
                    src: mem::take(&mut module.src),
                    span,
                });
            }
            let Node::Ident { span } = &sub_nodes[0] else {
                return Err(Error::UnexpectedToken {
                    file: mem::take(&mut module.file),
                    src: mem::take(&mut module.src),
                    span
                });
            };
            let keyword = &module.src[span.clone()];
            let Some(static_func) = STATIC_FUNCS.get(keyword) else {
                return Err(Error::UnknownStaticFunc {
                    file: mem::take(&mut module.file),
                    src: mem::take(&mut module.src),
                    span: span.clone(),
                });
            };
            let value = (*static_func)(task, module_index, span.clone(), sub_nodes)?;
            task.modules[module_index].statics[static_index].data =
                StaticData::Intermediary { value };
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
                return Err(Error::UnknownStaticVariable {
                    file: mem::take(&mut module.file),
                    src: mem::take(&mut module.src),
                    span,
                });
            };
            let static_ = &mut module.statics[*static_index];
            let StaticData::Intermediary { value } = &static_.data else {unreachable!()};
            match value {
                IntermediaryStaticValue::Int(_)
                | IntermediaryStaticValue::UInt(_)
                | IntermediaryStaticValue::Float(_) => {
                    ir.push(Insn::LoadStatic {
                        dst: Reg::R0,
                        index: *static_index,
                    });
                }
                IntermediaryStaticValue::String(_) => {
                    ir.push(Insn::LdStaticAbsAddr {
                        dst: Reg::R0,
                        index: *static_index,
                    });
                }
                IntermediaryStaticValue::Buffer { size: _ } => {
                    ir.push(Insn::LdStaticAbsAddr {
                        dst: Reg::R0,
                        index: *static_index,
                    });
                }
            }
            ir.push(Insn::Ret);
            *data = FuncData::Intermediary { ir };
            Ok(())
        }
        Node::Int { .. } | Node::UInt { .. } | Node::Float { .. } => {
            match expr {
                Node::Int { value, .. } => {
                    if (-(1 << 22)..((1 << 22) - 1)).contains(&value) {
                        ir.push(Insn::Raw(L0_MOVS | value as u32 & ((1 << 22) - 1)));
                    } else {
                        module.statics.push(Static {
                            data: StaticData::Intermediary {
                                value: IntermediaryStaticValue::Int(value),
                            },
                            used: false,
                        });
                        ir.push(Insn::LoadStatic {
                            dst: Reg::R0,
                            index: module.statics.len() - 1,
                        });
                    }
                }
                Node::UInt { value, .. } => {
                    if value < (1 << 22) {
                        ir.push(Insn::Raw(L0_MOV | value as u32 & ((1 << 22) - 1)))
                    } else {
                        module.statics.push(Static {
                            data: StaticData::Intermediary {
                                value: IntermediaryStaticValue::UInt(value),
                            },
                            used: false,
                        });
                        ir.push(Insn::LoadStatic {
                            dst: Reg::R0,
                            index: module.statics.len() - 1,
                        });
                    }
                }
                Node::Float { value, .. } => {
                    module.statics.push(Static {
                        data: StaticData::Intermediary {
                            value: IntermediaryStaticValue::Float(value),
                        },
                        used: false,
                    });
                    ir.push(Insn::LoadStatic {
                        dst: Reg::R0,
                        index: module.statics.len() - 1,
                    });
                }
                _ => unreachable!(),
            }
            ir.push(Insn::Ret);
            *data = FuncData::Intermediary { ir };
            Ok(())
        }
        Node::String { value, .. } => {
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
    let mut module = &mut task.modules[module_index];
    if sub_nodes.is_empty() {
        if depth == 0 {
            ir.push(Insn::Ret);
        }
        return Ok(());
    }
    let Node::Ident { span: name_span } = &sub_nodes[0] else {
        return Err(Error::UnexpectedToken {
            file: mem::take(&mut module.file),
            src: mem::take(&mut module.src),
            span: sub_nodes[0].span(),
        });
    };
    let mut name = &module.src[name_span.clone()];
    match name {
        "do" => {
            let mut sub_nodes = sub_nodes.into_iter().peekable();
            sub_nodes.next().unwrap();
            while let Some(node) = sub_nodes.next() {
                match node {
                    Node::Ident { span } => {
                        if sub_nodes.peek().is_some() {
                            return Err(Error::UnexpectedToken {
                                file: mem::take(&mut module.file),
                                src: mem::take(&mut module.src),
                                span,
                            });
                        }
                        todo!()
                    }
                    node @ Node::Int { .. }
                    | node @ Node::UInt { .. }
                    | node @ Node::Float { .. } => {
                        if sub_nodes.peek().is_some() {
                            return Err(Error::UnexpectedToken {
                                file: mem::take(&mut module.file),
                                src: mem::take(&mut module.src),
                                span: node.span(),
                            });
                        }
                        match node {
                            Node::Int { value, .. } => {
                                if (-(1 << 22)..((1 << 22) - 1)).contains(&value) {
                                    ir.push(Insn::Raw(L0_MOVS | value as u32 & ((1 << 22) - 1)));
                                } else {
                                    module.statics.push(Static {
                                        data: StaticData::Intermediary {
                                            value: IntermediaryStaticValue::Int(value),
                                        },
                                        used: false,
                                    });
                                    ir.push(Insn::LoadStatic {
                                        dst: Reg::R0,
                                        index: module.statics.len() - 1,
                                    });
                                }
                            }
                            Node::UInt { value, .. } => {
                                if value < (1 << 22) {
                                    ir.push(Insn::Raw(L0_MOV | value as u32 & ((1 << 22) - 1)))
                                } else {
                                    module.statics.push(Static {
                                        data: StaticData::Intermediary {
                                            value: IntermediaryStaticValue::UInt(value),
                                        },
                                        used: false,
                                    });
                                    ir.push(Insn::LoadStatic {
                                        dst: Reg::R0,
                                        index: module.statics.len() - 1,
                                    });
                                }
                            }
                            Node::Float { value, .. } => {
                                module.statics.push(Static {
                                    data: StaticData::Intermediary {
                                        value: IntermediaryStaticValue::Float(value),
                                    },
                                    used: false,
                                });
                                ir.push(Insn::LoadStatic {
                                    dst: Reg::R0,
                                    index: module.statics.len() - 1,
                                });
                            }
                            _ => unreachable!(),
                        }
                    }
                    Node::String { span, value } => {
                        if sub_nodes.peek().is_some() {
                            return Err(Error::UnexpectedToken {
                                file: mem::take(&mut module.file),
                                src: mem::take(&mut module.src),
                                span,
                            });
                        }
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
                        module = &mut task.modules[module_index];
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
                let insn = insns::find(task, module_index, insns, span, &sub_nodes)?;
                module = &mut task.modules[module_index];
                name = &module.src[name_span.clone()];
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
                    ir.push(Insn::BrLabelLinked {
                        module_index,
                        func_index: *func_index,
                    });
                    if depth == 0 {
                        ir.push(Insn::Ret);
                    }
                    return Ok(());
                }
            }
            return Err(Error::UnknownFunc {
                file: mem::take(&mut module.file),
                src: mem::take(&mut module.src),
                span: name_span.clone(),
            });
        }
    }
    Ok(())
}
