use compiler::CompileTask;
use parser::ast::build_ast;

use crate::parser::tokenizer::tokenize;

pub mod compiler;
pub mod parser;
pub mod util;

fn main() {
    let src = r#"
(mod assembly)
(scope abc 1)
"#
    .to_string();
    let src2 = r#"
(mod assembly)
(static buf
  (buffer 1024))
"#
    .to_string();
    let tokens = tokenize(src).unwrap();
    let tokens2 = tokenize(src2).unwrap();
    let ast = build_ast("testing".into(), tokens).unwrap();
    let ast2 = build_ast("testing2".into(), tokens2).unwrap();
    let mut task = CompileTask::default();
    task.include(ast).unwrap();
    task.include(ast2).unwrap();
    task.gen_intermediary().unwrap();
    dbg!(task);
}
