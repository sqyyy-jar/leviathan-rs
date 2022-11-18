use leviathan_common::{
    prelude::*,
    structure::{Function, Structure},
    util::NamespacedTree,
};
use std::collections::LinkedList;

pub struct Resolver {
    tree: NamespacedTree<FunctionOverloader>,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            tree: NamespacedTree::new(),
        }
    }

    pub fn load_structure(&mut self, structure: Structure) -> Result<()> {
        let Structure {
            namespace,
            imports,
            functions,
        } = structure;
        if namespace.packages.len() < 1 {
            return Err(Error::Generic(
                "Unable to resolve structure with invalid namespace".into(),
            ));
        }
        self.tree.get_mut(&namespace.packages[0]);
        Ok(())
    }
}

pub struct FunctionOverloader(LinkedList<Function>);

impl FunctionOverloader {
    pub fn new() -> Self {
        Self(LinkedList::new())
    }
}
