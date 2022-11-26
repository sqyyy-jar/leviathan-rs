pub mod callbacks;
pub mod error;
pub mod grouper;
pub mod parser;
pub mod source;
pub mod state;

#[test]
pub fn test() {
    use logos::Logos;

    let source = r#"
(fn test {:x, :y} "abc")
"#;
    let lexer = parser::Token::lexer(source);
    unimplemented!("{:#?}", lexer.spanned().collect::<Vec<_>>())
}
