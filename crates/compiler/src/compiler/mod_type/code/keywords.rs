use crate::{
    compiler::{
        cast,
        error::{Error, Result},
        CompileTask, Static, StaticData,
    },
    parser::Node,
    util::source::Span,
};

use super::{take_file, take_src, CodeLanguage};

pub fn collect_use(
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
        let CodeLanguage::Collected { imports, .. } =
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

pub fn collect_static(
    task: &mut CompileTask,
    module_index: usize,
    _main_module: bool,
    span: Span,
    nodes: Vec<Node>,
) -> Result<()> {
    let module = &mut task.modules[module_index];
    if nodes.len() == 1 {
        return Err(Error::InvalidStatement {
            file: take_file(module),
            src: take_src(module),
            span,
        });
    }
    if nodes.len() % 2 != 1 {
        return Err(Error::InvalidStatement {
            file: take_file(module),
            src: take_src(module),
            span,
        });
    }
    let var_count = (nodes.len() - 1) / 2;
    let mut nodes = nodes.into_iter().skip(1);
    let CodeLanguage::Collected { .. } = cast::<CodeLanguage>(&mut module.type_).as_mut() else {
        unreachable!()
    };
    for _ in 0..var_count {
        let name = nodes.next().unwrap();
        let Node::Ident { span: name_span } = name else {
            return Err(Error::UnexpectedToken {
                file: take_file(module),
                src: take_src(module),
                span: name.span(),
            });
        };
        let name = &module.src[name_span.clone()];
        if module.static_indices.contains_key(name) {
            return Err(Error::DuplicateName {
                file: take_file(module),
                src: take_src(module),
                span: name_span,
            });
        }
        let value = nodes.next().unwrap();
        module.statics.push(Static {
            data: StaticData::Collected { node: value },
            used: false,
        });
        module
            .static_indices
            .insert(name.to_string(), module.statics.len() - 1);
    }
    Ok(())
}

pub fn collect_fn(
    _task: &mut CompileTask,
    _module_index: usize,
    _main_module: bool,
    _span: Span,
    _nodes: Vec<Node>,
) -> Result<()> {
    todo!()
}
