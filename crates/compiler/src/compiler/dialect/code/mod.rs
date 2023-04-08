pub mod keywords;

use std::{collections::HashMap, mem};

use leviathan_ir::binary::{BinaryModule, BinaryStatic};
use phf::{phf_map, Map};

use crate::{
    compiler::{
        error::{Error, Result},
        CompileTask, Dialect, Func, FuncData, Module, Static, StaticData, UncollectedModule,
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
            let name = if task.collect_offsets {
                get_key_by_value(&self.static_indices, &i).cloned()
            } else {
                None
            };
            let static_ = &mut self.statics[i];
            let StaticData { node } = mem::take(&mut static_.data);
            let value = compile_static(module, node, name)?;
            binary_mod.statics.insert(i, value);
        }
        for i in 0..self.funcs.len() {
            let Func { data, .. } = &mut self.funcs[i];
            let FuncData { node } = mem::take(data);
            compile_func_body(task, module_index, node)?;
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
            span: _,
            type_: _,
            sub_nodes: _,
        } => todo!(),
        _ => unreachable!(),
    }
}

fn compile_func_body(_task: &mut CompileTask, _module_index: usize, _expr: Node) -> Result<()> {
    todo!()
}
