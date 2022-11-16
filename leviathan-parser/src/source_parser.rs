use crate::util::{is_closing_bracket, is_valid_ident_char, is_whitespace};
use leviathan_common::parser::source::{Node, NodeType};
use leviathan_common::{parser::reader::SourceReader, prelude::*, util::TextPosition};
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug)]
pub struct ParseResult(pub Vec<Node>);

pub fn parse(source: &String) -> Result<ParseResult> {
    let mut reader = SourceReader::<Node>::with_is_whitespace(source, is_whitespace);
    reader.read_whitespace();
    loop {
        let element = parse_element(&mut reader, false, true)?;
        match element {
            Some(element) => reader.push_element(element),
            None => break,
        }
    }
    Ok(ParseResult(reader.destruct().1))
}

fn parse_element(
    reader: &mut SourceReader<Node>,
    expect_element: bool,
    is_root: bool,
) -> Result<Option<Node>> {
    let position = reader.pos();
    let c = reader.peek();
    if let None = c {
        if expect_element {
            return Err(Error::SourceUnexpectedEndOfSource(position));
        } else {
            return Ok(None);
        }
    }
    let c = c.unwrap();
    match c {
        '#' => {
            let result = parse_comment(reader, position)?;
            Ok(Some(result))
        }
        ':' => {
            let result = parse_atom(reader, position)?;
            Ok(Some(result))
        }
        '"' => {
            let result = parse_string(reader, position)?;
            Ok(Some(result))
        }
        '(' => {
            let result = parse_node(reader, position, is_root)?;
            Ok(Some(result))
        }
        '[' => {
            let result = parse_list(reader, position)?;
            Ok(Some(result))
        }
        '{' => {
            let result = parse_map(reader, position)?;
            Ok(Some(result))
        }
        '.' | '-' | '0'..='9' => {
            let result = parse_number(reader, position)?;
            Ok(Some(result))
        }
        c => {
            if c.is_ascii_whitespace() {
                reader.read();
                return Err(Error::SourceUnexpectedWhitespace(reader.pos()));
            }
            if is_root {
                reader.read();
                return Err(Error::SourceUnexpectedCharacter(reader.pos(), c));
            }
            if !is_valid_ident_char(c) {
                reader.read();
                return Err(Error::SourceInvalidCharacter(reader.pos(), c));
            }
            let position = reader.pos();
            let identifier = parse_identifier(reader, false)?;
            Ok(Some(Node {
                position,
                value: NodeType::Identifier(identifier),
            }))
        }
    }
}

fn parse_identifier(reader: &mut SourceReader<Node>, allow_empty: bool) -> Result<String> {
    let position = reader.pos();
    let mut buffer = String::new();
    loop {
        let c = reader.peek();
        if let None = c {
            reader.read();
            return Err(Error::SourceUnexpectedEndOfSource(reader.pos()));
        }
        let c = c.unwrap();
        if is_whitespace(c) {
            reader.read_whitespace();
            break;
        }
        if is_closing_bracket(c) {
            break;
        }
        if !is_valid_ident_char(c) {
            reader.read();
            return Err(Error::SourceInvalidCharacter(reader.pos(), c));
        }
        reader.read();
        buffer.push(c);
    }
    if !allow_empty && buffer.is_empty() {
        return Err(Error::SourceEmptyIdentifier(position));
    }
    return Ok(buffer);
}

