pub mod insns;
pub mod macros;
pub mod static_funcs;

use std::{collections::HashMap, mem};

use leviathan_ir::{
    binary::{BinaryFunc, BinaryModule, BinaryStatic},
    layers::{
        lower::{LowOp, LowerLayer, Reg},
        Coord,
    },
};

use crate::{
    compiler::{
        dialect::assembly::{insns::INSN_MACROS, macros::MACROS},
        error::{Error, Result},
        CompileTask, Dialect, Func, FuncData, Static, StaticData, Type, UncollectedModule,
    },
    parser::{BracketType, Node},
    util::source::Span,
};

use self::static_funcs::STATIC_FUNCS;

pub struct AssemblyLanguage {
    pub unresolved_imports: Vec<Import>,
    pub imports: Vec<usize>,
    pub label_indices: HashMap<String, usize>,
    pub labels: Vec<Func>,
    pub static_indices: HashMap<String, usize>,
    pub statics: Vec<Static>,
}

impl Dialect for AssemblyLanguage {
    fn collect(
        &mut self,
        task: &mut CompileTask,
        module_index: usize,
        UncollectedModule { root }: UncollectedModule,
        main: bool,
    ) -> Result<()> {
        let module = &mut task.modules[module_index];
        let mut nodes = root.into_iter();
        nodes.next().unwrap();
        for node in nodes {
            let Node::Node {
                span,
                type_: BracketType::Round,
                mut sub_nodes,
            } = node else
            {
                panic!("Invalid AST");
            };
            if sub_nodes.is_empty() {
                return Err(Error::EmptyNode {
                    file: module.take_file(),
                    src: module.take_src(),
                    span,
                });
            }
            let Node::Ident { span: keyword_span } = &sub_nodes[0] else {
                return Err(Error::UnexpectedToken {
                    file: module.take_file(),
                    src: module.take_src(),
                    span: sub_nodes[0].span(),
                });
            };
            let keyword = &module.src[keyword_span.clone()];
            match keyword {
                "use" => {
                    if sub_nodes.len() != 2 {
                        return Err(Error::InvalidStatement {
                            file: module.take_file(),
                            src: module.take_src(),
                            span,
                        });
                    }
                    let Node::Ident { .. } = &sub_nodes[1] else {
                        return Err(Error::UnexpectedToken {
                            file: module.take_file(),
                            src: module.take_src(),
                            span: sub_nodes[1].span(),
                        });
                    };
                    self.unresolved_imports.push(Import {
                        node: sub_nodes.pop().unwrap(),
                    });
                }
                "static" => {
                    if sub_nodes.len() != 3 {
                        return Err(Error::InvalidStatement {
                            file: module.take_file(),
                            src: module.take_src(),
                            span,
                        });
                    }
                    let Node::Ident { span: name_span } = &sub_nodes[1] else {
                        return Err(Error::UnexpectedToken {
                            file: module.take_file(),
                            src: module.take_src(),
                            span: sub_nodes[1].span(),
                        });
                    };
                    let name = &module.src[name_span.clone()];
                    if self.static_indices.contains_key(name) {
                        return Err(Error::DuplicateName {
                            file: module.take_file(),
                            src: module.take_src(),
                            span: name_span.clone(),
                        });
                    }
                    self.statics.push(Static {
                        data: StaticData {
                            node: sub_nodes.pop().unwrap(),
                        },
                        used: false,
                    });
                    self.static_indices
                        .insert(name.to_string(), self.statics.len() - 1);
                }
                "-label" | "+label" => {
                    let public = keyword.starts_with('+');
                    if sub_nodes.len() != 3 {
                        return Err(Error::InvalidStatement {
                            file: module.take_file(),
                            src: module.take_src(),
                            span,
                        });
                    }
                    let Node::Ident { span: name_span } = &sub_nodes[1] else {
                        return Err(Error::UnexpectedToken {
                            file: module.take_file(),
                            src: module.take_src(),
                            span: sub_nodes[1].span(),
                        });
                    };
                    let name = &module.src[name_span.clone()];
                    if self.label_indices.contains_key(name) {
                        return Err(Error::DuplicateName {
                            file: module.take_file(),
                            src: module.take_src(),
                            span: name_span.clone(),
                        });
                    }
                    self.labels.push(Func {
                        public,
                        params: vec![(None, Type::Unknown)],
                        return_: Type::Unknown,
                        data: FuncData {
                            node: sub_nodes.pop().unwrap(),
                        },
                        used: false,
                    });
                    self.label_indices
                        .insert(name.to_string(), self.labels.len() - 1);
                    if main && name == "main" {
                        task.main = Some(Coord {
                            module: module_index,
                            element: self.labels.len() - 1,
                        });
                    }
                }
                _ => {
                    return Err(Error::InvalidKeyword {
                        file: module.take_file(),
                        src: module.take_src(),
                        span: keyword_span.clone(),
                    })
                }
            }
        }
        if main && task.main.is_none() {
            return Err(Error::NoMainFound {
                file: module.take_file(),
            });
        }
        Ok(())
    }

