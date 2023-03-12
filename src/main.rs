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
(+scope def 2)
"#
    .to_string();
    let tokens = dbg!(tokenize(src).unwrap());
    let tokens2 = dbg!(tokenize(src2).unwrap());
    let ast = dbg!(build_ast("testing".into(), tokens).unwrap());
    let ast2 = dbg!(build_ast("testing".into(), tokens2).unwrap());
    let mut task = CompileTask::default();
    task.include(ast).unwrap();
    task.include(ast2).unwrap();
    dbg!(task);
}