fn parse_comment(reader: &mut SourceReader<Node>, position: TextPosition) -> Result<Node> {
    reader.read();
    let mut text = String::new();
    loop {
        let c = reader.read();
        if let None = c {
            return Err(Error::SourceUnexpectedEndOfSource(reader.pos()));
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

fn parse_atom(reader: &mut SourceReader<Node>, position: TextPosition) -> Result<Node> {
    reader.read();
    let string = parse_identifier(reader, true)?;
    return Ok(Node {
        position,
        value: NodeType::Atom(string),
    });
}

fn parse_string(reader: &mut SourceReader<Node>, position: TextPosition) -> Result<Node> {
    reader.read();
    let mut string = String::new();
    loop {
        let c = reader.read();
        if let None = c {
            return Err(Error::SourceUnexpectedEndOfSource(reader.pos()));
        }
        let c = c.unwrap();
        if c == '\n' {
            return Err(Error::SourceUnexpectedNewline(reader.pos()));
        }
        if c == '"' {
            reader.read_whitespace();
            break;
        }
        if c == '\\' {
            let ec = reader.read();
            if let None = ec {
                return Err(Error::SourceUnexpectedEndOfSource(reader.pos()));
            }
            let ec = ec.unwrap();
            match ec {
                'n' => string.push('\n'),
                't' => string.push('\t'),
                'r' => string.push('\r'),
                '"' => string.push('"'),
                '\\' => string.push('\\'),
                _ => return Err(Error::SourceInvalidEscapeCharacter(reader.pos(), ec)),
            }
        }
        string.push(c);
    }
    Ok(Node {
        position,
        value: NodeType::String(string),
    })
}

fn parse_number(reader: &mut SourceReader<Node>, position: TextPosition) -> Result<Node> {
    let mut buffer = String::new();
    let mut is_float = false;
    let c = reader.read().unwrap();
    if c == '.' {
        is_float = true;
    }
    buffer.push(c);
    loop {
        let c = reader.peek();
        if let None = c {
            return Err(Error::SourceUnexpectedEndOfSource(reader.pos()));
        }
        let c = c.unwrap();
        if c == '.' {
            reader.read();
            if is_float {
                return Err(Error::SourceInvalidFloatingPointNumber(reader.pos()));
            }
            is_float = true;
            continue;
        }
        if !c.is_ascii_digit() {
            if !is_whitespace(c) {
                if is_closing_bracket(c) {
                    break;
                }
                reader.read();
                return Err(Error::SourceUnexpectedCharacter(reader.pos(), c));
            }
            reader.read_whitespace();
            break;
        }
        reader.read();
        buffer.push(c);
    }
    if is_float {
        let result = buffer.parse::<f64>();
        if result.is_err() {
            return Err(Error::SourceFloatingPointParseError(
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
            return Err(Error::SourceIntegerParseError(position, result.unwrap_err()));
        }
        Ok(Node {
            position,
            value: NodeType::Integer(result.unwrap()),
        })
    }
}

fn parse_node(
    reader: &mut SourceReader<Node>,
    position: TextPosition,
    _is_root: bool,
) -> Result<Node> {
    reader.read();
    reader.read_whitespace();
    let operator = parse_identifier(reader, false)?;
    let mut arguments = Vec::with_capacity(0);
    loop {
        let c = reader.peek();
        if let None = c {
            reader.read();
            return Err(Error::SourceUnexpectedEndOfSource(reader.pos()));
        }
        let c = c.unwrap();
        if c == ')' {
            reader.read();
            reader.read_whitespace();
            break;
        }
        let element = parse_element(reader, true, false)?;
        match element {
            Some(element) => arguments.push(element),
            None => return Err(Error::SourceUnexpectedEndOfSource(reader.pos())),
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

fn parse_list(reader: &mut SourceReader<Node>, position: TextPosition) -> Result<Node> {
    reader.read();
    reader.read_whitespace();
    let mut values = Vec::with_capacity(0);
    loop {
        let Some(c) = reader.peek() else {
            reader.read();
            return Err(Error::SourceUnexpectedEndOfSource(reader.pos()));
        };
        if c == ']' {
            reader.read();
            reader.read_whitespace();
            break;
        }
        if is_closing_bracket(c) {
            reader.read();
            return Err(Error::SourceUnexpectedCharacter(reader.pos(), c));
        }
        let element = parse_element(reader, true, false)?;
        match element {
            Some(element) => values.push(element),
            None => return Err(Error::SourceUnexpectedEndOfSource(reader.pos())),
        }
    }
    Ok(Node {
        position,
        value: NodeType::List(values),
    })
}

fn parse_map(reader: &mut SourceReader<Node>, position: TextPosition) -> Result<Node> {
    reader.read();
    reader.read_whitespace();
    let mut map = HashMap::new();
    loop {
        let Some(c) = reader.peek() else {
            reader.read();
            return Err(Error::SourceUnexpectedEndOfSource(reader.pos()));
        };
        if c == '}' {
            reader.read();
            reader.read_whitespace();
            break;
        }
        if is_closing_bracket(c) {
            reader.read();
            return Err(Error::SourceUnexpectedCharacter(reader.pos(), c));
        }
        let key_position = reader.pos();
        let key = parse_element(reader, true, false)?;
        let Some(key) = key else {
            return Err(Error::SourceUnexpectedEndOfSource(reader.pos()));
        };
        let Node { position: _, value: NodeType::Atom(key_string) } = key else {
            return Err(Error::SourceUnexpectedElement(key.position, key.value));
        };
        let value = parse_element(reader, true, false)?;
        match value {
            Some(value) => {
                let replace = map.insert(key_string, value);
                if let Some(_) = replace {
                    return Err(Error::SourceDuplicateKeyInMap(key_position));
                }
            }
            None => return Err(Error::SourceUnexpectedEndOfSource(reader.pos())),
        }
    }
    Ok(Node {
        position,
        value: NodeType::Map(map),
    })
}
