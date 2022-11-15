use crate::parser::source::NodeType;
use crate::util::TextPosition;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(thiserror::Error)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[cfg_attr(not(feature = "parser_source"), cfg(never))]
    #[error("Unexpected end of source at {0}")]
    UnexpectedEndOfSource(TextPosition),
    #[cfg_attr(not(feature = "parser_source"), cfg(never))]
    #[error("Unexpected whitespace at {0}")]
    UnexpectedWhitespace(TextPosition),
    #[cfg_attr(not(feature = "parser_source"), cfg(never))]
    #[error("Unexpected character '{1}' at {0}")]
    UnexpectedCharacter(TextPosition, char),
    #[cfg_attr(not(feature = "parser_source"), cfg(never))]
    #[error("Unexpected character '{1}' at {0}, expected {2}")]
    UnexpectedCharacterExpected(TextPosition, char, String),
    #[cfg_attr(not(feature = "parser_source"), cfg(never))]
    #[error("Unexpected newline at {0}")]
    UnexpectedNewline(TextPosition),
    #[cfg_attr(not(feature = "parser_source"), cfg(never))]
    #[error("Unexpected element at {0}: {1:?}")]
    UnexpectedElement(TextPosition, NodeType),
    #[cfg_attr(not(feature = "parser_source"), cfg(never))]
    #[error("Not allowed character '{1}' at {0}")]
    InvalidCharacter(TextPosition, char),
    #[cfg_attr(not(feature = "parser_source"), cfg(never))]
    #[error("Found invalid escape character '{1}' in string at {0}")]
    InvalidEscapeCharacter(TextPosition, char),
    #[cfg_attr(not(feature = "parser_source"), cfg(never))]
    #[error("Invalid floating point number at {0}")]
    InvalidFloatingPointNumber(TextPosition),
    #[cfg_attr(not(feature = "parser_source"), cfg(never))]
    #[error("Could not parse floating point number at {0}: {1}")]
    FloatingPointParseError(TextPosition, <f64 as FromStr>::Err),
    #[cfg_attr(not(feature = "parser_source"), cfg(never))]
    #[error("Could not parse integer at {0}: {1}")]
    IntegerParseError(TextPosition, <i64 as FromStr>::Err),
    #[cfg_attr(not(feature = "parser_source"), cfg(never))]
    #[error("The identifier at {0} cannot be empty")]
    EmptyIdentifier(TextPosition),
    #[cfg_attr(not(feature = "parser_source"), cfg(never))]
    #[error("Duplicate key in map at {0}")]
    DuplicateKeyInMap(TextPosition),
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
