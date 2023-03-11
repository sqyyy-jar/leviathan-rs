use crate::util::source::Span;

pub mod ast;
pub mod tokenizer;

pub enum Token {
    LeftParen { span: Span },
    RightParen { span: Span },
    Ident { span: Span },
    Int { span: Span, value: i64 },
    UInt { span: Span, value: u64 },
    Float { span: Span, value: f64 },
    String { span: Span, value: Span },
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
