use crate::util::source::Span;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidOperation,
    EmptyModule { name: String },
    InvalidModuleDeclaration { span: Span },
    UnknownModuleType { span: Span },
    EmptyNode { span: Span },
    UnexpectedToken { span: Span },
    InvalidKeyword { span: Span },
    InvalidStatement { span: Span },
    DuplicateName { span: Span },
}
