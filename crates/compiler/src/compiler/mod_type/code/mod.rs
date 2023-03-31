pub mod keywords;

use std::{collections::HashMap, mem};

use phf::{phf_map, Map};

use crate::{
    compiler::{
        error::{Error, Result},
        intermediary::{Insn, IntermediaryStaticValue},
        CompileTask, Dialect, Func, FuncData, Module_, Static, StaticData, UncollectedModule,
    },
    parser::{BracketType, Node},
    util::source::Span,
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

    fn compile_module(&mut self, task: &mut CompileTask, module_index: usize) -> Result<()> {
        let module = &mut task.modules[module_index];
        let imports = mem::replace(&mut self.imports, Vec::with_capacity(0));
        let mut new_imports = Vec::with_capacity(imports.len());
        for import_span in self.unresolved_imports.drain(..) {
            let import = &module.src[import_span.clone()];
            let Some(import) = task.module_indices.get(import) else {
                return Err(Error::UnknownModule {
                    file: module.take_file(),
                    src: module.take_src(),
                    span: import_span,
                });
            };
            new_imports.push(*import);
        }
        for i in 0..self.statics.len() {
            let static_ = &mut self.statics[i];
            let StaticData::Collected { node } = mem::replace(
                &mut static_.data,
                StaticData::Intermediary {
                    value: IntermediaryStaticValue::Int(0),
                },
            ) else {
                unreachable!()
            };
            let value = compile_static(module, node)?;
            self.statics[i].data = StaticData::Intermediary { value };
        }
        for i in 0..self.funcs.len() {
            let Func { data, .. } = &mut self.funcs[i];
            let FuncData::Collected { node } = mem::replace(
                data,
                FuncData::Intermediary {
                    ir: Vec::with_capacity(0),
                },
            ) else {unreachable!()};
            let ir = compile_func_body(task, module_index, node)?;
            self.funcs[i].data = FuncData::Intermediary { ir };
        }
        Ok(())
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

fn compile_static(module: &mut Module_, node: Node) -> Result<IntermediaryStaticValue> {
    match node {
        Node::Ident { span } => Err(Error::UnexpectedToken {
            file: module.take_file(),
            src: module.take_src(),
            span,
        }),
        Node::Int { value, .. } => Ok(IntermediaryStaticValue::Int(value)),
        Node::UInt { value, .. } => Ok(IntermediaryStaticValue::UInt(value)),
        Node::Float { value, .. } => Ok(IntermediaryStaticValue::Float(value)),
        Node::String { value, .. } => Ok(IntermediaryStaticValue::String(value)),
        Node::Node {
            span: _,
            type_: _,
            sub_nodes: _,
        } => todo!(),
    }
}

fn compile_func_body(
    _task: &mut CompileTask,
    _module_index: usize,
    _expr: Node,
) -> Result<Vec<Insn>> {
    todo!()
}
