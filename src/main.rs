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
  (buffer 1024))
"#;
    let tokens = tokenize(src.to_string()).unwrap_or_else(|err| err.abort("main.lvt"));
    let tokens2 = tokenize(src2.to_string()).unwrap_or_else(|err| err.abort("other.lvt"));
    let ast = build_ast("main.lvt".into(), "main".into(), tokens)
        .unwrap_or_else(|err| err.abort("main.lvt"));
    let ast2 = build_ast("other.lvt".into(), "other".into(), tokens2)
        .unwrap_or_else(|err| err.abort("main.lvt"));
    let mut task = CompileTask::default();
    task.include(ast, true).unwrap_or_else(|err| err.abort());
    task.include(ast2, false).unwrap_or_else(|err| err.abort());
    task.gen_intermediary().unwrap_or_else(|err| err.abort());
    dbg!(task);
}
