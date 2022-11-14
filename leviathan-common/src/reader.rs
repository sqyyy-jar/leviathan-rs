use std::{iter::Peekable, str::Chars};

use crate::util::TextPosition;

pub struct SourceReader<'a, T: Sized> {
    source: Peekable<Chars<'a>>,
    position: TextPosition,
    elements: Vec<T>,
    last: Option<char>,
    is_whitespace: fn(char) -> bool,
}

impl<'a, T: Sized> SourceReader<'a, T> {
    pub fn new(source: &'a String) -> Self {
        Self {
            source: source.chars().peekable(),
            position: TextPosition { line: 1, column: 1 },
            elements: Vec::new(),
            last: None,
            is_whitespace: char::is_whitespace,
        }
    }

    pub fn with_is_whitespace(source: &'a String, is_whitespace: fn(char) -> bool) -> Self {
        Self {
            source: source.chars().peekable(),
            position: TextPosition { line: 1, column: 1 },
            elements: Vec::new(),
            last: None,
            is_whitespace,
        }
    }

    pub fn last_was_whitespace(&self) -> bool {
        if let None = self.last {
            return false;
        }
        return (self.is_whitespace)(self.last.unwrap());
    }

    pub fn peek(&mut self) -> Option<char> {
        self.source.peek().copied()
    }

    pub fn read(&mut self) -> Option<char> {
        let next = self.source.next();
        self.position.column += 1;
        if let Some('\n') = next {
            self.position.line += 1;
            self.position.column = 0;
        }
        self.last = next;
        next
    }

    pub fn destruct(self) -> (TextPosition, Vec<T>) {
        (self.position, self.elements)
    }
}
