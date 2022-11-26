use crate::parser::Token;
use logos::Lexer;

pub(crate) fn parse_int(lex: &mut Lexer<Token>) -> Option<i64> {
    let slice = lex.slice();
    slice.parse().ok()
}

pub(crate) fn parse_float(lex: &mut Lexer<Token>) -> Option<f64> {
    let slice = lex.slice();
    slice.parse().ok()
}
