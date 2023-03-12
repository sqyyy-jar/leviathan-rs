use crate::{
    compiler::{
        collecting::{CollectedModule, CollectedModuleData},
        error::Result,
        ModuleType,
    },
    parser::{BareModule, Node},
};

pub struct Assembly;

impl ModuleType for Assembly {
    fn collect(&self, BareModule { name, src, root }: BareModule) -> Result<CollectedModule> {
        let mut nodes = root.into_iter();
        nodes.next().unwrap();
        let mut exported_funcs = Vec::with_capacity(0);
        for node in nodes {
            let Node::Node { span, sub_nodes } = node else {
                panic!("Invalid AST");
            };
        }
        Ok(CollectedModule {
            name,
            src,
            exported_funcs,
            data: CollectedModuleData::Assembly {},
        })
    }
}
