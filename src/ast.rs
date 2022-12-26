use crate::{
    grouper::{Group, GroupElement, GroupType},
    tokenizer::Token,
};
use std::ops::Range;

#[derive(Debug)]
pub enum Error {
    EmptySource(),
    InvalidRootToken(),
    InvalidNamespaceGroup(),
}

pub struct Ast<'a> {
    pub namespace: AstNamespace<'a>,
    pub functions: Vec<AstFunction<'a>>,
}

pub struct AstNamespace<'a>(Vec<&'a str>);

pub struct AstFunction<'a> {
    pub range: Range<usize>,
    pub name: &'a Token,
    pub tags: Option<&'a Group>,
    pub parameters: &'a Group,
    pub code: Option<AstCode<'a>>,
}

pub enum AstCode<'a> {
    Token(&'a Token),
    Group(&'a Group),
}

struct State<'a> {
    source: &'a str,
    tokens: &'a Vec<Token>,
    root_group: &'a Group,
    group_index: usize,
}

impl State<'_> {}

pub fn build_ast(source: &str, tokens: &Vec<Token>, root_group: &Group) -> Result<(), Error> {
    let mut state = State {
        source,
        tokens,
        root_group,
        group_index: 0,
    };
    if state.root_group.elements.len() < 1 {
        return Err(Error::EmptySource());
    }
    let mut iter = state.root_group.elements.iter();
    let Some(namespace_group) = iter.next() else {
        unreachable!()
    };
    let GroupElement::Group(namespace_group) = namespace_group else {
        return Err(Error::InvalidNamespaceGroup());
    };
    if namespace_group.group_type != GroupType::Round {
        return Err(Error::InvalidNamespaceGroup());
    }
    if namespace_group.elements.len() != 2 {
        return Err(Error::InvalidNamespaceGroup());
    }
    for group in iter {
        let GroupElement::Group(group) = group else {
            return Err(Error::InvalidRootToken());
        };
        if group.group_type != GroupType::Round {
            return Err(Error::InvalidRootToken());
        }
        todo!()
    }
    todo!()
}
