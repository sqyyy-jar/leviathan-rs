use std::collections::HashMap;

use crate::{
    grouper::{Group, GroupElement, GroupType},
    tokenizer::{Token, TokenType},
};

const MAX_TYPE_RECURSION: usize = 8;

#[derive(Debug)]
pub enum Error {
    NonRootGroup(),
    NoNamespaceFound(),
    InvalidNamespace(),
    InvalidFunction(),
    InvalidFunctionName(),
    InvalidFunctionTags(),
    InvalidFunctionTag(),
    InvalidFunctionParams(),
    InvalidFunctionParam(),
    InvalidFunctionParamType(),
    InvalidFunctionCode(),
    InvalidStringLiteral(),
    RawTokensAtRootLevel(),
    InvalidGroupTypeAtRootLevel(),
    InvalidGroupAtRootLevel(),
    EmptyGroupAtRootLevel(),
    TooHighTypeRecursion(),
    UnexpectedEndOfTokens(),
    UnexpectedEndOfSource(),
}

#[derive(Debug)]
pub struct CompileResult {
    pub namespace: String,
    pub functions: Vec<CompiledFunction>,
}

#[derive(Debug)]
pub struct CompiledFunction {
    pub name: String,
    pub tags: Vec<String>,
    pub parameters: Vec<(String, CompiledType)>,
    pub code: Option<CodeSegment>,
}

#[derive(Debug)]
pub enum CodeSegment {
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    List(Vec<CodeSegment>),
    Map(HashMap<String, CodeSegment>),
    Call {
        func: String,
        params: Vec<CodeSegment>,
    },
}

#[derive(Debug)]
pub enum CompiledType {
    Unit,
    Bool,
    Int,
    Float,
    Str,
    List(Box<CompiledType>),
    Map(Box<CompiledType>),
}

struct State<'a> {
    source: &'a str,
    tokens: &'a Vec<Token>,
    ast: &'a Group,
}

pub fn compile(source: &str, tokens: &Vec<Token>, ast: &Group) -> Result<CompileResult, Error> {
    let state = State {
        source,
        tokens,
        ast,
    };
    if ast.group_type != GroupType::Root {
        return Err(Error::NonRootGroup());
    }
    let namespace = compile_namespace(&state)?;
    let mut functions = Vec::with_capacity(0);
    for element in ast.elements.iter().skip(1) {
        let GroupElement::Group(group) = element else {
            return Err(Error::RawTokensAtRootLevel());
        };
        if group.group_type != GroupType::Round {
            return Err(Error::InvalidGroupTypeAtRootLevel());
        }
        if group.elements.is_empty() {
            return Err(Error::EmptyGroupAtRootLevel());
        }
        let GroupElement::TokenRange(range) = &group.elements[0] else {
            return Err(Error::InvalidGroupAtRootLevel());
        };
        let Some(first_token) = tokens.get(range.start)  else{
            return Err(Error::UnexpectedEndOfTokens());
        };
        match first_token.token_type {
            TokenType::Fn => {
                functions.push(compile_function(&state, group)?);
            }
            _ => {
                return Err(Error::InvalidGroupAtRootLevel());
            }
        }
    }
    Ok(CompileResult {
        namespace,
        functions,
    })
}

fn compile_namespace(state: &State) -> Result<String, Error> {
    if state.ast.elements.is_empty() {
        return Err(Error::NoNamespaceFound());
    }
    let GroupElement::Group(namespace_group) = &state.ast.elements[0] else {
        return Err(Error::NoNamespaceFound());
    };
    if namespace_group.group_type != GroupType::Round
        || namespace_group.elements.len() != 1
        || namespace_group.range.len() != 4
    {
        return Err(Error::InvalidNamespace());
    }
    let GroupElement::TokenRange(namespace_range) = &namespace_group.elements[0] else {
        return Err(Error::InvalidNamespace());
    };
    if namespace_range.len() != 2 {
        return Err(Error::InvalidNamespace());
    }
    let Some(namespace_keyword) = state.tokens.get(namespace_range.start) else {
        return Err(Error::UnexpectedEndOfTokens());
    };
    if namespace_keyword.token_type != TokenType::Ns {
        return Err(Error::InvalidNamespace());
    }
    let Some(namespace_token) = state.tokens.get(namespace_range.start + 1) else {
        return Err(Error::UnexpectedEndOfTokens());
    };
    if namespace_token.token_type != TokenType::Ident {
        return Err(Error::InvalidNamespace());
    }
    Ok(state.source[namespace_token.chars.clone()].to_string())
}

