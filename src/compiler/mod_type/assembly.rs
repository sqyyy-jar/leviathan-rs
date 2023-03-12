use crate::{
    compiler::{collecting::CollectedModule, error::Result, ModuleType},
    parser::Node,
};

pub struct Assembly;

impl ModuleType for Assembly {
    fn collect(&self, name: String, root: Vec<Node>) -> Result<CollectedModule> {
        todo!()
    }
}
