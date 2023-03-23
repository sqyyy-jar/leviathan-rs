use crate::{
    compiler::{
        cast,
        error::{Error, Result},
        CompileTask,
    },
    parser::Node,
    util::source::Span,
};

use super::{take_file, take_src, CodeLanguage};

pub fn r#use(
    task: &mut CompileTask,
    module_index: usize,
    _main_module: bool,
    span: Span,
    nodes: Vec<Node>,
) -> Result<()> {
    let module = &mut task.modules[module_index];
    if nodes.len() < 2 {
        return Err(Error::InvalidStatement {
            file: take_file(module),
            src: take_src(module),
            span,
        });
    }
    for node in nodes.into_iter().skip(1) {
        let Node::Ident { span: include_span } = &node else {
            return Err(Error::UnexpectedToken {
                file: take_file(module),
                src: take_src(module),
                span: node.span(),
            });
        };
        let include = &module.src[include_span.clone()];
        let CodeLanguage::Collected { imports } =
            cast::<CodeLanguage>(&mut module.type_).as_mut() else
        {unreachable!()};
        for import_span in imports.iter() {
            let import = &module.src[import_span.clone()];
            if include == import {
                return Err(Error::UnexpectedToken {
                    file: take_file(module),
                    src: take_src(module),
                    span: include_span.clone(),
                });
            }
        }
        imports.push(include_span.clone());
    }
    Ok(())
}

pub fn r#static(
    _task: &mut CompileTask,
    _module_index: usize,
    _main_module: bool,
    _span: Span,
    _nodes: Vec<Node>,
) -> Result<()> {
    todo!()
}

pub fn r#fn(
    _task: &mut CompileTask,
    _module_index: usize,
    _main_module: bool,
    _span: Span,
    _nodes: Vec<Node>,
) -> Result<()> {
    todo!()
}
