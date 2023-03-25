use crate::{
    compiler::{
        cast,
        error::{Error, Result},
        CompileTask, Func, FuncData, Module, Static, StaticData, Type,
    },
    parser::{BracketType, Node},
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
    task: &mut CompileTask,
    module_index: usize,
    _main_module: bool,
    span: Span,
    nodes: Vec<Node>,
) -> Result<()> {
    let module = &mut task.modules[module_index];
    let node_count = nodes.len();
    if !(4..=5).contains(&node_count) {
        return Err(Error::InvalidStatement {
            file: take_file(module),
            src: take_src(module),
            span,
        });
    }
    let mut nodes = nodes.into_iter();
    let Node::Ident { span: keyword_span } = nodes.next().unwrap() else {
        unreachable!()
    };
    let public = module.src[keyword_span].ends_with('!');
    let name_node = nodes.next().unwrap();
    let Node::Ident { span: name_span } = name_node else {
        return Err(Error::UnexpectedToken {
            file: take_file(module),
            src: take_src(module),
            span: name_node.span(),
        });
    };
    let params_node = nodes.next().unwrap();
    let Node::Node {
        span: params_span,
        type_: BracketType::Square,
        sub_nodes: param_nodes,
    } = params_node else
    {
        return Err(Error::UnexpectedToken {
            file: take_file(module),
            src: take_src(module),
            span: params_node.span(),
        });
    };
    let param_count = param_nodes.len();
    if param_count % 2 != 0 {
        return Err(Error::InvalidParams {
            file: take_file(module),
            src: take_src(module),
            span: params_span,
        });
    }
    let mut param_nodes = param_nodes.into_iter();
    let mut params = Vec::with_capacity(param_count / 2);
    for _ in 0..param_count / 2 {
        let param_name_node = param_nodes.next().unwrap();
        let Node::Ident { span: param_name_span } = param_name_node else {
            return Err(Error::InvalidParams {
                file: take_file(module),
                src: take_src(module),
                span: param_name_node.span(),
            });
        };
        let Some(param_name) = &module.src[param_name_span.clone()].strip_prefix(':') else {
            return Err(Error::InvalidParams {
                file: take_file(module),
                src: take_src(module),
                span: param_name_span,
            });
        };
        let param_name = param_name.to_string();
        let param_type_node = param_nodes.next().unwrap();
        let Node::Ident { span: param_type_span } = param_type_node else {
            return Err(Error::InvalidParams {
                file: take_file(module),
                src: take_src(module),
                span: param_type_node.span(),
            });
        };
        let param_type = parse_type(module, param_type_span)?;
        params.push((Some(param_name), param_type));
    }
    let return_ = if node_count == 5 {
        let return_node = nodes.next().unwrap();
        let Node::Ident { span: return_span } = return_node else {
            return Err(Error::UnexpectedToken {
                file: take_file(module),
                src: take_src(module),
                span: return_node.span(),
            });
        };
        parse_type(module, return_span)?
    } else {
        Type::Unit
    };
    let expr_node = nodes.next().unwrap();
    module.funcs.push(Func {
        public,
        params,
        return_,
        data: FuncData::Collected { node: expr_node },
        used: false,
    });
    todo!()
}

fn parse_type(module: &mut Module, span: Span) -> Result<Type> {
    match &module.src[span.clone()] {
        "unit" => Ok(Type::Unit),
        "int" => Ok(Type::Int),
        "uint" => Ok(Type::UInt),
        "float" => Ok(Type::Float),
        "str" => Ok(Type::String),
        "ptr" => todo!(),
        _ => Err(Error::InvalidType {
            file: take_file(module),
            src: take_src(module),
            span,
        }),
    }
}
