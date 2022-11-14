use crate::source_parser::Parser;
use leviathan_common::prelude::*;

pub fn read_char(parser: &mut Parser) -> Option<char> {
    parser.position.column += 1;
    let next = parser.source.next();
    if let Some('\n') = next {
        parser.position.line += 1;
        parser.position.column = 1;
    }
    next
}

pub fn read_whitespace(parser: &mut Parser) -> Result<char> {
    let position = parser.position;
    let c = read_char(parser);
    if let None = c {
        return Err(Error::UnexpectedEndOfSource(parser.position));
    }
    let c = c.unwrap();
    if !is_whitespace(c) {
        return Err(Error::UnexpectedCharacter(position, c));
    }
    return Ok(c);
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
