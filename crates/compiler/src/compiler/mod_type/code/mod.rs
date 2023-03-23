use std::mem;

use phf::{phf_map, Map};

use crate::{
    compiler::{
        error::{Error, Result},
        CompileTask, Module, ModuleType, ModuleVTable, UncollectedModule,
    },
    parser::Node,
    util::source::Span,
};

type KeywordProc = fn(
    task: &mut CompileTask,
    module_index: usize,
    main_module: bool,
    span: Span,
    nodes: Vec<Node>,
) -> Result<()>;

const KEYWORDS: Map<&'static str, KeywordProc> = phf_map! {
    "use" => |_, _, _, _, _| {todo!()},
    "static" => |_, _, _, _, _| {todo!()},
    "fn" => |_, _, _, _, _| {todo!()},
};

pub struct CodeLanguage;

impl ModuleType for CodeLanguage {
    fn vtable(&self) -> ModuleVTable {
        ModuleVTable {
            collect,
            gen_intermediary,
        }
    }
}

fn collect(
    task: &mut CompileTask,
    module_index: usize,
    UncollectedModule { root }: UncollectedModule,
    main_module: bool,
) -> Result<()> {
    let mut module = &mut task.modules[module_index];
    for stmnt in root {
        let Node::Node {
            span: stmnt_span,
            sub_nodes: stmnt_nodes,
        } = stmnt else
        {
            return Err(Error::UnexpectedToken {
                file: take_file(module),
                src: take_src(module),
                span: stmnt.span(),
            });
        };
        if stmnt_nodes.is_empty() {
            return Err(Error::EmptyNode {
                file: take_file(module),
                src: take_src(module),
                span: stmnt_span,
            });
        }
        let Node::Ident { span: keyword_span } = &stmnt_nodes[0] else {
            return Err(Error::UnexpectedToken {
                file: take_file(module),
                src: take_src(module),
                span: stmnt_nodes[0].span(),
            });
        };
        let keyword = &module.src[keyword_span.clone()];
        let Some(keyword_proc) = KEYWORDS.get(keyword) else {
            return Err(Error::InvalidKeyword {
                file: take_file(module),
                src: take_src(module),
                span: keyword_span.clone(),
            });
        };
        (*keyword_proc)(task, module_index, main_module, stmnt_span, stmnt_nodes)?;
        module = &mut task.modules[module_index];
    }
    Ok(())
}

fn gen_intermediary(_task: &mut CompileTask, _module_index: usize) -> Result<()> {
    todo!()
}

fn take_file(module: &mut Module) -> String {
    mem::replace(&mut module.file, String::with_capacity(0))
}

fn take_src(module: &mut Module) -> String {
    mem::replace(&mut module.src, String::with_capacity(0))
}
