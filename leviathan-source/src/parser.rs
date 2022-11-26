use crate::callbacks;
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[regex(r"\(")]
    SymbolLeftParen,

    #[regex(r"\)")]
    SymbolRightParen,

    #[regex(r"\[")]
    SymbolLeftBracket,

    #[regex(r"]")]
    SymbolRightBracket,

    #[regex(r"\{")]
    SymbolLeftBrace,

    #[regex(r"}")]
    SymbolRightBrace,

    #[regex(r"[0-9]+", callbacks::parse_int)]
    Int(i64),

    #[regex(r"([0-9]+\.[0-9]*)|(\.[0-9]+)", callbacks::parse_float)]
    Float(f64),

    #[regex(r#"[^\s0-9(){}\[\],":][^\s(){}\[\],"]*"#)]
    Ident,

    #[regex(r#":[^\s(){}\[\],"]+"#)]
    Atom,

    #[regex(r#""([^"]|\\")*""#)]
    String,

    #[error]
    #[regex(r"[ \t\n\f,]+", logos::skip)]
    Error,
}
