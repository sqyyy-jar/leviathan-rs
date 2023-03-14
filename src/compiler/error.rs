use std::ops::RangeFrom;

use crate::util::source::Span;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidOperation,
    DuplicateModule { name: String },
    EmptyModule { name: String },
    InvalidModuleDeclaration { span: Span },
    UnknownModuleType { span: Span },
    EmptyNode { span: Span },
    UnexpectedToken { span: Span },
    InvalidKeyword { span: Span },
    InvalidStatement { span: Span },
    DuplicateName { span: Span },
    UnknownFunc { span: Span },
    UnknownStaticFunc { span: Span },
    InvalidCallSignature { span: Span },
    NotInSizeRangeFrom { span: Span, range: RangeFrom<usize> },
}
