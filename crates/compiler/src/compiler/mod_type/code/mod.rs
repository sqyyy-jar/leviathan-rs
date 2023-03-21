use crate::compiler::ModuleType;

pub struct CodeLanguage;

impl ModuleType for CodeLanguage {
    fn collect(
        &self,
        _task: &mut crate::compiler::CompileTask,
        _module_index: usize,
        _module: crate::compiler::UncollectedModule,
        _main: bool,
    ) -> crate::compiler::error::Result<()> {
        todo!()
    }

    fn gen_intermediary(
        &self,
        _task: &mut crate::compiler::CompileTask,
        _module_index: usize,
    ) -> crate::compiler::error::Result<()> {
        todo!()
    }
}
