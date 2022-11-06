pub mod parser;
pub mod util;

#[test]
pub fn test_panic() {
    let result = parser::Parser::parse(
        &r#"
(+ -1.0 2.0 -3 4)
"#
        .to_string(),
    );
    panic!("{:#?}", result);
}
