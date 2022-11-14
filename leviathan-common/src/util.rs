use std::{collections::HashMap, fmt::Display};

#[derive(Clone, Copy, Debug)]
pub struct TextPosition {
    pub line: u32,
    pub column: u32,
}

impl Display for TextPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}:{}", self.line, self.column).as_str())
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    pub position: TextPosition,
    pub value: NodeType,
}

#[derive(Clone, Debug)]
pub enum NodeType {
    Node {
        operator: String,
        arguments: Vec<Node>,
    },
    List(Vec<Node>),
    Map(HashMap<String, Node>),
    Identifier(String),
    Atom(String),
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    Comment(String),
}
