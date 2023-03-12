use crate::util::source::Span;

pub mod ast;
pub mod error;
pub mod tokenizer;

#[derive(Debug)]
pub struct TokenList {
    pub src: String,
    pub tokens: Vec<Token>,
}

#[derive(Debug)]
pub struct BareModule {
    pub name: String,
    pub src: String,
    pub root: Vec<Node>,
}

#[derive(Debug)]
pub enum Token {
    LeftParen { span: Span },
    RightParen { span: Span },
    Ident { span: Span },
    Int { span: Span, value: i64 },
    UInt { span: Span, value: u64 },
    Float { span: Span, value: f64 },
    String { span: Span, value: String },
}

impl Token {
    pub fn span(&self) -> Span {
        match self {
            Token::LeftParen { span }
            | Token::RightParen { span }
            | Token::Ident { span }
            | Token::Int { span, .. }
            | Token::UInt { span, .. }
            | Token::Float { span, .. }
            | Token::String { span, .. } => span.clone(),
        }
    }
}

#[derive(Debug)]
pub enum Node {
    Ident { span: Span },
    Int { span: Span, value: i64 },
    UInt { span: Span, value: u64 },
    Float { span: Span, value: f64 },
    String { span: Span, value: String },
    Node { span: Span, sub_nodes: Vec<Node> },
}

impl Node {
    pub fn span(&self) -> Span {
        match self {
            Node::Ident { span }
            | Node::Int { span, .. }
            | Node::UInt { span, .. }
            | Node::Float { span, .. }
            | Node::String { span, .. }
            | Node::Node { span, .. } => span.clone(),
        }
    }
}
