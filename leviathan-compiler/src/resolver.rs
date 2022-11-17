use leviathan_common::structure::{Function, Structure};
use std::collections::{HashMap, HashSet, LinkedList};

pub struct Resolver<'a> {
    _function_pool: HashSet<Function>,
    _functions: HashMap<String, FunctionOverloader<'a>>,
    _types: HashMap<String, () /* TODO */>,
}

impl Resolver<'_> {
    pub fn new() -> Self {
        Self {
            _function_pool: HashSet::new(),
            _functions: HashMap::new(),
            _types: HashMap::new(),
        }
    }

    pub fn add(&mut self, _structure: Structure) {
        todo!()
    }
}

pub struct FunctionOverloader<'a>(LinkedList<&'a Function>);

impl<'a> FunctionOverloader<'a> {
    pub fn new() -> Self {
        Self(LinkedList::new())
    }
}
