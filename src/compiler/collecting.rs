pub struct CollectedModule {
    pub name: String,
    pub exported_funcs: Vec<CollectedModuleFunctionExport>,
    pub data: CollectedModuleData,
}

pub struct CollectedModuleFunctionExport {
    pub name: String,
}

pub enum CollectedModuleData {
    Assembly {},
}
