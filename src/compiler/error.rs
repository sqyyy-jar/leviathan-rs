use std::{ops::RangeFrom, process::exit};

use ariadne::Source;

use crate::util::{
    ariadne::{error_report, span_error_report},
    source::Span,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidOperation,
    DuplicateModule {
        name: Option<String>,
    },
    EmptyModule {
        name: Option<String>,
    },
    InvalidModuleDeclaration {
        src: Option<String>,
        span: Span,
    },
    UnknownModuleType {
        src: Option<String>,
        span: Span,
    },
    EmptyNode {
        src: Option<String>,
        span: Span,
    },
    UnexpectedToken {
        src: Option<String>,
        span: Span,
    },
    InvalidKeyword {
        src: Option<String>,
        span: Span,
    },
    InvalidStatement {
        src: Option<String>,
        span: Span,
    },
    DuplicateName {
        src: Option<String>,
        span: Span,
    },
    UnknownFunc {
        src: Option<String>,
        span: Span,
    },
    UnknownStaticFunc {
        src: Option<String>,
        span: Span,
    },
    UnknownStaticVariable {
        src: Option<String>,
        span: Span,
    },
    InvalidCallSignature {
        src: Option<String>,
        span: Span,
    },
    NotInSizeRangeFrom {
        src: Option<String>,
        span: Span,
        range: RangeFrom<usize>,
    },
}

impl Error {
    pub fn complete(mut self, source: String) -> Self {
        match &mut self {
            Error::InvalidOperation | Error::DuplicateModule { .. } | Error::EmptyModule { .. } => {
            }
            Error::InvalidModuleDeclaration { src, .. }
            | Error::UnknownModuleType { src, .. }
            | Error::EmptyNode { src, .. }
            | Error::UnexpectedToken { src, .. }
            | Error::InvalidKeyword { src, .. }
            | Error::InvalidStatement { src, .. }
            | Error::DuplicateName { src, .. }
            | Error::UnknownFunc { src, .. }
            | Error::UnknownStaticFunc { src, .. }
            | Error::UnknownStaticVariable { src, .. }
            | Error::InvalidCallSignature { src, .. }
            | Error::NotInSizeRangeFrom { src, .. } => {
                *src = Some(source);
            }
        }
        self
    }

    pub fn report(&self, filename: &str) {
        let report;
        let source;
        match self {
            Error::InvalidOperation => panic!("InvalidOperation"),
            Error::DuplicateModule { name: Some(name) } => {
                report = error_report(
                    filename,
                    &format!("A module with the name '{name}' already exists",),
                );
                source = Source::from("");
            }
            Error::EmptyModule { name: Some(name) } => {
                report = error_report(filename, &format!("The module '{name}' is empty"));
                source = Source::from("");
            }
            Error::InvalidModuleDeclaration {
                src: Some(src),
                span,
            } => {
                report = span_error_report(filename, span, "This module declaration is not valid");
                source = Source::from(src);
            }
            Error::UnknownModuleType {
                src: Some(src),
                span,
            } => {
                report = span_error_report(filename, span, "This module type is unknown");
                source = Source::from(src);
            }
            Error::EmptyNode {
                src: Some(src),
                span,
            } => {
                report = span_error_report(filename, span, "This node must not be empty");
                source = Source::from(src);
            }
            Error::UnexpectedToken {
                src: Some(src),
                span,
            } => {
                report = span_error_report(filename, span, "This token is not valid here");
                source = Source::from(src);
            }
            Error::InvalidKeyword {
                src: Some(src),
                span,
            } => {
                report = span_error_report(filename, span, "This keyword not valid");
                source = Source::from(src);
            }
            Error::InvalidStatement {
                src: Some(src),
                span,
            } => {
                report = span_error_report(filename, span, "This statement not valid");
                source = Source::from(src);
            }
            Error::DuplicateName {
                src: Some(src),
                span,
            } => {
                report = span_error_report(filename, span, "This name is already in use");
                source = Source::from(src);
            }
            Error::UnknownFunc {
                src: Some(src),
                span,
            } => {
                report = span_error_report(filename, span, "This function is not known");
                source = Source::from(src);
            }
            Error::UnknownStaticFunc {
                src: Some(src),
                span,
            } => {
                report = span_error_report(filename, span, "This static function is not known");
                source = Source::from(src);
            }
            Error::UnknownStaticVariable {
                src: Some(src),
                span,
            } => {
                report = span_error_report(filename, span, "This static variable does not exist");
                source = Source::from(src);
            }
            Error::InvalidCallSignature {
                src: Some(src),
                span,
            } => {
                report = span_error_report(
                    filename,
                    span,
                    "This call signature does not match the function signature",
                );
                source = Source::from(src);
            }
            Error::NotInSizeRangeFrom {
                src: Some(src),
                span,
                range,
            } => {
                report = span_error_report(
                    filename,
                    span,
                    &format!("This number must be bigger or equal to {}", range.start),
                );
                source = Source::from(src);
            }
            _ => panic!("Tried to report incomplete error"),
        }
        report.eprint((filename, source)).unwrap();
    }

    pub fn abort(&self, filename: &str) -> ! {
        self.report(filename);
        exit(1);
    }
}
