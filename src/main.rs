use parser::ast::build_ast;

use crate::parser::tokenizer::tokenize;

pub mod compiler;
pub mod parser;
pub mod util;

fn main() {
    let tokens = dbg!(tokenize(
        r#"
(mod )
"#
    ))
    .unwrap();
    let _ast = dbg!(build_ast(tokens)).unwrap();
}
