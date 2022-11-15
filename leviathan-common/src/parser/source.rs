#![cfg_attr(not(feature = "parser_source"), cfg(never))]

use crate::util::TextPosition;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Clone)]
pub struct Node {
    pub position: TextPosition,
    pub value: NodeType,
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Node({})::{:#?}", &self.position, &self.value))
    }
}

#[derive(Clone)]
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

impl Debug for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Node {
                operator,
                arguments,
            } => f
                .debug_struct("Node")
                .field("operator", operator)
                .field("arguments", arguments)
                .finish(),
            Self::List(value) => f.write_fmt(format_args!("List {:#?}", value)),
            Self::Map(value) => f.write_fmt(format_args!("Map {:#?}", value)),
            Self::Identifier(value) => f.write_fmt(format_args!("Identifier({:?})", value)),
            Self::Atom(value) => f.write_fmt(format_args!("Atom({:?})", value)),
            Self::String(value) => f.write_fmt(format_args!("String({:?})", value)),
            Self::Integer(value) => f.write_fmt(format_args!("Integer({:?})", value)),
            Self::Float(value) => f.write_fmt(format_args!("Float({:?})", value)),
            Self::Bool(value) => f.write_fmt(format_args!("Bool({:?})", value)),
            Self::Comment(value) => f.write_fmt(format_args!("Comment({:?})", value)),
        }
    }
}
