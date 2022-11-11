mod error;
pub mod parser;
mod prelude;
pub mod util;

#[test]
pub fn test_panic() {
    let result = parser::Parser::parse(
        &r#"
# Test code goes here
"#
        .to_string(),
    );
    panic!("{:#?}", result);
}
