use super::mod_type::assembly::{AssemblyCollectedScope, AssemblyCollectedStatic};

#[derive(Debug)]
pub struct CollectedModule {
    pub funcs: Vec<CollectedFunction>,
    pub data: CollectedModuleData,
}

#[derive(Debug)]
pub struct CollectedFunction {
    pub name: String,
    pub public: bool,
}

#[derive(Debug)]
pub enum CollectedModuleData {
    Assembly {
        statics: Vec<AssemblyCollectedStatic>,
        scopes: Vec<AssemblyCollectedScope>,
    },
    Code(()),
}
