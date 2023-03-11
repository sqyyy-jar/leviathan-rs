use crate::util::source::{Source, Span};

use super::{
    error::{Error, Result},
    Token,
};

pub fn tokenize(source: &str) -> Result<Vec<Token>> {
    let mut source = Source::new(source);
    let mut tokens = Vec::new();
    let mut g_index = 0;
    let mut g_len = 0;
    let mut g_string = false;
    while source.has_next() {
        let c = source.peek();
        match c {
            '(' => {
                let index = source.index;
                source.eat();
                if g_len > 0 || g_string {
                    return Err(Error::NoWhitespaceBetweenTokens {
                        span: index..source.index,
                    });
                }
                tokens.push(Token::LeftParen {
                    span: index..source.index,
                });
            }
            ')' => {
                let index = source.index;
                if g_len > 0 {
                    tokens.push(parse_token(g_index..g_index + g_len, &mut source)?);
                    g_len = 0;
                }
                source.eat();
                tokens.push(Token::RightParen {
                    span: index..source.index,
                });
            }
            '"' => {
                let mut buf = String::with_capacity(0);
                let s_index = source.index;
                source.eat();
                let s_value_index = source.index;
                let mut s_value_len = 0;
                while source.has_next() {
                    let c = source.peek();
                    match c {
                        '"' => {
                            break;
                        }
                        '\\' => {
                            s_value_len += c.len_utf8();
                            let index = source.index;
                            source.eat();
                            if !source.has_next() {
                                return Err(Error::UnexpectedEndOfSource {
                                    span: index..source.index,
                                });
                            }
                            let esc_c = source.peek();
                            source.eat();
                            match esc_c {
                                '"' | '\\' => {
                                    buf.push(esc_c);
                                    s_value_len += esc_c.len_utf8();
                                }
                                'n' => {
                                    buf.push('\n');
                                    s_value_len += 1;
                                }
                                't' => {
                                    buf.push('\t');
                                    s_value_len += 1;
                                }
                                'r' => {
                                    buf.push('\r');
                                    s_value_len += 1;
                                }
                                'x' => {
                                    todo!("\\x")
                                }
                                _ => {
                                    return Err(Error::InvalidStringEscapeCode {
                                        span: index..source.index,
                                    });
                                }
                            }
                        }
                        _ => {
                            buf.push(c);
                            source.eat();
                            s_value_len += c.len_utf8();
                        }
                    }
                }
                if !source.has_next() {
                    return Err(Error::UnexpectedEndOfSource {
                        span: s_index..s_value_index + s_value_len,
                    });
                }
                source.eat();
                let s_len = c.len_utf8() * 2 + s_value_len;
                tokens.push(Token::String {
                    span: s_index..s_index + s_len,
                    value: buf,
                });
                g_string = true;
            }
            _ => {
                if c.is_whitespace() {
                    if g_len > 0 {
                        tokens.push(parse_token(g_index..g_index + g_len, &mut source)?);
                    }
                    g_len = 0;
                    source.eat();
                    continue;
                }
                if g_string {
                    let index = source.index;
                    source.eat();
                    return Err(Error::NoWhitespaceBetweenTokens {
                        span: index..source.index,
                    });
                }
                if g_len == 0 {
                    g_index = source.index;
                }
                source.eat();
                g_len += c.len_utf8();
            }
        }
    }
    Ok(tokens)
}

fn parse_token(span: Span, source: &mut Source) -> Result<Token> {
    let s = source.str(span.clone());
    if let Some(s) = s.strip_suffix('u') {
        if let Ok(value) = s.parse() {
            return Ok(Token::UInt { span, value });
        }
    }
    if let Ok(value) = s.parse() {
        return Ok(Token::Int { span, value });
    }
    if let Ok(value) = s.parse() {
        return Ok(Token::Float { span, value });
    }
    if s.chars().next().unwrap().is_ascii_digit() {
        return Err(Error::IdentStartingWithDigit { span });
    }
    Ok(Token::Ident { span })
}