use std::fs::File;

use compiler::CompileTask;
use parser::ast::build_ast;

use crate::parser::tokenizer::tokenize;

pub mod compiler;
pub mod parser;
pub mod util;

fn main() {
    let src = include_str!("../main.lvt");
    let src2 = r#"
(mod assembly)
(static buf
  (buffer 16))
"#;
    let tokens = tokenize("main".into(), "main.lvt".into(), src.to_string())
        .unwrap_or_else(|err| err.abort());
    let tokens2 = tokenize("other".into(), "other.lvt".into(), src2.to_string())
        .unwrap_or_else(|err| err.abort());
    let ast = build_ast(tokens).unwrap_or_else(|err| err.abort());
    let ast2 = build_ast(tokens2).unwrap_or_else(|err| err.abort());
    let mut task = CompileTask::default();
    task.include(ast, true).unwrap_or_else(|err| err.abort());
    task.include(ast2, false).unwrap_or_else(|err| err.abort());
    task.compile().unwrap_or_else(|err| err.abort());
    task.filter().unwrap_or_else(|err| err.abort());
    let mut file = File::create("out.bin").unwrap();
    task.assemble(&mut file).unwrap_or_else(|err| err.abort());
    dbg!(task);
}
