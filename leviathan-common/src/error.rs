use crate::util::{NodeType, TextPosition};
use std::{str::FromStr, fmt::{Debug, Formatter, Display}};

#[derive(thiserror::Error)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error("Unexpected end of source at {0}")]
    UnexpectedEndOfSource(TextPosition),
    #[error("Unexpected whitespace at {0}")]
    UnexpectedWhitespace(TextPosition),
    #[error("Unexpected character '{1}' at {0}")]
    UnexpectedCharacter(TextPosition, char),
    #[error("Unexpected character '{1}' at {0}, expected {2}")]
    UnexpectedCharacterExpected(TextPosition, char, String),
    #[error("Unexpected newline at {0}")]
    UnexpectedNewline(TextPosition),
    #[error("Unexpected element at {0}: {1:?}")]
    UnexpectedElement(TextPosition, NodeType),
    #[error("Not allowed character '{1}' at {0}")]
    InvalidCharacter(TextPosition, char),
    #[error("Found invalid escape character '{1}' in string at {0}")]
    InvalidEscapeCharacter(TextPosition, char),
    #[error("Invalid floating point number at {0}")]
    InvalidFloatingPointNumber(TextPosition),
    #[error("Could not parse floating point number at {0}: {1}")]
    FloatingPointParseError(TextPosition, <f64 as FromStr>::Err),
    #[error("Could not parse integer at {0}: {1}")]
    IntegerParseError(TextPosition, <i64 as FromStr>::Err),
    #[error("The identifier at {0} cannot be empty")]
    EmptyIdentifier(TextPosition),
    #[error("Duplicate key in map at {0}")]
    DuplicateKeyInMap(TextPosition),
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
