pub mod keywords;

use std::mem;

use phf::{phf_map, Map};

use crate::{
    compiler::{
        cast,
        error::{Error, Result},
        CompileTask, Module, ModuleType, ModuleVTable, UncollectedModule,
    },
    parser::Node,
    util::source::Span,
};

use self::keywords::{r#fn, r#static, r#use};

type KeywordProc = fn(
    task: &mut CompileTask,
    module_index: usize,
    main_module: bool,
    span: Span,
    nodes: Vec<Node>,
) -> Result<()>;

const KEYWORDS: Map<&'static str, KeywordProc> = phf_map! {
    "use" => r#use,
    "static" => r#static,
    "fn" => r#fn,
};

pub enum CodeLanguage {
    Collected { imports: Vec<Span> },
    Intermediary { imports: Vec<usize> },
}

impl Default for CodeLanguage {
    fn default() -> Self {
        Self::Collected {
            imports: Vec::with_capacity(0),
        }
    }
}

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

fn gen_intermediary(task: &mut CompileTask, module_index: usize) -> Result<()> {
    let module = &mut task.modules[module_index];
    let CodeLanguage::Collected { imports } = cast::<CodeLanguage>(&mut module.type_).as_mut() else {
        unreachable!()
    };
    let imports = mem::replace(imports, Vec::with_capacity(0));
    let mut new_imports = Vec::with_capacity(imports.len());
    for import_span in imports {
        let import = &module.src[import_span.clone()];
        let Some(import) = task.module_indices.get(import) else {
            return Err(Error::UnknownModule {
                file: take_file(module),
                src: take_src(module),
                span: import_span,
            });
        };
        new_imports.push(*import);
    }
    module.type_ = Box::new(CodeLanguage::Intermediary {
        imports: new_imports,
    });
    Ok(())
}

fn take_file(module: &mut Module) -> String {
    mem::replace(&mut module.file, String::with_capacity(0))
}

fn take_src(module: &mut Module) -> String {
    mem::replace(&mut module.src, String::with_capacity(0))
}
