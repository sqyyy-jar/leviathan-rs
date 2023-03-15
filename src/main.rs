use compiler::CompileTask;
use parser::ast::build_ast;

use crate::parser::tokenizer::tokenize;

pub mod compiler;
pub mod parser;
pub mod util;

fn main() {
    let src = r#"
(mod assembly)
(scope abc 1
"#;
    let src2 = r#"
(mod assembly)
(static buf
  (buffer 1024))
"#;
    let tokens = tokenize(src.to_string()).unwrap_or_else(|err| err.abort("main.lvt"));
    let tokens2 = tokenize(src2.to_string()).unwrap_or_else(|err| err.abort("main.lvt"));
    let ast = build_ast("testing".into(), tokens).unwrap_or_else(|err| err.abort("main.lvt"));
    let ast2 = build_ast("testing2".into(), tokens2).unwrap_or_else(|err| err.abort("main.lvt"));
    let mut task = CompileTask::default();
    task.include(ast).unwrap();
    task.include(ast2).unwrap();
    task.gen_intermediary().unwrap();
    dbg!(task);
}
