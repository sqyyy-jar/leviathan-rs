use compiler::CompileTask;
use parser::ast::build_ast;

use crate::parser::tokenizer::tokenize;

pub mod compiler;
pub mod parser;
pub mod util;

fn main() {
    let src = r#"
    (mod assembly)
    "#
    .to_string();
    let tokens = dbg!(tokenize(src).unwrap());
    let ast = dbg!(build_ast("testing".into(), tokens).unwrap());
    let mut task = CompileTask::default();
    task.include(ast).unwrap();
    dbg!(task);
}
