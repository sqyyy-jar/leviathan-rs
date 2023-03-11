use crate::util::source::Span;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IdentStartingWithDigit { span: Span },
    NoWhitespaceBetweenTokens { span: Span },
    UnexpectedEndOfSource { span: Span },
    InvalidStringEscapeCode { span: Span },
}
