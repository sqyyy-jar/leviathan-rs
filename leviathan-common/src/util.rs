use crate::prelude::*;
use std::{
    collections::{HashMap, LinkedList},
    fmt::{Debug, Display},
};

#[derive(Clone, Copy)]
pub struct TextPosition {
    pub line: u32,
    pub column: u32,
}

impl TextPosition {
    pub fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }
}

impl Display for TextPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}:{}", self.line, self.column).as_str())
    }
}

impl Debug for TextPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("(line {}, column {})", self.line, self.column).as_str())
    }
}

pub struct Namespace(Vec<String>);

pub struct NamespacedTree<V: Sized> {
    root: HashMap<String, NamespacedTreeNode<V>>,
}

impl<V> NamespacedTree<V> {
    pub fn new() -> Self {
        Self {
            root: HashMap::new(),
        }
    }

    pub fn get(&self, key: &String) -> Option<&NamespacedTreeNode<V>> {
        self.root.get(key)
    }

    pub fn insert(
        &mut self,
        key: &String,
        value: NamespacedTreeNode<V>,
    ) -> Option<NamespacedTreeNode<V>> {
        self.root.insert(key.clone(), value)
    }

    pub fn get_mut(&mut self, key: &String) -> Option<&mut NamespacedTreeNode<V>> {
        self.root.get_mut(key)
    }

    pub fn insert_at(&mut self, namespace: &Namespace) -> Result<()> {
        let Some(first_key) = namespace.0.get(0) else {
            return Err(Error::InvalidNamespace);
        };
        if !self.root.contains_key(first_key) {
            self.root
                .insert(first_key.clone(), NamespacedTreeNode::empty_branch());
        }
        let Some(mut current) = self.root.get(first_key) else {
            return Err(Error::Generic("Unexpected error in util".into()));
        };
        let mut iter = namespace.0.iter();
        iter.next();
        for package in iter {
            match current {
                NamespacedTreeNode::Leaf { elements } => {

                }
                _ => {todo!()}
            }
        }
        todo!()
    }

    pub fn collect_vec(self) -> Vec<V> {
        let mut vec = Vec::with_capacity(0);
        for root_element in self.root {
            root_element.1.collect_vec(&mut vec);
        }
        vec
    }
}

pub enum NamespacedTreeNode<V: Sized> {
    Leaf {
        elements: LinkedList<V>,
    },
    TinyLeaf(V),
    Branch {
        elements: LinkedList<V>,
        subnodes: HashMap<String, NamespacedTreeNode<V>>,
    },
    BareBranch {
        subnodes: HashMap<String, NamespacedTreeNode<V>>,
    },
    BranchExtension {
        package: String,
        link: Box<NamespacedTreeNode<V>>,
    },
}

impl<V> NamespacedTreeNode<V> {
    pub fn empty_leaf() -> Self {
        Self::Leaf {
            elements: LinkedList::new(),
        }
    }

    pub fn tiny_leaf(value: V) -> Self {
        Self::TinyLeaf(value)
    }

    pub fn empty_branch() -> Self {
        Self::Branch {
            elements: LinkedList::new(),
            subnodes: HashMap::with_capacity(0),
        }
    }

    pub fn empty_bare_branch() -> Self {
        Self::BareBranch {
            subnodes: HashMap::with_capacity(0),
        }
    }

    pub fn branch_extension(package: String, link: NamespacedTreeNode<V>) -> Self {
        Self::BranchExtension {
            package,
            link: Box::new(link),
        }
    }

    pub fn collect_vec(self, vec: &mut Vec<V>) {
        match self {
            NamespacedTreeNode::Leaf { elements } => {
                elements.into_iter().for_each(|it| vec.push(it));
            }
            NamespacedTreeNode::TinyLeaf(element) => {
                vec.push(element);
            }
            NamespacedTreeNode::Branch { elements, subnodes } => {
                elements.into_iter().for_each(|it| vec.push(it));
                subnodes.into_iter().for_each(|it| it.1.collect_vec(vec));
            }
            NamespacedTreeNode::BareBranch { subnodes } => {
                subnodes.into_iter().for_each(|it| it.1.collect_vec(vec));
            }
            NamespacedTreeNode::BranchExtension { package: _, link } => {
                link.collect_vec(vec);
            }
        }
    }
}
