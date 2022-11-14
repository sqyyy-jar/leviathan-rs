use crate::util::{
    consume_whitespaces, is_closing_bracket, is_valid_ident_char, is_whitespace, peek_char,
    read_char,
};
use leviathan_common::{
    prelude::*,
    util::{Node, NodeType, TextPosition},
};
use std::{collections::HashMap, iter::Peekable, str::Chars};

pub struct Parser<'a> {
    pub source: Peekable<Chars<'a>>,
    pub position: TextPosition,
    pub nodes: Vec<Node>,
}

#[derive(Debug)]
pub struct ParseResult {
    nodes: Vec<Node>,
}

impl<'a> Parser<'a> {
    pub fn parse(source: &'a String) -> Result<ParseResult> {
        let mut parser = Self {
            source: source.chars().peekable(),
            position: TextPosition { line: 1, column: 1 },
            nodes: Vec::new(),
        };
        consume_whitespaces(&mut parser);
        loop {
            let element = parser.parse_element(false, true)?;
            match element {
                Some(element) => parser.nodes.push(element),
                None => break,
            }
        }
        let result = Ok(ParseResult {
            nodes: parser.nodes.clone(),
        });
        return result;
    }

    fn parse_element(&mut self, expect_element: bool, is_root: bool) -> Result<Option<Node>> {
        let position = self.position;
        let c = peek_char(self);
        if let None = c {
            if expect_element {
                return Err(Error::UnexpectedEndOfSource(position));
            } else {
                return Ok(None);
            }
        }
        let c = c.unwrap();
        match c {
            '#' => {
                let result = self.parse_comment(position)?;
                Ok(Some(result))
            }
            ':' => {
                let result = self.parse_atom(position)?;
                Ok(Some(result))
            }
            '"' => {
                let result = self.parse_string(position)?;
                Ok(Some(result))
            }
            '(' => {
                let result = self.parse_node(position, is_root)?;
                Ok(Some(result))
            }
            '[' => {
                let result = self.parse_list(position)?;
                Ok(Some(result))
            }
            '{' => {
                let result = self.parse_map(position)?;
                Ok(Some(result))
            }
            '.' | '-' | '0'..='9' => {
                let result = self.parse_number(position)?;
                Ok(Some(result))
            }
            c => {
                if c.is_ascii_whitespace() {
                    read_char(self);
                    return Err(Error::UnexpectedWhitespace(self.position));
                }
                if is_root {
                    read_char(self);
                    return Err(Error::UnexpectedCharacter(self.position, c));
                }
                if !is_valid_ident_char(c) {
                    read_char(self);
                    return Err(Error::InvalidCharacter(self.position, c));
                }
                let position = self.position;
                let identifier = self.parse_identifier(false)?;
                Ok(Some(Node {
                    position,
                    value: NodeType::Identifier(identifier),
                }))
            }
        }
    }

    fn parse_identifier(&mut self, allow_empty: bool) -> Result<String> {
        let position = self.position;
        let mut buffer = String::new();
        loop {
            let c = peek_char(self);
            if let None = c {
                read_char(self);
                return Err(Error::UnexpectedEndOfSource(self.position));
            }
            let c = c.unwrap();
            if is_whitespace(c) {
                consume_whitespaces(self);
                break;
            }
            if is_closing_bracket(c) {
                break;
            }
            if !is_valid_ident_char(c) {
                read_char(self);
                return Err(Error::InvalidCharacter(self.position, c));
            }
            read_char(self);
            buffer.push(c);
        }
        if !allow_empty && buffer.is_empty() {
            return Err(Error::EmptyIdentifier(position));
        }
        return Ok(buffer);
    }

    fn parse_comment(&mut self, position: TextPosition) -> Result<Node> {
        read_char(self);
        let mut text = String::new();
        loop {
            let c = read_char(self);
            if let None = c {
                return Err(Error::UnexpectedEndOfSource(self.position));
            }
            let c = c.unwrap();
            if c == '\n' {
                break;
            }
            if c == '\r' {
                continue;
            }
            text.push(c);
        }
        Ok(Node {
            position,
            value: NodeType::Comment(text.trim().to_string()),
        })
    }

    fn parse_atom(&mut self, position: TextPosition) -> Result<Node> {
        read_char(self);
        let string = self.parse_identifier(true)?;
        return Ok(Node {
            position,
            value: NodeType::Atom(string),
        });
    }

    fn parse_string(&mut self, position: TextPosition) -> Result<Node> {
        read_char(self);
        let mut string = String::new();
        loop {
            let c = read_char(self);
            if let None = c {
                return Err(Error::UnexpectedEndOfSource(self.position));
            }
            let c = c.unwrap();
            if c == '\n' {
                return Err(Error::UnexpectedNewline(self.position));
            }
            if c == '"' {
                consume_whitespaces(self);
                break;
            }
            if c == '\\' {
                let ec = read_char(self);
                if let None = ec {
                    return Err(Error::UnexpectedEndOfSource(self.position));
                }
                let ec = ec.unwrap();
                match ec {
                    'n' => string.push('\n'),
                    't' => string.push('\t'),
                    'r' => string.push('\r'),
                    '"' => string.push('"'),
                    '\\' => string.push('\\'),
                    _ => return Err(Error::InvalidEscapeCharacter(self.position, ec)),
                }
            }
            string.push(c);
        }
        Ok(Node {
            position,
            value: NodeType::String(string),
        })
    }