    fn compile_module(
        &mut self,
        task: &mut CompileTask,
        module_index: usize,
    ) -> Result<BinaryModule> {
        let mut binary_mod = BinaryModule::default();
        let module = &mut task.modules[module_index];
        let statics_len = self.statics.len();
        let funcs_len = self.labels.len();
        for import in self.unresolved_imports.drain(..) {
            let Node::Ident { span } = import.node else {unreachable!()};
            let name = &module.src[span.clone()];
            let Some(include) = task.module_indices.get(name) else {
                return Err(Error::UnknownModule {
                    file: module.take_file(),
                    src: module.take_src(),
                    span,
                });
            };
            if *include == module_index {
                return Err(Error::SelfImport {
                    file: module.take_file(),
                    src: module.take_src(),
                    span,
                });
            }
            self.imports.push(*include);
        }
        for static_index in 0..statics_len {
            let static_ = compile_static(self, task, module_index, static_index)?;
            binary_mod.statics.insert(static_index, static_);
        }
        for func_index in 0..funcs_len {
            let func = compile_label(self, task, module_index, func_index, &mut binary_mod)?;
            binary_mod.funcs.insert(func_index, func);
        }
        Ok(binary_mod)
    }

    fn lookup_callable(&self, name: &str) -> Option<usize> {
        let Some(index) = self.label_indices.get(name).cloned() else {
            return None;
        };
        if !self.labels[index].public {
            return None;
        }
        Some(index)
    }
}

impl Default for AssemblyLanguage {
    fn default() -> Self {
        Self {
            unresolved_imports: Vec::with_capacity(0),
            imports: Vec::with_capacity(0),
            label_indices: HashMap::with_capacity(0),
            labels: Vec::with_capacity(0),
            static_indices: HashMap::with_capacity(0),
            statics: Vec::with_capacity(0),
        }
    }
}

#[derive(Debug)]
pub struct Import {
    pub node: Node,
}

fn compile_static(
    dialect: &mut AssemblyLanguage,
    task: &mut CompileTask,
    module_index: usize,
    static_index: usize,
) -> Result<BinaryStatic> {
    let module = &mut task.modules[module_index];
    let Static { data, used: _ } = &mut dialect.statics[static_index];
    let StaticData { node } = mem::take(data);
    match node {
        Node::Ident { span } => Err(Error::UnexpectedToken {
            file: module.take_file(),
            src: module.take_src(),
            span,
        }),
        Node::Int { value, .. } => Ok(BinaryStatic::Int(value)),
        Node::UInt { value, .. } => Ok(BinaryStatic::UInt(value)),
        Node::Float { value, .. } => Ok(BinaryStatic::Float(value)),
        Node::String { value, .. } => Ok(BinaryStatic::String(value)),
        Node::Node {
            span,
            type_,
            sub_nodes,
        } => {
            if type_ != BracketType::Round {
                return Err(Error::InvalidBracketType {
                    file: module.take_file(),
                    src: module.take_src(),
                    span,
                });
            }
            if sub_nodes.is_empty() {
                return Err(Error::EmptyNode {
                    file: module.take_file(),
                    src: module.take_src(),
                    span,
                });
            }
            let Node::Ident { span } = &sub_nodes[0] else {
                return Err(Error::UnexpectedToken {
                    file: module.take_file(),
                    src: module.take_src(),
                    span
                });
            };
            let keyword = &module.src[span.clone()];
            let Some(static_func) = STATIC_FUNCS.get(keyword) else {
                return Err(Error::UnknownStaticFunc {
                    file: module.take_file(),
                    src: module.take_src(),
                    span: span.clone(),
                });
            };
            let value = (*static_func)(task, module_index, span.clone(), sub_nodes)?;
            Ok(value)
        }
        _ => unreachable!(),
    }
}

