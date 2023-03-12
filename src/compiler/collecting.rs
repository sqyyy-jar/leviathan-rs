#[derive(Debug)]
pub struct CollectedModule {
    pub name: String,
    pub src: String,
    pub exported_funcs: Vec<CollectedModuleFunctionExport>,
    pub data: CollectedModuleData,
}

#[derive(Debug)]
pub struct CollectedModuleFunctionExport {
    pub name: String,
}

#[derive(Debug)]
pub enum CollectedModuleData {
    Assembly {},
}
