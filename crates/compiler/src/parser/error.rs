use std::process::exit;

use ariadne::Source;

use crate::util::{
    ariadne::{span_double_error_report, span_error_report},
    source::Span,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IdentStartingWithDigit {
        file: String,
        src: String,
        span: Span,
    },
    NoWhitespaceBetweenTokens {
        file: String,
        src: String,
        span: Span,
    },
    UnexpectedEndOfSource {
        file: String,
        src: String,
        span: Span,
    },
    InvalidStringEscapeCode {
        file: String,
        src: String,
        span: Span,
    },
    IllegalTokenAtRootLevel {
        file: String,
        src: String,
        span: Span,
    },
    UnclosedParenthesis {
        file: String,
        src: String,
        span: Span,
    },
    InvalidUtf8 {
        file: String,
        src: String,
        span: Span,
    },
    MissmatchBrackets {
        file: String,
        src: String,
        span_a: Span,
        span_b: Span,
    },
}

impl Error {
    pub fn report(&self) {
        let report;
        let source;
        let file: &str = match self {
            Error::IdentStartingWithDigit { file, src, span } => {
                report = span_error_report(file, span, "Identifiers cannot start with a digit");
                source = Source::from(src);
                file
            }
            Error::NoWhitespaceBetweenTokens { file, src, span } => {
                report = span_error_report(file, span, "There must be whitespace between token");
                source = Source::from(src);
                file
            }
            Error::UnexpectedEndOfSource { file, src, span } => {
                report = span_error_report(file, span, "The code is not allowed to end here");
                source = Source::from(src);
                file
            }
            Error::InvalidStringEscapeCode { file, src, span } => {
                report = span_error_report(file, span, "This escape code is not valid");
                source = Source::from(src);
                file
            }
            Error::IllegalTokenAtRootLevel { file, src, span } => {
                report =
                    span_error_report(file, span, "This token is not allowed on the root-level");
                source = Source::from(src);
                file
            }
            Error::UnclosedParenthesis { file, src, span } => {
                report = span_error_report(file, span, "This parenthesis must be closed");
                source = Source::from(src);
                file
            }
            Error::InvalidUtf8 { file, src, span } => {
                report = span_error_report(file, span, "This is invalid Utf8");
                source = Source::from(src);
                file
            }
            Error::MissmatchBrackets {
                file,
                src,
                span_a,
                span_b,
            } => {
                report = span_double_error_report(
                    file,
                    span_a,
                    span_b,
                    "start",
                    "end - These brackets do not match",
                );
                source = Source::from(src);
                file
            }
        };
        report.eprint((file, source)).unwrap();
    }

    pub fn abort(&self) -> ! {
        self.report();
        exit(1);
    }
}
