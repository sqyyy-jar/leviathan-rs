use crate::parser::tokenizer::tokenize;

pub mod parser;
pub mod util;

fn main() {
    dbg!(tokenize(
r#"
1 2. 3u
"#
    )).unwrap();
}
