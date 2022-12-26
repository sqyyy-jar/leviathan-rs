use crate::{grouper::Group, tokenizer::Token};
use std::ops::Range;

#[derive(Debug)]
pub enum Error {}

pub struct Ast<'a> {
    namespace: AstNamespace<'a>,
    functions: Vec<AstFunction<'a>>,
}

pub struct AstNamespace<'a>(Vec<&'a str>);

pub struct AstFunction<'a> {
    range: Range<usize>,
    name: &'a Token,
    parameters: &'a Group,
    tags: Option<&'a Group>,
    code: Option<AstCode<'a>>,
}

pub enum AstCode<'a> {
    Token(&'a Token),
    Group(&'a Group),
}

struct State<'a> {
    source: &'a str,
    tokens: &'a Vec<Token>,
    root_group: &'a Group,
}

pub fn build_ast(source: &str, tokens: &Vec<Token>, root_group: &Group) -> Result<(), Error> {
    let mut state = State {
        source,
        tokens,
        root_group,
    };
    todo!()
}
