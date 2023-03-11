use std::{iter::Peekable, ops::Range, str::Chars};

pub type Span = Range<usize>;

pub struct Source<'a> {
    source: &'a str,
    chars: Peekable<Chars<'a>>,
    pub index: usize,
}

impl<'a> Source<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars().peekable(),
            index: 0,
        }
    }

    pub fn has_next(&mut self) -> bool {
        self.chars.peek().is_some()
    }

    pub fn peek(&mut self) -> char {
        *self.chars.peek().unwrap()
    }

    pub fn eat(&mut self) {
        let c = self.chars.next().unwrap();
        self.index += c.len_utf8();
    }

    pub fn str(&mut self, span: Span) -> &'a str {
        &self.source[span]
    }
}
