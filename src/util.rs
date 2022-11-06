use crate::parser::{Parser, TextPosition};
use std::str::FromStr;

#[derive(Debug)]
pub enum Error {
    UnexpectedEndOfSource(TextPosition),
    UnexpectedWhitespace(TextPosition),
    UnexpectedCharacter(TextPosition, char),
    UnexpectedNewline(TextPosition),
    InvalidCharacter(TextPosition, char),
    InvalidEscapeCharacter(TextPosition, char),
    InvalidFloatingPointNumber(TextPosition),
    UnparsableFloatingPointNumber(TextPosition, <f64 as FromStr>::Err),
    UnparsableInteger(TextPosition, <i64 as FromStr>::Err),
    EmptyIdentifier(TextPosition),
}

pub fn read_char(parser: &mut Parser) -> Option<char> {
    parser.position.column += 1;
    let next = parser.source.next();
    if let Some('\n') = next {
        parser.position.line += 1;
        parser.position.column = 1;
    }
    next
}

pub fn peek_char(parser: &mut Parser) -> Option<char> {
    let next = parser.source.peek().copied();
    next
}

pub fn consume_whitespaces(parser: &mut Parser) {
    loop {
        match peek_char(parser) {
            Some(c) => {
                if is_whitespace(c) {
                    read_char(parser);
                    continue;
                }
                return;
            }
            None => return,
        }
    }
}

pub fn is_valid_ident_char(c: char) -> bool {
    if c.is_whitespace() || c == ',' {
        return false;
    }
    match c {
        '[' | ']' | '{' | '}' | '(' | '"' | '\'' | '`' => false,
        _ => true,
    }
}

pub fn is_closing_bracket(c: char) -> bool {
    c == ')' || c == ']' || c == '}'
}

pub fn is_whitespace(c: char) -> bool {
    c.is_ascii_whitespace() || c == ','
}