fn compile_label(
    dialect: &mut AssemblyLanguage,
    task: &mut CompileTask,
    module_index: usize,
    func_index: usize,
    binary_mod: &mut BinaryModule,
) -> Result<BinaryFunc> {
    let mut binary_func = LowerLayer::default();
    let module = &mut task.modules[module_index];
    let Func {
        public: _,
        params: _,
        return_: _,
        data,
        used: _,
    } = &mut dialect.labels[func_index];
    let FuncData { node } = mem::take(data);
    match node {
        Node::Ident { span } => {
            let name = &module.src[span.clone()];
            let Some(static_index) = dialect.static_indices.get(name) else {
                return Err(Error::UnknownStaticVariable {
                    file: module.take_file(),
                    src: module.take_src(),
                    span,
                });
            };
            let static_ = binary_mod.statics.get_mut(static_index).unwrap();
            match static_ {
                BinaryStatic::Int(_) | BinaryStatic::UInt(_) | BinaryStatic::Float(_) => {
                    binary_func.ops.push(LowOp::LoadStatic64 {
                        dst: Reg::new(0),
                        coord: Coord {
                            module: module_index,
                            element: *static_index,
                        },
                    });
                }
                BinaryStatic::String(_) | BinaryStatic::FilledBuffer { .. } => {
                    binary_func.ops.push(LowOp::LoadStatic64 {
                        dst: Reg::new(0),
                        coord: Coord {
                            module: module_index,
                            element: *static_index,
                        },
                    });
                }
            }
            binary_func.ops.push(LowOp::Return);
            Ok(binary_func.to_func())
        }
        Node::Int { .. } | Node::UInt { .. } | Node::Float { .. } => {
            match node {
                Node::Int { value, .. } => {
                    if (-(1 << 22)..((1 << 22) - 1)).contains(&value) {
                        binary_func.ops.push(LowOp::MoveSignedImmediate {
                            dst: Reg::new(0),
                            immediate: value as i32,
                        });
                    } else {
                        binary_func.locals.push(BinaryStatic::Int(value));
                        binary_func.ops.push(LowOp::LoadLocalStatic64 {
                            dst: Reg::new(0),
                            coord: binary_func.locals.len() - 1,
                        });
                    }
                }
                Node::UInt { value, .. } => {
                    if value < (1 << 22) {
                        binary_func.ops.push(LowOp::MoveImmediate {
                            dst: Reg::new(0),
                            immediate: value as u32,
                        });
                    } else {
                        binary_func.locals.push(BinaryStatic::UInt(value));
                        binary_func.ops.push(LowOp::LoadLocalStatic64 {
                            dst: Reg::new(0),
                            coord: binary_func.locals.len() - 1,
                        });
                    }
                }
                Node::Float { value, .. } => {
                    if value == 0.0 {
                        binary_func.ops.push(LowOp::MoveImmediate {
                            dst: Reg::new(0),
                            immediate: 0,
                        });
                    } else {
                        binary_func.locals.push(BinaryStatic::Float(value));
                        binary_func.ops.push(LowOp::LoadLocalStatic64 {
                            dst: Reg::new(0),
                            coord: binary_func.locals.len() - 1,
                        });
                    }
                }
                _ => unreachable!(),
            }
            binary_func.ops.push(LowOp::Return);
            Ok(binary_func.to_func())
        }
        Node::String { value, .. } => {
            binary_func.locals.push(BinaryStatic::String(value));
            binary_func.ops.push(LowOp::LoadLocalStaticAddress {
                dst: Reg::new(0),
                coord: binary_func.locals.len() - 1,
            });
            binary_func.ops.push(LowOp::Return);
            Ok(binary_func.to_func())
        }
        Node::Node {
            span,
            type_,
            sub_nodes,
        } => {
            if type_ != BracketType::Round {
                return Err(Error::InvalidBracketType {
                    file: module.take_file(),
                    src: module.take_src(),
                    span,
                });
            }
            compile_label_node(
                dialect,
                task,
                module_index,
                &mut binary_func,
                sub_nodes,
                span,
                0,
            )?;
            Ok(binary_func.to_func())
        }
        _ => unreachable!(),
    }
}

