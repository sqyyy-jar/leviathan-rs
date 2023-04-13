use std::{
    ops::{Range, RangeFrom},
    process::exit,
};

use ariadne::Source;

use crate::util::{
    ariadne::{error_report, span_error_report, span_error_report_with_note},
    source::Span,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidOperation,
    NoMainFound {
        file: String,
    },
    DuplicateModule {
        file: String,
        name: String,
    },
    EmptyModule {
        file: String,
        name: String,
    },
    InvalidModuleDeclaration {
        file: String,
        src: String,
        span: Span,
    },
    UnknownModuleDialect {
        file: String,
        src: String,
        span: Span,
    },
    EmptyNode {
        file: String,
        src: String,
        span: Span,
    },
    UnexpectedToken {
        file: String,
        src: String,
        span: Span,
    },
    UnexpectedTokens {
        file: String,
        src: String,
        span: Span,
    },
    InvalidKeyword {
        file: String,
        src: String,
        span: Span,
    },
    InvalidStatement {
        file: String,
        src: String,
        span: Span,
    },
    InvalidType {
        file: String,
        src: String,
        span: Span,
    },
    InvalidBracketType {
        file: String,
        src: String,
        span: Span,
    },
    InvalidParams {
        file: String,
        src: String,
        span: Span,
    },
    DuplicateName {
        file: String,
        src: String,
        span: Span,
    },
    DuplicateImport {
        file: String,
        src: String,
        span: Span,
    },
    UnknownModule {
        file: String,
        src: String,
        span: Span,
    },
    UnknownFunc {
        file: String,
        src: String,
        span: Span,
    },
    UnknownStaticFunc {
        file: String,
        src: String,
        span: Span,
    },
    UnknownStaticVariable {
        file: String,
        src: String,
        span: Span,
    },
    InvalidCallSignature {
        file: String,
        src: String,
        span: Span,
    },
    InvalidCondition {
        file: String,
        src: String,
        span: Span,
    },
    InvalidRegister {
        file: String,
        src: String,
        span: Span,
    },
    SelfImport {
        file: String,
        src: String,
        span: Span,
    },
    NotInSizeRangeFrom {
        file: String,
        src: String,
        span: Span,
        range: RangeFrom<usize>,
    },
    NotInSizeRange {
        file: String,
        src: String,
        span: Span,
        range: Range<usize>,
    },
    NotInI64Range {
        file: String,
        src: String,
        span: Span,
        range: Range<i64>,
    },
    NegativeNumber {
        file: String,
        src: String,
        span: Span,
    },
    OversizedNumber {
        file: String,
        src: String,
        span: Span,
    },
    InvalidByte {
        file: String,
        src: String,
        span: Span,
    },
    EmptyBuffer {
        file: String,
        src: String,
        span: Span,
    },
    EmptyArray {
        file: String,
        src: String,
        span: Span,
    },
    IoError(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl Error {
    pub fn report(&self) {
        let report;
        let source;
        let file: &str = match self {
            Error::InvalidOperation => panic!("InvalidOperation"),
            Error::NoMainFound { file } => {
                report = error_report(file, "No main function was found");
                source = Source::from("");
                file
            }
            Error::DuplicateModule { file, name } => {
                report = error_report(
                    file,
                    &format!("A module with the name '{name}' already exists"),
                );
                source = Source::from("");
                file
            }
            Error::EmptyModule { name, file } => {
                report = error_report(file, &format!("The module '{name}' is empty"));
                source = Source::from("");
                file
            }
            Error::InvalidModuleDeclaration { file, src, span } => {
                report = span_error_report(file, span, "This module declaration is not valid");
                source = Source::from(src);
                file
            }
            Error::UnknownModuleDialect { file, src, span } => {
                report = span_error_report(file, span, "This module dialect is unknown");
                source = Source::from(src);
                file
            }
            Error::EmptyNode { file, src, span } => {
                report = span_error_report(file, span, "This node must not be empty");
                source = Source::from(src);
                file
            }
            Error::UnexpectedToken { file, src, span } => {
                report = span_error_report(file, span, "This token is not valid here");
                source = Source::from(src);
                file
            }
            Error::UnexpectedTokens { file, src, span } => {
                report = span_error_report(file, span, "These tokens are not valid here");
                source = Source::from(src);
                file
            }
            Error::InvalidKeyword { file, src, span } => {
                report = span_error_report(file, span, "This keyword is not valid");
                source = Source::from(src);
                file
            }
            Error::InvalidStatement { file, src, span } => {
                report = span_error_report(file, span, "This statement is not valid");
                source = Source::from(src);
                file
            }
            Error::InvalidType { file, src, span } => {
                report = span_error_report(file, span, "This type is not valid");
                source = Source::from(src);
                file
            }
            Error::InvalidBracketType { file, src, span } => {
                report = span_error_report(file, span, "This bracket type is not allowed here");
                source = Source::from(src);
                file
            }
            Error::InvalidParams { file, src, span } => {
                report = span_error_report(file, span, "These function parameters are not valid");
                source = Source::from(src);
                file
            }
            Error::DuplicateName { file, src, span } => {
                report = span_error_report(file, span, "This name is already in use");
                source = Source::from(src);
                file
            }
            Error::DuplicateImport { file, src, span } => {
                report = span_error_report(file, span, "This module is already imported");
                source = Source::from(src);
                file
            }
            Error::UnknownModule { file, src, span } => {
                report = span_error_report(file, span, "This module could not be found");
                source = Source::from(src);
                file
            }
            Error::UnknownFunc { file, src, span } => {
                report = span_error_report(file, span, "This function could not be found");
                source = Source::from(src);
                file
            }
            Error::UnknownStaticFunc { file, src, span } => {
                report = span_error_report(file, span, "This static function is not known");
                source = Source::from(src);
                file
            }
            Error::UnknownStaticVariable { file, src, span } => {
                report = span_error_report(file, span, "This static variable does not exist");
                source = Source::from(src);
                file
            }
            Error::InvalidCallSignature { file, src, span } => {
                report = span_error_report(
                    file,
                    span,
                    "This call signature does not match the function signature",
                );
                source = Source::from(src);
                file
            }
            Error::InvalidCondition { file, src, span } => {
                report = span_error_report_with_note(
                    file,
                    span,
                    "Invalid condition",
                    "Valid conditions are '=', '!=', '<', '>', '<=', '>=', '!0' and '=0'",
                );
                source = Source::from(src);
                file
            }
            Error::InvalidRegister { file, src, span } => {
                report = span_error_report(file, span, "Invalid register");
                source = Source::from(src);
                file
            }
            Error::SelfImport { file, src, span } => {
                report =
                    span_error_report(file, span, "A module cannot be imported inside of itself");
                source = Source::from(src);
                file
            }
            Error::NotInSizeRangeFrom {
                file,
                src,
                span,
                range,
            } => {
                report = span_error_report(
                    file,
                    span,
                    &format!("This number must be bigger or equal to {}", range.start),
                );
                source = Source::from(src);
                file
            }
            Error::NotInSizeRange {
                file,
                src,
                span,
                range,
            } => {
                report = span_error_report(
                    file,
                    span,
                    &format!("This number must be in range {range:?}"),
                );
                source = Source::from(src);
                file
            }
            Error::NotInI64Range {
                file,
                src,
                span,
                range,
            } => {
                report = span_error_report(
                    file,
                    span,
                    &format!("This number must be in range {range:?}"),
                );
                source = Source::from(src);
                file
            }
            Error::NegativeNumber { file, src, span } => {
                report = span_error_report(file, span, "This number must not be negative");
                source = Source::from(src);
                file
            }
            Error::OversizedNumber { file, src, span } => {
                report = span_error_report(file, span, "This number is too big");
                source = Source::from(src);
                file
            }
            Error::InvalidByte { file, src, span } => {
                report = span_error_report(file, span, "This number does not fit into a byte");
                source = Source::from(src);
                file
            }
            Error::EmptyBuffer { file, src, span } => {
                report = span_error_report(file, span, "A buffer initialization must not be empty");
                source = Source::from(src);
                file
            }
            Error::EmptyArray { file, src, span } => {
                report = span_error_report(file, span, "This array must not be empty");
                source = Source::from(src);
                file
            }
            Error::IoError(err) => {
                report = error_report("", &format!("I/O: {err}"));
                source = Source::from("");
                ""
            }
        };
        report.eprint((file, source)).unwrap();
    }

    pub fn abort(&self) -> ! {
        self.report();
        exit(1);
    }
}
