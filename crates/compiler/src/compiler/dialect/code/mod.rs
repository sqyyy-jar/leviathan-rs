pub mod keywords;

use std::{collections::HashMap, mem};

use leviathan_ir::binary::{BinaryFunc, BinaryModule, BinaryStatic};
use phf::{phf_map, Map};

use crate::{
    compiler::{
        error::{Error, Result},
        CompileTask, Dialect, Func, FuncData, Module, Static, UncollectedModule,
    },
    parser::{BracketType, Node},
    util::{get_key_by_value, source::Span},
};

use self::keywords::{collect_fn, collect_static, collect_use};

type KeywordProc = fn(
    data: &mut CodeLanguage,
    task: &mut CompileTask,
    module_index: usize,
    main_module: bool,
    span: Span,
    nodes: Vec<Node>,
) -> Result<()>;

const KEYWORDS: Map<&'static str, KeywordProc> = phf_map! {
    "use" => collect_use,
    "static" => collect_static,
    "fn" => collect_fn,
    "fn!" => collect_fn,
};

pub struct CodeLanguage {
    pub unresolved_imports: Vec<Span>,
    pub imports: Vec<usize>,
    pub func_indices: HashMap<String, usize>,
    pub funcs: Vec<Func>,
    pub static_indices: HashMap<String, usize>,
    pub statics: Vec<Static>,
}

impl Default for CodeLanguage {
    fn default() -> Self {
        Self {
            unresolved_imports: Vec::with_capacity(0),
            imports: Vec::with_capacity(0),
            func_indices: HashMap::with_capacity(0),
            funcs: Vec::with_capacity(0),
            static_indices: HashMap::with_capacity(0),
            statics: Vec::with_capacity(0),
        }
    }
}

impl Dialect for CodeLanguage {
    fn collect(
        &mut self,
        task: &mut CompileTask,
        module_index: usize,
        UncollectedModule { root }: UncollectedModule,
        main: bool,
    ) -> Result<()> {
        let mut module = &mut task.modules[module_index];
        for stmnt in root.into_iter().skip(1) {
            let Node::Node {
                span: stmnt_span,
                type_: BracketType::Round,
                sub_nodes: stmnt_nodes,
            } = stmnt else
            {
                return Err(Error::UnexpectedToken {
                    file: module.take_file(),
                    src: module.take_src(),
                    span: stmnt.span(),
                });
            };
            if stmnt_nodes.is_empty() {
                return Err(Error::EmptyNode {
                    file: module.take_file(),
                    src: module.take_src(),
                    span: stmnt_span,
                });
            }
            let Node::Ident { span: keyword_span } = &stmnt_nodes[0] else {
                return Err(Error::UnexpectedToken {
                    file: module.take_file(),
                    src: module.take_src(),
                    span: stmnt_nodes[0].span(),
                });
            };
            let keyword = &module.src[keyword_span.clone()];
            let Some(keyword_proc) = KEYWORDS.get(keyword) else {
                return Err(Error::InvalidKeyword {
                    file: module.take_file(),
                    src: module.take_src(),
                    span: keyword_span.clone(),
                });
            };
            (*keyword_proc)(self, task, module_index, main, stmnt_span, stmnt_nodes)?;
            module = &mut task.modules[module_index];
        }
        Ok(())
    }

    fn compile_module(
        &mut self,
        task: &mut CompileTask,
        module_index: usize,
    ) -> Result<BinaryModule> {
        let mut binary_mod = BinaryModule::default();
        if task.collect_offsets {
            let name = get_key_by_value(&task.module_indices, &module_index);
            binary_mod.name = name.cloned();
        }
        let module = &mut task.modules[module_index];
        for import_span in self.unresolved_imports.drain(..) {
            let import = &module.src[import_span.clone()];
            let Some(import) = task.module_indices.get(import) else {
                return Err(Error::UnknownModule {
                    file: module.take_file(),
                    src: module.take_src(),
                    span: import_span,
                });
            };
            if *import == module_index {
                return Err(Error::SelfImport {
                    file: module.take_file(),
                    src: module.take_src(),
                    span: import_span,
                });
            }
            if self.imports.contains(import) {
                return Err(Error::DuplicateImport {
                    file: module.take_file(),
                    src: module.take_src(),
                    span: import_span,
                });
            }
            self.imports.push(*import);
        }
        self.unresolved_imports.shrink_to_fit();
        for i in 0..self.statics.len() {
            let name = if task.collect_offsets {
                get_key_by_value(&self.static_indices, &i).cloned()
            } else {
                None
            };
            let static_ = mem::take(&mut self.statics[i]);
            let value = compile_static(module, static_.node, name)?;
            binary_mod.statics.insert(i, value);
        }
        for i in 0..self.funcs.len() {
            let Func { data, .. } = &mut self.funcs[i];
            let FuncData { node } = mem::take(data);
            let value = compile_func(task, module_index, node)?;
            binary_mod.funcs.insert(i, value);
        }
        Ok(binary_mod)
    }

    fn lookup_callable(&self, name: &str) -> Option<usize> {
        let Some(index) = self.func_indices.get(name).cloned() else {
            return None;
        };
        if !self.funcs[index].public {
            return None;
        }
        Some(index)
    }
}

