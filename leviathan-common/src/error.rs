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
    // --------------------------------------------------------------
    #[cfg_attr(not(feature = "parser_source"), cfg(never))]
    #[error("Unexpected end of source at {0}")]
    SourceUnexpectedEndOfSource(TextPosition),
    #[cfg_attr(not(feature = "parser_source"), cfg(never))]
    #[error("Unexpected whitespace at {0}")]
    SourceUnexpectedWhitespace(TextPosition),
    #[cfg_attr(not(feature = "parser_source"), cfg(never))]
    #[error("Unexpected character '{1}' at {0}")]
    SourceUnexpectedCharacter(TextPosition, char),
    #[cfg_attr(not(feature = "parser_source"), cfg(never))]
    #[error("Unexpected character '{1}' at {0}, expected {2}")]
    SourceUnexpectedCharacterExpected(TextPosition, char, String),
    #[cfg_attr(not(feature = "parser_source"), cfg(never))]
    #[error("Unexpected newline at {0}")]
    SourceUnexpectedNewline(TextPosition),
    #[cfg_attr(not(feature = "parser_source"), cfg(never))]
    #[error("Unexpected element at {0}: {1:?}")]
    SourceUnexpectedElement(TextPosition, NodeType),
    #[cfg_attr(not(feature = "parser_source"), cfg(never))]
    #[error("Not allowed character '{1}' at {0}")]
    SourceInvalidCharacter(TextPosition, char),
    #[cfg_attr(not(feature = "parser_source"), cfg(never))]
    #[error("Found invalid escape character '{1}' in string at {0}")]
    SourceInvalidEscapeCharacter(TextPosition, char),
    #[cfg_attr(not(feature = "parser_source"), cfg(never))]
    #[error("Invalid floating point number at {0}")]
    SourceInvalidFloatingPointNumber(TextPosition),
    #[cfg_attr(not(feature = "parser_source"), cfg(never))]
    #[error("Could not parse floating point number at {0}: {1}")]
    SourceFloatingPointParseError(TextPosition, <f64 as FromStr>::Err),
    #[cfg_attr(not(feature = "parser_source"), cfg(never))]
    #[error("Could not parse integer at {0}: {1}")]
    SourceIntegerParseError(TextPosition, <i64 as FromStr>::Err),
    #[cfg_attr(not(feature = "parser_source"), cfg(never))]
    #[error("The identifier at {0} cannot be empty")]
    SourceEmptyIdentifier(TextPosition),
    #[cfg_attr(not(feature = "parser_source"), cfg(never))]
    #[error("Duplicate key in map at {0}")]
    SourceDuplicateKeyInMap(TextPosition),
    // --------------------------------------------------------------
    #[cfg_attr(not(feature = "parser_structure"), cfg(never))]
    #[error("Expected namespace (ns <IDENT>) at {0}")]
    StructureExpectedNamespace(TextPosition),
    #[cfg_attr(not(feature = "parser_structure"), cfg(never))]
    #[error("Expected namespace (ns <IDENT>) at {0} but got {1}")]
    StructureExpectedNamespaceGot(TextPosition, String),
    #[cfg_attr(not(feature = "parser_structure"), cfg(never))]
    #[error("Invalid namespace (ns <IDENT>) at {0}")]
    StructureInvalidNamespace(TextPosition),
    #[cfg_attr(not(feature = "parser_structure"), cfg(never))]
    #[error("Invalid namespace arguments (ns <IDENT> <ARGS>) at {0}")]
    StructureInvalidNamespaceArguments(TextPosition),
    #[cfg_attr(not(feature = "parser_structure"), cfg(never))]
    #[error("Invalid namespace argument (ns <IDENT> <ARGS>) at {0}")]
    StructureInvalidNamespaceArgument(TextPosition),
    #[cfg_attr(not(feature = "parser_structure"), cfg(never))]
    #[error("Unexpected element at {0}")]
    StructureUnexpectedElement(TextPosition),
    #[cfg_attr(not(feature = "parser_structure"), cfg(never))]
    #[error("Unknown root operator '{1}' at {0}")]
    StructureUnknownRootOperator(TextPosition, String),
    #[cfg_attr(not(feature = "parser_structure"), cfg(never))]
    #[error("Wrong function structure at {0}; expected (fn <NAME> <ARGS> <TAGS?> <BODY>)")]
    StructureWrongFunctionStructure(TextPosition),
    #[cfg_attr(not(feature = "parser_structure"), cfg(never))]
    #[error("Invalid function parameters at {0}; expected [:<NAME> <TYPE> ...]")]
    StructureInvalidFunctionParameters(TextPosition),
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
