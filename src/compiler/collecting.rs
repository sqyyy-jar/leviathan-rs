use super::mod_type::assembly::AssemblyCollectedScope;

#[derive(Debug)]
pub struct CollectedModule {
    pub src: String,
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
    Assembly { scopes: Vec<AssemblyCollectedScope> },
    Code(()),
}
