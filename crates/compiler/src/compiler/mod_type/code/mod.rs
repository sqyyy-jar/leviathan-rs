use std::mem;

use crate::{
    compiler::{
        error::{Error, Result},
        CompileTask, Module, ModuleType, UncollectedModule,
    },
    parser::Node,
};

pub struct CodeLanguage;

impl ModuleType for CodeLanguage {
    fn collect(
        &self,
        _task: &mut CompileTask,
        _module_index: usize,
        UncollectedModule { root }: UncollectedModule,
        _main: bool,
    ) -> Result<()> {
        let mut module = &mut _task.modules[_module_index];
        for stmnt in root {
            let Node::Node {
                span: _stmnt_span,
                sub_nodes: _stmnt_nodes,
            } = stmnt else
            {
                return Err(Error::UnexpectedToken {
                    file: take_file(module),
                    src: take_src(module),
                    span: stmnt.span(),
                });
            };
        }
        todo!()
    }

    fn gen_intermediary(&self, _task: &mut CompileTask, _module_index: usize) -> Result<()> {
        todo!()
    }
}

fn take_file(module: &mut Module) -> String {
    mem::replace(&mut module.file, String::with_capacity(0))
}

fn take_src(module: &mut Module) -> String {
    mem::replace(&mut module.src, String::with_capacity(0))
}
