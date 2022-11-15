use crate::util::TextPosition;
use std::{iter::Peekable, str::Chars};

pub struct SourceReader<'a, T: Sized> {
    source: Peekable<Chars<'a>>,
    position: TextPosition,
    elements: Vec<T>,
    last: Option<char>,
    is_whitespace: fn(char) -> bool,
}

impl<'a, T> SourceReader<'a, T> {
    pub fn new(source: &'a String) -> SourceReader<T> {
        SourceReader {
            source: source.chars().peekable(),
            position: TextPosition { line: 1, column: 0 },
            elements: Vec::new(),
            last: None,
            is_whitespace: char::is_whitespace,
        }
    }

    pub fn with_is_whitespace(
        source: &'a String,
        is_whitespace: fn(char) -> bool,
    ) -> SourceReader<T> {
        SourceReader {
            source: source.chars().peekable(),
            position: TextPosition { line: 1, column: 0 },
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

    #[inline(always)]
    pub fn last(&self) -> Option<char> {
        self.last
    }

    #[inline(always)]
    pub fn pos(&self) -> TextPosition {
        self.position
    }

    pub fn has_finished(&mut self) -> bool {
        self.peek().is_none()
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

    pub fn read_whitespace(&mut self) {
        loop {
            let c = self.peek();
            if let Some(c) = c {
                if !(self.is_whitespace)(c) {
                    return;
                }
                self.read();
                continue;
            }
            return;
        }
    }

    pub fn push_element(&mut self, element: T) {
        self.elements.push(element);
    }

    pub fn destruct(self) -> (TextPosition, Vec<T>) {
        (self.position, self.elements)
    }
}
