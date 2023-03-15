use std::{ops::RangeFrom, process::exit};

use crate::util::source::Span;

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
            | Error::InvalidCallSignature { src, .. }
            | Error::NotInSizeRangeFrom { src, .. } => {
                *src = Some(source);
            }
        }
        self
    }

    // pub fn report(&self, filename: &str) {
    //     let report;
    //     let source;
    //     let src = match self {
    //         Error::InvalidOperation => panic!("InvalidOperation"),
    //         Error::DuplicateModule { name } => {
    //             report = error_report(
    //                 filename,
    //                 &format!("A module with the name '{name}' already exists"),
    //             );
    //             source = Source::from("");
    //         }
    //         Error::EmptyModule { name } => {
    //             report = error_report(filename, &format!("The module '{name}' is empty"));
    //             source = Source::from("");
    //         }
    //         Error::InvalidModuleDeclaration { src, span } => {
    //             report = span_error_report(filename, span, "");
    //             source = Source::from(src);
    //         }
    //         Error::UnknownModuleType { src, span } => {
    //             report = error_report(filename, &format!("This module type is unknown"));
    //             source = Source::from(src);
    //         }
    //         Error::EmptyNode { src, span } => {
    //             report = error_report(
    //                 filename,
    //                 &format!("A module with the name '' already exists"),
    //             );
    //             source = Source::from("");
    //         }
    //         Error::UnexpectedToken { span } => {
    //             report = error_report(
    //                 filename,
    //                 &format!("A module with the name '' already exists"),
    //             );
    //             source = Source::from("");
    //         }
    //         Error::InvalidKeyword { span } => {
    //             report = error_report(
    //                 filename,
    //                 &format!("A module with the name '' already exists"),
    //             );
    //             source = Source::from("");
    //         }
    //         Error::InvalidStatement { span } => {
    //             report = error_report(
    //                 filename,
    //                 &format!("A module with the name '' already exists"),
    //             );
    //             source = Source::from("");
    //         }
    //         Error::DuplicateName { span } => {
    //             report = error_report(
    //                 filename,
    //                 &format!("A module with the name '' already exists"),
    //             );
    //             source = Source::from("");
    //         }
    //         Error::UnknownFunc { span } => {
    //             report = error_report(
    //                 filename,
    //                 &format!("A module with the name '' already exists"),
    //             );
    //             source = Source::from("");
    //         }
    //         Error::UnknownStaticFunc { span } => {
    //             report = error_report(
    //                 filename,
    //                 &format!("A module with the name '' already exists"),
    //             );
    //             source = Source::from("");
    //         }
    //         Error::InvalidCallSignature { span } => {
    //             report = error_report(
    //                 filename,
    //                 &format!("A module with the name '' already exists"),
    //             );
    //             source = Source::from("");
    //         }
    //         Error::NotInSizeRangeFrom { span, range } => {
    //             report = error_report(
    //                 filename,
    //                 &format!("A module with the name '' already exists"),
    //             );
    //             source = Source::from("");
    //         }
    //     };
    //     report.eprint((filename, source)).unwrap();
    // }

    pub fn abort(&self, _filename: &str) -> ! {
        // self.report(filename);
        exit(1);
    }
}
