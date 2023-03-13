use super::mod_type::assembly::AssemblyCollectedModuleData;

#[derive(Debug)]
pub struct CollectedModule {
    pub src: String,
    pub exported_funcs: Vec<CollectedModuleFunction>,
    pub data: CollectedModuleData,
}

#[derive(Debug)]
pub struct CollectedModuleFunction {
    pub name: String,
    pub public: bool,
}

#[derive(Debug)]
pub enum CollectedModuleData {
    Assembly(AssemblyCollectedModuleData),
}
