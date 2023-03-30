use crate::Span;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidBinOp { left: Span, right: Span },
    InvalidCast { span: Span },
    InvalidBitNot { span: Span },
}
