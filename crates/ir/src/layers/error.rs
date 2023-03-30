use crate::Span;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NonMatchingTypes { left: Span, right: Span },
    InvalidUnaryOp { expr: Span },
    InvalidBinOp { left: Span, right: Span },
}