fn compile_static(module: &mut Module, node: Node, name: Option<String>) -> Result<BinaryStatic> {
    match node {
        Node::Ident { span } => Err(Error::UnexpectedToken {
            file: module.take_file(),
            src: module.take_src(),
            span,
        }),
        Node::Int { value, .. } => Ok(BinaryStatic::Int { name, value }),
        Node::UInt { value, .. } => Ok(BinaryStatic::UInt { name, value }),
        Node::Float { value, .. } => Ok(BinaryStatic::Float { name, value }),
        Node::String { value, .. } => Ok(BinaryStatic::String { name, value }),
        Node::Node {
            span,
            type_: BracketType::Curly,
            mut sub_nodes,
        } => match sub_nodes.len() {
            0 => Err(Error::EmptyBuffer {
                file: module.take_file(),
                src: module.take_src(),
                span,
            }),
            1 => {
                let length_node = sub_nodes.pop().unwrap();
                let length = expect_unsigned_num(module, length_node)?;
                Ok(BinaryStatic::FilledBuffer {
                    name,
                    size: length as usize,
                    fill: 0,
                })
            }
            2 => {
                let length = expect_unsigned_num(module, sub_nodes.pop().unwrap())?;
                let fill = expect_byte(module, sub_nodes.pop().unwrap())?;
                Ok(BinaryStatic::FilledBuffer {
                    name,
                    size: length as usize,
                    fill,
                })
            }
            3 => Err(Error::UnexpectedToken {
                file: module.take_file(),
                src: module.take_src(),
                span: sub_nodes[2].span(),
            }),
            _ => Err(Error::UnexpectedTokens {
                file: module.take_file(),
                src: module.take_src(),
                span: sub_nodes[2].span().start..sub_nodes.last().unwrap().span().end,
            }),
        },
        Node::Node {
            span,
            type_: BracketType::Square,
            mut sub_nodes,
        } => {
            if sub_nodes.is_empty() {
                return Err(Error::EmptyArray {
                    file: module.take_file(),
                    src: module.take_src(),
                    span,
                });
            }
            match &sub_nodes[0] {
                Node::Int { .. } => {
                    let mut values = Vec::with_capacity(sub_nodes.len());
                    for sub_node in sub_nodes.drain(..) {
                        values.push(expect_signed_num(module, sub_node)?);
                    }
                    Ok(BinaryStatic::IntArray { name, values })
                }
                Node::UInt { .. } => {
                    let mut values = Vec::with_capacity(sub_nodes.len());
                    for sub_node in sub_nodes.drain(..) {
                        values.push(expect_unsigned_num(module, sub_node)?);
                    }
                    Ok(BinaryStatic::UIntArray { name, values })
                }
                Node::Float { .. } => {
                    let mut values = Vec::with_capacity(sub_nodes.len());
                    for sub_node in sub_nodes.drain(..) {
                        values.push(expect_float(module, sub_node)?);
                    }
                    Ok(BinaryStatic::FloatArray { name, values })
                }
                _ => Err(Error::UnexpectedToken {
                    file: module.take_file(),
                    src: module.take_src(),
                    span: sub_nodes[0].span(),
                }),
            }
        }
        _ => Err(Error::UnexpectedToken {
            file: module.take_file(),
            src: module.take_src(),
            span: node.span(),
        }),
    }
}

fn compile_func(_task: &mut CompileTask, _module_index: usize, node: Node) -> Result<BinaryFunc> {
    match node {
        Node::Node { span, type_: BracketType::Curly, sub_nodes } => {
            todo!("Parse statement")
        }
        _ => {
            todo!("Parse expression")
        }
    }
}

pub fn expect_byte(module: &mut Module, node: Node) -> Result<u8> {
    match node {
        Node::Int { span, value } => {
            if (-128..=255).contains(&value) {
                return Ok(value as u8);
            }
            Err(Error::InvalidByte {
                file: module.take_file(),
                src: module.take_src(),
                span,
            })
        }
        Node::UInt { span, value } => {
            if (0..=255).contains(&value) {
                return Ok(value as u8);
            }
            Err(Error::InvalidByte {
                file: module.take_file(),
                src: module.take_src(),
                span,
            })
        }
        _ => Err(Error::UnexpectedToken {
            file: module.take_file(),
            src: module.take_src(),
            span: node.span(),
        }),
    }
}

pub fn expect_unsigned_num(module: &mut Module, node: Node) -> Result<u64> {
    match node {
        Node::Int { span, value } => {
            if value < 0 {
                return Err(Error::NegativeNumber {
                    file: module.take_file(),
                    src: module.take_src(),
                    span,
                });
            }
            Ok(value as u64)
        }
        Node::UInt { value, .. } => Ok(value),
        _ => Err(Error::UnexpectedToken {
            file: module.take_file(),
            src: module.take_src(),
            span: node.span(),
        }),
    }
}

pub fn expect_signed_num(module: &mut Module, node: Node) -> Result<i64> {
    match node {
        Node::Int { value, .. } => Ok(value),
        Node::UInt { span, value } => {
            if value > i64::MAX as u64 {
                return Err(Error::OversizedNumber {
                    file: module.take_file(),
                    src: module.take_src(),
                    span,
                });
            }
            Ok(value as i64)
        }
        _ => Err(Error::UnexpectedToken {
            file: module.take_file(),
            src: module.take_src(),
            span: node.span(),
        }),
    }
}

pub fn expect_float(module: &mut Module, node: Node) -> Result<f64> {
    if let Node::Float { value, .. } = node {
        return Ok(value);
    }
    Err(Error::UnexpectedToken {
        file: module.take_file(),
        src: module.take_src(),
        span: node.span(),
    })
}
