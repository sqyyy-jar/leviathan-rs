use std::process::exit;

use ariadne::Source;

use crate::util::{ariadne::span_error_report, source::Span};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IdentStartingWithDigit { src: String, span: Span },
    NoWhitespaceBetweenTokens { src: String, span: Span },
    UnexpectedEndOfSource { src: String, span: Span },
    InvalidStringEscapeCode { src: String, span: Span },
    IllegalTokenAtRootLevel { src: String, span: Span },
    UnclosedParenthesis { src: String, span: Span },
    InvalidUtf8 { src: String, span: Span },
}

impl Error {
    pub fn report(&self, filename: &str) {
        let report;
        let src = match self {
            Error::IdentStartingWithDigit { src, span } => {
                report =
                    span_error_report(filename, span, "Identifiers cannot start with a digit");
                src
            }
            Error::NoWhitespaceBetweenTokens { src, span } => {
                report =
                    span_error_report(filename, span, "There must be whitespace between token");
                src
            }
            Error::UnexpectedEndOfSource { src, span } => {
                report = span_error_report(filename, span, "The code is not allowed to end here");
                src
            }
            Error::InvalidStringEscapeCode { src, span } => {
                report = span_error_report(filename, span, "This escape code is not valid");
                src
            }
            Error::IllegalTokenAtRootLevel { src, span } => {
                report = span_error_report(
                    filename,
                    span,
                    "This token is not allowed on the root-level",
                );
                src
            }
            Error::UnclosedParenthesis { src, span } => {
                report = span_error_report(filename, span, "This parenthesis must be closed");
                src
            }
            Error::InvalidUtf8 { src, span } => {
                report = span_error_report(filename, span, "This is invalid Utf8");
                src
            }
        };
        report.eprint((filename, Source::from(src))).unwrap();
    }

    pub fn abort(&self, filename: &str) -> ! {
        self.report(filename);
        exit(1);
    }
}