fn compile_label_node(
    dialect: &mut AssemblyLanguage,
    task: &mut CompileTask,
    module_index: usize,
    binary_func: &mut LowerLayer,
    mut sub_nodes: Vec<Node>,
    span: Span,
    depth: usize,
) -> Result<()> {
    let mut module = &mut task.modules[module_index];
    if sub_nodes.is_empty() {
        if depth == 0 {
            binary_func.ops.push(LowOp::Return);
        }
        return Ok(());
    }
    let Node::Ident { span: name_span } = &sub_nodes[0] else {
        return Err(Error::UnexpectedToken {
            file: module.take_file(),
            src: module.take_src(),
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
                                file: module.take_file(),
                                src: module.take_src(),
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
                                file: module.take_file(),
                                src: module.take_src(),
                                span: node.span(),
                            });
                        }
                        match node {
                            Node::Int { value, .. } => {
                                if (-(1 << 22)..((1 << 22) - 1)).contains(&value) {
                                    binary_func.ops.push(LowOp::MoveSignedImmediate {
                                        dst: Reg::new(0),
                                        immediate: value as i32,
                                    });
                                } else {
                                    binary_func.locals.push(BinaryStatic::Int(value));
                                    binary_func.ops.push(LowOp::LoadLocalStatic64 {
                                        dst: Reg::new(0),
                                        coord: binary_func.locals.len() - 1,
                                    });
                                }
                            }
                            Node::UInt { value, .. } => {
                                if value < (1 << 22) {
                                    binary_func.ops.push(LowOp::MoveImmediate {
                                        dst: Reg::new(0),
                                        immediate: value as u32,
                                    });
                                } else {
                                    binary_func.locals.push(BinaryStatic::UInt(value));
                                    binary_func.ops.push(LowOp::LoadLocalStatic64 {
                                        dst: Reg::new(0),
                                        coord: binary_func.locals.len() - 1,
                                    });
                                }
                            }
                            Node::Float { value, .. } => {
                                if value == 0.0 {
                                    binary_func.ops.push(LowOp::MoveImmediate {
                                        dst: Reg::new(0),
                                        immediate: 0,
                                    });
                                } else {
                                    binary_func.locals.push(BinaryStatic::Float(value));
                                    binary_func.ops.push(LowOp::LoadLocalStatic64 {
                                        dst: Reg::new(0),
                                        coord: binary_func.locals.len() - 1,
                                    });
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    Node::String { span, value } => {
                        if sub_nodes.peek().is_some() {
                            return Err(Error::UnexpectedToken {
                                file: module.take_file(),
                                src: module.take_src(),
                                span,
                            });
                        }
                        binary_func.locals.push(BinaryStatic::String(value));
                        binary_func.ops.push(LowOp::LoadLocalStaticAddress {
                            dst: Reg::new(0),
                            coord: binary_func.locals.len() - 1,
                        });
                    }
                    Node::Node {
                        span,
                        type_,
                        sub_nodes,
                    } => {
                        if type_ != BracketType::Round {
                            return Err(Error::InvalidBracketType {
                                file: module.take_file(),
                                src: module.take_src(),
                                span,
                            });
                        }
                        compile_label_node(
                            dialect,
                            task,
                            module_index,
                            binary_func,
                            sub_nodes,
                            span,
                            depth + 1,
                        )?;
                        module = &mut task.modules[module_index];
                    }
                    _ => unreachable!(),
                }
            }
            if depth == 0 {
                binary_func.ops.push(LowOp::Return);
            }
        }
        "if" => {
            if sub_nodes.len() != 4 {
                return Err(Error::InvalidStatement {
                    file: module.take_file(),
                    src: module.take_src(),
                    span,
                });
            }
            let Node::Ident { span: cond_span } = &sub_nodes[1] else {
                return Err(Error::UnexpectedToken {
                    file: module.take_file(),
                    src: module.take_src(),
                    span: sub_nodes[1].span(),
                });
            };
            let cond = &module.src[cond_span.clone()];
            let Node::Ident { span: reg_span } = &sub_nodes[2] else {
                return Err(Error::UnexpectedToken {
                    file: module.take_file(),
                    src: module.take_src(),
                    span: sub_nodes[2].span(),
                });
            };
            let reg = &module.src[reg_span.clone()];
            if !reg.starts_with('r') && !reg.starts_with('R') {
                return Err(Error::InvalidRegister {
                    file: module.take_file(),
                    src: module.take_src(),
                    span: reg_span.clone(),
                });
            }
            let Ok(reg) = reg[1..].parse::<usize>() else {
                return Err(Error::InvalidRegister {
                    file: module.take_file(),
                    src: module.take_src(),
                    span: reg_span.clone(),
                });
            };
            if reg > 31 {
                return Err(Error::InvalidRegister {
                    file: module.take_file(),
                    src: module.take_src(),
                    span: reg_span.clone(),
                });
            }
            let reg = Reg::new(reg as u8);
            let pos = binary_func.alloc_coord();
            match cond {
                "=" => binary_func
                    .ops
                    .push(LowOp::BranchCoordNonEqual { reg, coord: pos }),
                "!=" => binary_func
                    .ops
                    .push(LowOp::BranchCoordEqual { reg, coord: pos }),
                "<" => binary_func
                    .ops
                    .push(LowOp::BranchCoordGreaterEqual { reg, coord: pos }),
                ">" => binary_func
                    .ops
                    .push(LowOp::BranchCoordLessEqual { reg, coord: pos }),
                "<=" => binary_func
                    .ops
                    .push(LowOp::BranchCoordGreater { reg, coord: pos }),
                ">=" => binary_func
                    .ops
                    .push(LowOp::BranchCoordLess { reg, coord: pos }),
                "!0" => binary_func
                    .ops
                    .push(LowOp::BranchCoordIfZero { reg, coord: pos }),
                "=0" => binary_func
                    .ops
                    .push(LowOp::BranchCoordIfNonZero { reg, coord: pos }),
                _ => {
                    return Err(Error::InvalidCondition {
                        file: module.take_file(),
                        src: module.take_src(),
                        span: cond_span.clone(),
                    });
                }
            }
            let expr = sub_nodes.pop().unwrap();
            let Node::Node { span, type_, sub_nodes } = expr else {
                return Err(Error::UnexpectedToken {
                    file: module.take_file(),
                    src: module.take_src(),
                    span: expr.span(),
                });
            };
            if type_ != BracketType::Round {
                return Err(Error::InvalidBracketType {
                    file: module.take_file(),
                    src: module.take_src(),
                    span,
                });
            }
            compile_label_node(
                dialect,
                task,
                module_index,
                binary_func,
                sub_nodes,
                span,
                depth + 1,
            )?;
            binary_func.ops.push(LowOp::PutCoord { coord: pos });
            if depth == 0 {
                binary_func.ops.push(LowOp::Return);
            }
        }
        "while" => {
            if sub_nodes.len() != 4 {
                return Err(Error::InvalidStatement {
                    file: module.take_file(),
                    src: module.take_src(),
                    span,
                });
            }
            let Node::Ident { span: cond_span } = &sub_nodes[1] else {
                return Err(Error::UnexpectedToken {
                    file: module.take_file(),
                    src: module.take_src(),
                    span: sub_nodes[1].span(),
                });
            };
            let cond = &module.src[cond_span.clone()];
            let Node::Ident { span: reg_span } = &sub_nodes[2] else {
                return Err(Error::UnexpectedToken {
                    file: module.take_file(),
                    src: module.take_src(),
                    span: sub_nodes[2].span(),
                });
            };
            let reg = &module.src[reg_span.clone()];
            if !reg.starts_with('r') && !reg.starts_with('R') {
                return Err(Error::InvalidRegister {
                    file: module.take_file(),
                    src: module.take_src(),
                    span: reg_span.clone(),
                });
            }
            let Ok(reg) = reg[1..].parse::<usize>() else {
                return Err(Error::InvalidRegister {
                    file: module.take_file(),
                    src: module.take_src(),
                    span: reg_span.clone(),
                });
            };
            if reg > 31 {
                return Err(Error::InvalidRegister {
                    file: module.take_file(),
                    src: module.take_src(),
                    span: reg_span.clone(),
                });
            }
            let reg = Reg::new(reg as u8);
            let pos = binary_func.alloc_coord();
            let cond_pos = binary_func.alloc_coord();
            binary_func.ops.push(LowOp::BranchCoord { coord: cond_pos });
            binary_func.ops.push(LowOp::PutCoord { coord: pos });
            let insn = match cond {
                "=" => LowOp::BranchCoordEqual { coord: pos, reg },
                "!=" => LowOp::BranchCoordNonEqual { coord: pos, reg },
                "<" => LowOp::BranchCoordLess { coord: pos, reg },
                ">" => LowOp::BranchCoordGreater { coord: pos, reg },
                "<=" => LowOp::BranchCoordLessEqual { coord: pos, reg },
                ">=" => LowOp::BranchCoordGreaterEqual { coord: pos, reg },
                "!0" => LowOp::BranchCoordIfNonZero { coord: pos, reg },
                "=0" => LowOp::BranchCoordIfZero { coord: pos, reg },
                _ => {
                    return Err(Error::InvalidCondition {
                        file: module.take_file(),
                        src: module.take_src(),
                        span: cond_span.clone(),
                    });
                }
            };
            let expr = sub_nodes.pop().unwrap();
            let Node::Node { span, type_, sub_nodes } = expr else {
                return Err(Error::UnexpectedToken {
                    file: module.take_file(),
                    src: module.take_src(),
                    span: expr.span(),
                });
            };
            if type_ != BracketType::Round {
                return Err(Error::InvalidBracketType {
                    file: module.take_file(),
                    src: module.take_src(),
                    span,
                });
            }
            compile_label_node(
                dialect,
                task,
                module_index,
                binary_func,
                sub_nodes,
                span,
                depth + 1,
            )?;
            binary_func.ops.push(LowOp::PutCoord { coord: cond_pos });
            binary_func.ops.push(insn);
            if depth == 0 {
                binary_func.ops.push(LowOp::Return);
            }
        }
        "do-while" => {
            if sub_nodes.len() != 4 {
                return Err(Error::InvalidStatement {
                    file: module.take_file(),
                    src: module.take_src(),
                    span,
                });
            }
            let Node::Ident { span: cond_span } = &sub_nodes[2] else {
                return Err(Error::UnexpectedToken {
                    file: module.take_file(),
                    src: module.take_src(),
                    span: sub_nodes[2].span(),
                });
            };
            let cond = &module.src[cond_span.clone()];
            let Node::Ident { span: reg_span } = &sub_nodes[3] else {
                return Err(Error::UnexpectedToken {
                    file: module.take_file(),
                    src: module.take_src(),
                    span: sub_nodes[3].span(),
                });
            };
            let reg = &module.src[reg_span.clone()];
            if !reg.starts_with('r') && !reg.starts_with('R') {
                return Err(Error::InvalidRegister {
                    file: module.take_file(),
                    src: module.take_src(),
                    span: reg_span.clone(),
                });
            }
            let Ok(reg) = reg[1..].parse::<usize>() else {
                return Err(Error::InvalidRegister {
                    file: module.take_file(),
                    src: module.take_src(),
                    span: reg_span.clone(),
                });
            };
            if reg > 31 {
                return Err(Error::InvalidRegister {
                    file: module.take_file(),
                    src: module.take_src(),
                    span: reg_span.clone(),
                });
            }
            let reg = Reg::new(reg as u8);
            let pos = binary_func.alloc_coord();
            binary_func.ops.push(LowOp::PutCoord { coord: pos });
            let insn = match cond {
                "=" => LowOp::BranchCoordEqual { coord: pos, reg },
                "!=" => LowOp::BranchCoordNonEqual { coord: pos, reg },
                "<" => LowOp::BranchCoordLess { coord: pos, reg },
                ">" => LowOp::BranchCoordGreater { coord: pos, reg },
                "<=" => LowOp::BranchCoordLessEqual { coord: pos, reg },
                ">=" => LowOp::BranchCoordGreaterEqual { coord: pos, reg },
                "!0" => LowOp::BranchCoordIfNonZero { coord: pos, reg },
                "=0" => LowOp::BranchCoordIfZero { coord: pos, reg },
                _ => {
                    return Err(Error::InvalidCondition {
                        file: module.take_file(),
                        src: module.take_src(),
                        span: cond_span.clone(),
                    });
                }
            };
            let expr = sub_nodes.remove(1);
            let Node::Node { span, type_, sub_nodes } = expr else {
                return Err(Error::UnexpectedToken {
                    file: module.take_file(),
                    src: module.take_src(),
                    span: expr.span(),
                });
            };
            if type_ != BracketType::Round {
                return Err(Error::InvalidBracketType {
                    file: module.take_file(),
                    src: module.take_src(),
                    span,
                });
            }
            compile_label_node(
                dialect,
                task,
                module_index,
                binary_func,
                sub_nodes,
                span,
                depth + 1,
            )?;
            binary_func.ops.push(insn);
            if depth == 0 {
                binary_func.ops.push(LowOp::Return);
            }
        }
        _ => {
            if let Some(macro_) = MACROS.get(name) {
                (*macro_)(dialect, task, module_index, binary_func, span, sub_nodes)?;
                if depth == 0 {
                    binary_func.ops.push(LowOp::Return);
                }
                return Ok(());
            };
            if let Some(insns) = INSN_MACROS.get(name) {
                let insn = insns::find(task, module_index, insns, span, &sub_nodes)?;
                module = &mut task.modules[module_index];
                name = &module.src[name_span.clone()];
                if let Some(insn) = insn {
                    insn(module, binary_func, sub_nodes);
                    if depth == 0 {
                        binary_func.ops.push(LowOp::Return);
                    }
                    return Ok(());
                }
            }
            if sub_nodes.len() == 1 {
                if let Some(func_index) = dialect.label_indices.get(name) {
                    binary_func.ops.push(LowOp::Call {
                        coord: Coord {
                            module: module_index,
                            element: *func_index,
                        },
                    });
                    if depth == 0 {
                        binary_func.ops.push(LowOp::Return);
                    }
                    return Ok(());
                }
            }
            let name = name.to_string();
            for i in dialect.imports.iter().cloned() {
                let include = &mut task.modules[i];
                if let Some(func_index) = include.dialect.as_mut().unwrap().lookup_callable(&name) {
                    binary_func.ops.push(LowOp::Call {
                        coord: Coord {
                            module: i,
                            element: func_index,
                        },
                    });
                    if depth == 0 {
                        binary_func.ops.push(LowOp::Return);
                    }
                    return Ok(());
                }
                module = &mut task.modules[module_index];
            }
            return Err(Error::UnknownFunc {
                file: module.take_file(),
                src: module.take_src(),
                span: name_span.clone(),
            });
        }
    }
    Ok(())
}
