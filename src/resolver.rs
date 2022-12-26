use std::collections::HashMap;

pub struct Resolver {
    root: HashMap<String, Node>,
}

pub enum Node {
    Branch(HashMap<String, Node>),
    Overload(),
}