fn compile_function(state: &State, group: &Group) -> Result<CompiledFunction, Error> {
    if group.elements.len() < 3 || group.elements.len() > 4 {
        return Err(Error::InvalidFunction());
    }
    let name = compile_function_name(state, group)?;
    let GroupElement::Group(first_group) = &group.elements[1] else {
        return Err(Error::InvalidFunction());
    };
    let mut tags = Vec::with_capacity(0);
    let index = if first_group.group_type == GroupType::Square {
        compile_function_tags(state, &mut tags, first_group)?;
        2
    } else {
        1
    };
    let GroupElement::Group(params) = &group.elements[index] else {
        return Err(Error::InvalidFunctionParams());
    };
    if params.group_type != GroupType::Curly {
        return Err(Error::InvalidFunctionParams());
    }
    let mut parameters = Vec::with_capacity(0);
    compile_params(state, params, &mut parameters)?;
    if index == 1 {
        if group.elements.len() != 3 {
            return Err(Error::InvalidFunction());
        }
    } else if group.elements.len() != 4 {
        return Err(Error::InvalidFunction());
    }
    let Some(code) = group.elements.get(index + 1) else {
        return Ok(CompiledFunction { name, tags, parameters, code: None });
    };
    match code {
        GroupElement::Group(group) => {
            match group.group_type {
                GroupType::Round => {
                    todo!()
                }
                _ => {
                    unreachable!()
                }
            }
            todo!()
        }
        GroupElement::TokenRange(range) => {
            if range.len() != 1 {
                return Err(Error::InvalidFunctionCode());
            }
            let Some(return_code) = state.tokens.get(range.start) else {
                return Err(Error::InvalidFunctionCode());
            };
            match return_code.token_type {
                TokenType::True => Ok(CompiledFunction {
                    name,
                    tags,
                    parameters,
                    code: Some(CodeSegment::Bool(true)),
                }),
                TokenType::False => Ok(CompiledFunction {
                    name,
                    tags,
                    parameters,
                    code: Some(CodeSegment::Bool(false)),
                }),
                TokenType::Int => Ok(CompiledFunction {
                    name,
                    tags,
                    parameters,
                    code: Some(CodeSegment::Int(
                        state.source[return_code.chars.clone()].parse().unwrap(),
                    )),
                }),
                TokenType::Float => Ok(CompiledFunction {
                    name,
                    tags,
                    parameters,
                    code: Some(CodeSegment::Float(
                        state.source[return_code.chars.clone()].parse().unwrap(),
                    )),
                }),
                TokenType::String => {
                    let mut buf = String::new();
                    let mut chars = state.source
                        [return_code.chars.start + 1..return_code.chars.end - 1]
                        .chars();
                    while let Some(c) = chars.next() {
                        if c == '\\' {
                            let Some(next) = chars.next() else {
                                return Err(Error::UnexpectedEndOfTokens());
                            };
                            match next {
                                '\\' => buf.push('\\'),
                                'n' => buf.push('\n'),
                                'r' => buf.push('\r'),
                                't' => buf.push('\t'),
                                '"' => buf.push('"'),
                                _ => return Err(Error::InvalidStringLiteral()),
                            }
                            continue;
                        }
                        buf.push(c);
                    }
                    Ok(CompiledFunction {
                        name,
                        tags,
                        parameters,
                        code: Some(CodeSegment::String(buf)),
                    })
                }
                _ => Err(Error::InvalidFunctionCode()),
            }
        }
    }
}

fn compile_function_name(state: &State, group: &Group) -> Result<String, Error> {
    let GroupElement::TokenRange(range) = &group.elements[0] else {
        return Err(Error::InvalidFunction());
    };
    if range.len() != 2 {
        return Err(Error::InvalidFunction());
    }
    let Some(token) = state.tokens.get(range.start + 1) else {
        return Err(Error::UnexpectedEndOfTokens());
    };
    if token.token_type != TokenType::Ident {
        return Err(Error::InvalidFunctionName());
    }
    Ok(state.source[token.chars.clone()].to_string())
}

