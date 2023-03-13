use super::collecting::CollectedModuleFunction;

#[derive(Debug)]
pub struct IntermediaryModule {
    pub src: String,
    pub exported_funcs: Vec<CollectedModuleFunction>,
    pub dependencies: Vec<IntermediaryDependencyPath>,
    pub funcs: Vec<IntermediaryFunction>,
}

#[derive(Debug)]
pub struct IntermediaryDependencyPath {
    pub module_index: usize,
    pub export_index: usize,
}

#[derive(Debug)]
pub enum IntermediaryFunction {
    Public { export_index: usize, ir: () },
    Private { name: String, ir: () },
}