    fn parse_number(&mut self, position: TextPosition) -> Result<Node> {
        let mut buffer = String::new();
        let mut is_float = false;
        let c = read_char(self).unwrap();
        if c == '.' {
            is_float = true;
        }
        buffer.push(c);
        loop {
            let c = peek_char(self);
            if let None = c {
                return Err(Error::UnexpectedEndOfSource(self.position));
            }
            let c = c.unwrap();
            if c == '.' {
                read_char(self);
                if is_float {
                    return Err(Error::InvalidFloatingPointNumber(self.position));
                }
                is_float = true;
                continue;
            }
            if !c.is_ascii_digit() {
                if !is_whitespace(c) {
                    if is_closing_bracket(c) {
                        break;
                    }
                    read_char(self);
                    return Err(Error::UnexpectedCharacter(self.position, c));
                }
                consume_whitespaces(self);
                break;
            }
            read_char(self);
            buffer.push(c);
        }
        if is_float {
            let result = buffer.parse::<f64>();
            if result.is_err() {
                return Err(Error::FloatingPointParseError(
                    position,
                    result.unwrap_err(),
                ));
            }
            Ok(Node {
                position,
                value: NodeType::Float(result.unwrap()),
            })
        } else {
            let result = buffer.parse::<i64>();
            if result.is_err() {
                return Err(Error::IntegerParseError(position, result.unwrap_err()));
            }
            Ok(Node {
                position,
                value: NodeType::Integer(result.unwrap()),
            })
        }
    }

    fn parse_node(&mut self, position: TextPosition, is_root: bool) -> Result<Node> {
        read_char(self);
        consume_whitespaces(self);
        let operator = self.parse_identifier(false)?;
        let mut arguments = Vec::with_capacity(0);
        loop {
            let c = peek_char(self);
            if let None = c {
                read_char(self);
                return Err(Error::UnexpectedEndOfSource(self.position));
            }
            let c = c.unwrap();
            if c == ')' {
                read_char(self);
                consume_whitespaces(self);
                break;
            }
            let element = self.parse_element(true, false)?;
            match element {
                Some(element) => arguments.push(element),
                None => return Err(Error::UnexpectedEndOfSource(self.position)),
            }
        }
        Ok(Node {
            position,
            value: NodeType::Node {
                operator,
                arguments,
            },
        })
    }

    fn parse_list(&mut self, position: TextPosition) -> Result<Node> {
        read_char(self);
        consume_whitespaces(self);
        let mut values = Vec::with_capacity(0);
        loop {
            let Some(c) = peek_char(self) else {
                read_char(self);
                return Err(Error::UnexpectedEndOfSource(self.position));
            };
            if c == ']' {
                read_char(self);
                consume_whitespaces(self);
                break;
            }
            if is_closing_bracket(c) {
                read_char(self);
                return Err(Error::UnexpectedCharacter(self.position, c));
            }
            let element = self.parse_element(true, false)?;
            match element {
                Some(element) => values.push(element),
                None => return Err(Error::UnexpectedEndOfSource(self.position)),
            }
        }
        Ok(Node {
            position,
            value: NodeType::List(values),
        })
    }

    fn parse_map(&mut self, position: TextPosition) -> Result<Node> {
        read_char(self);
        consume_whitespaces(self);
        let mut map = HashMap::new();
        loop {
            let Some(c) = peek_char(self) else {
                read_char(self);
                return Err(Error::UnexpectedEndOfSource(self.position));
            };
            if c == '}' {
                read_char(self);
                consume_whitespaces(self);
                break;
            }
            if is_closing_bracket(c) {
                read_char(self);
                return Err(Error::UnexpectedCharacter(self.position, c));
            }
            let key_position = self.position;
            let key = self.parse_element(true, false)?;
            let Some(key) = key else {
                return Err(Error::UnexpectedEndOfSource(self.position));
            };
            let Node { position: _, value: NodeType::Atom(key_string) } = key else {
                return Err(Error::UnexpectedElement(key.position, key.value));
            };
            let value = self.parse_element(true, false)?;
            match value {
                Some(value) => {
                    let replace = map.insert(key_string, value);
                    if let Some(_) = replace {
                        return Err(Error::DuplicateKeyInMap(key_position));
                    }
                }
                None => return Err(Error::UnexpectedEndOfSource(self.position)),
            }
        }
        Ok(Node {
            position,
            value: NodeType::Map(map),
        })
    }
}
