use std::{iter::Peekable, ops::Range, str::Chars};

#[derive(Debug)]
pub enum Error {
    InvalidFloatingPointNumber(),
    NonAsciiWhitespace(),
    UnexpectedEOF(),
}

#[derive(Debug)]
pub struct Token {
    pub line: usize,
    pub column: usize,
    pub chars: Range<usize>,
    pub token_type: TokenType,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Semicolon,
    True,
    False,
    Int,
    Float,
    String,
    Atom,
    Ident,
}

struct State<'a> {
    source: Peekable<Chars<'a>>,
    line: usize,
    column: usize,
    index: usize,
    tokens: Vec<Token>,
}

impl State<'_> {
    fn push_single(&mut self, token_type: TokenType) {
        let index = self.index;
        let column = self.column;
        self.advance();
        self.tokens.push(Token {
            line: self.line,
            column,
            chars: index..self.index,
            token_type,
        });
    }

    fn newline(&mut self) {
        self.line += 1;
        self.advance();
        self.column = 1;
    }

    fn peek(&mut self) -> Result<char, Error> {
        let Some(c) = self.source.peek().cloned() else {
            return Err(Error::UnexpectedEOF());
        };
        Ok(c)
    }

    fn advance(&mut self) {
        self.column += 1;
        self.index += self.source.next().unwrap().len_utf8();
    }

    fn number(
        &mut self,
        start_index: usize,
        start_column: usize,
        mut buf: String,
        sign: bool,
        mut float: bool,
    ) -> Result<(), Error> {
        if !sign {
            let Some(c) = self.source.peek().cloned() else {
                return Err(Error::UnexpectedEOF());
            };
            if c == '-' {
                buf.push(c);
                self.advance();
            }
        }
        while let Some(c) = self.source.peek().cloned() {
            match c {
                '0'..='9' => {
                    buf.push(c);
                    self.advance();
                }
                '.' => {
                    if float {
                        break;
                    }
                    buf.push(c);
                    float = true;
                    self.advance();
                }
                _ => {
                    break;
                }
            }
        }
        self.tokens.push(Token {
            line: self.line,
            column: start_column,
            chars: start_index..self.index,
            token_type: if float {
                TokenType::Float
            } else {
                TokenType::Int
            },
        });
        Ok(())
    }

    fn atom(&mut self) -> Result<(), Error> {
        let start_index = self.index;
        let start_column = self.column;
        let mut len = 0;
        while let Some(c) = self.source.peek().cloned() {
            match c {
                '(' | ')' | '[' | ']' | '{' | '}' => {
                    break;
                }
                c => {
                    if c.is_ascii_whitespace() || c == ',' {
                        break;
                    }
                    if c.is_whitespace() {
                        return Err(Error::NonAsciiWhitespace());
                    }
                    self.advance();
                    len += 1;
                }
            }
        }
        self.tokens.push(Token {
            line: self.line,
            column: start_column,
            chars: start_index..self.index,
            token_type: if len != 1 {
                TokenType::Atom
            } else {
                TokenType::Ident
            },
        });
        Ok(())
    }

    fn ident(
        &mut self,
        start_index: usize,
        start_column: usize,
        mut buf: String,
    ) -> Result<(), Error> {
        while let Some(c) = self.source.peek().cloned() {
            match c {
                '(' | ')' | '[' | ']' | '{' | '}' => {
                    break;
                }
                c => {
                    if c.is_ascii_whitespace() || c == ',' {
                        break;
                    }
                    if c.is_whitespace() {
                        return Err(Error::NonAsciiWhitespace());
                    }
                    self.advance();
                    buf.push(c);
                }
            }
        }
        self.tokens.push(Token {
            line: self.line,
            column: start_column,
            chars: start_index..self.index,
            token_type: match buf.as_str() {
                "true" => TokenType::True,
                "false" => TokenType::False,
                _ => TokenType::Ident,
            },
        });
        Ok(())
    }
}

pub fn parse(source: &str) -> Result<Vec<Token>, Error> {
    let mut state = State {
        source: source.chars().peekable(),
        line: 1,
        column: 1,
        index: 0,
        tokens: Vec::new(),
    };
    while let Some(c) = state.source.peek().cloned() {
        match c {
            '(' => {
                state.push_single(TokenType::LeftParen);
            }
            ')' => {
                state.push_single(TokenType::RightParen);
            }
            '[' => {
                state.push_single(TokenType::LeftBracket);
            }
            ']' => {
                state.push_single(TokenType::RightBracket);
            }
            '{' => {
                state.push_single(TokenType::LeftBrace);
            }
            '}' => {
                state.push_single(TokenType::RightBrace);
            }
            '\n' => {
                state.newline();
            }
            ':' => {
                state.atom()?;
            }
            ';' => {
                state.push_single(TokenType::Semicolon);
            }
            '0'..='9' => {
                state.number(state.index, state.column, String::new(), false, false)?;
            }
            '-' => 'block: {
                let start_index = state.index;
                let start_column = state.column;
                state.advance();
                let Ok(a) = state.peek() else {
                    state.ident(start_index, start_column, String::from("-"))?;
                    break 'block;
                };
                if a.is_ascii_digit() {
                    state.number(start_index, start_column, String::from("-"), true, false)?;
                    break 'block;
                }
                if a == '.' {
                    state.advance();
                    let Ok(b) = state.peek() else {
                        state.ident(start_index, start_column, String::from("-."))?;
                        break 'block;
                    };
                    if b.is_ascii_digit() {
                        state.number(start_index, start_column, String::from("-."), true, true)?;
                        break 'block;
                    }
                    state.ident(start_index, start_column, String::from("-."))?;
                    break 'block;
                }
                state.ident(start_index, start_column, String::from("-"))?;
            }
            '.' => 'block: {
                let start_index = state.index;
                let start_column = state.column;
                state.advance();
                let Ok(a) = state.peek() else {
                    state.ident(start_index, start_column, String::from("."))?;
                    break 'block;
                };
                if a.is_ascii_digit() {
                    state.number(start_index, start_column, String::from("."), false, true)?;
                    break 'block;
                }
                state.ident(start_index, start_column, String::from("."))?;
            }
            '"' => {
                todo!("String")
            }
            w if w.is_ascii_whitespace() || w == ',' => {
                state.advance();
            }
            c => {
                if c.is_ascii_whitespace() || c == ',' {
                    state.advance();
                    continue;
                }
                if c.is_whitespace() {
                    return Err(Error::NonAsciiWhitespace());
                }
                state.ident(state.index, state.column, String::new())?;
            }
        }
    }
    Ok(state.tokens)
}