fn compile_function_tags(
    state: &State,
    tags: &mut Vec<String>,
    first_group: &Group,
) -> Result<(), Error> {
    if first_group.elements.len() != 1 {
        return Err(Error::InvalidFunctionTags());
    }
    let GroupElement::TokenRange(tags_range) = &first_group.elements[0] else {
        return Err(Error::InvalidFunctionTags());
    };
    for i in tags_range.clone() {
        let Some(token) = state.tokens.get(i) else {
            return Err(Error::UnexpectedEndOfTokens());
        };
        if token.token_type != TokenType::Atom {
            return Err(Error::InvalidFunctionTag());
        }
        tags.push(state.source[token.chars.start + 1..token.chars.end].to_string());
    }
    Ok(())
}

fn compile_params(
    state: &State,
    params: &Group,
    params_list: &mut Vec<(String, CompiledType)>,
) -> Result<(), Error> {
    let mut i = params.range.start + 1;
    let end = params.range.end - 1;
    while i < end {
        let Some(name) = state.tokens.get(i) else {
            return Err(Error::UnexpectedEndOfTokens());
        };
        if name.token_type != TokenType::Atom {
            return Err(Error::InvalidFunctionParam());
        }
        let name = state.source[name.chars.start + 1..name.chars.end].to_string();
        let mut param_type = Vec::with_capacity(1);
        i += 1;
        while i < end {
            let Some(type_component) = state.tokens.get(i) else {
                return Err(Error::UnexpectedEndOfTokens());
            };
            if type_component.token_type == TokenType::Atom {
                break;
            }
            param_type.push(type_component);
            i += 1;
        }
        params_list.push((name, compile_type(state, &param_type, 0)?));
    }
    Ok(())
}

fn compile_type(state: &State, parts: &[&Token], rec: usize) -> Result<CompiledType, Error> {
    if rec >= MAX_TYPE_RECURSION {
        return Err(Error::TooHighTypeRecursion());
    }
    if parts.is_empty() {
        return Ok(CompiledType::Unit);
    }
    if parts[0].token_type != TokenType::Ident {
        return Err(Error::InvalidFunctionParamType());
    }
    let base_type = &state.source[parts[0].chars.clone()];
    match base_type {
        "unit" => {
            if parts.len() > 1 {
                return Err(Error::InvalidFunctionParamType());
            }
            Ok(CompiledType::Unit)
        }
        "bool" => {
            if parts.len() > 1 {
                return Err(Error::InvalidFunctionParamType());
            }
            Ok(CompiledType::Bool)
        }
        "int" => {
            if parts.len() > 1 {
                return Err(Error::InvalidFunctionParamType());
            }
            Ok(CompiledType::Int)
        }
        "float" => {
            if parts.len() > 1 {
                return Err(Error::InvalidFunctionParamType());
            }
            Ok(CompiledType::Float)
        }
        "str" => {
            if parts.len() > 1 {
                return Err(Error::InvalidFunctionParamType());
            }
            Ok(CompiledType::Str)
        }
        "list" => {
            if parts.len() < 4 {
                return Err(Error::InvalidFunctionParamType());
            }
            if parts[1].token_type != TokenType::LeftBracket {
                return Err(Error::InvalidFunctionParamType());
            }
            let inner = compile_type(state, &parts[2..parts.len() - 1], rec + 1)?;
            if parts[parts.len() - 1].token_type != TokenType::RightBracket {
                return Err(Error::InvalidFunctionParamType());
            }
            Ok(CompiledType::List(Box::new(inner)))
        }
        "map" => {
            if parts.len() < 4 {
                return Err(Error::InvalidFunctionParamType());
            }
            if parts[1].token_type != TokenType::LeftBracket {
                return Err(Error::InvalidFunctionParamType());
            }
            let inner = compile_type(state, &parts[2..parts.len() - 1], rec + 1)?;
            if parts[parts.len() - 1].token_type != TokenType::RightBracket {
                return Err(Error::InvalidFunctionParamType());
            }
            Ok(CompiledType::Map(Box::new(inner)))
        }
        _ => Err(Error::InvalidFunctionParamType()),
    }
}
