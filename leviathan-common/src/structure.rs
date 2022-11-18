#![cfg_attr(not(feature = "parser_structure"), cfg(never))]

use crate::util::TextPosition;
use std::{collections::HashMap, fmt::Debug};

#[derive(Debug)]
pub struct Structure {
    pub namespace: Namespace,
    pub imports: Vec<String>,
    pub functions: Vec<Function>,
}

impl Structure {
    pub fn new(namespace: Namespace, imports: Vec<String>, functions: Vec<Function>) -> Self {
        Self {
            namespace,
            imports,
            functions,
        }
    }
}

#[derive(Debug)]
pub struct Namespace {
    pub packages: Vec<String>,
    pub tags: Vec<String>,
}

impl Namespace {
    pub fn new() -> Self {
        Self {
            packages: Vec::with_capacity(0),
            tags: Vec::with_capacity(0),
        }
    }
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<(String, String)>,
    pub return_type: Type,
    pub tags: Vec<Expression>,
    pub code: Expression,
}

#[derive(Debug)]
pub struct Expression {
    pub position: TextPosition,
    pub value: ExpressionType,
}

#[derive(Debug)]
pub enum ExpressionType {
    Invoke {
        operator: String,
        arguments: Vec<Expression>,
    },
    List(Vec<Expression>),
    Map(HashMap<String, Expression>),
    Identifier(String),
    Atom(String),
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    Comment(String),
}

pub enum Type {
    Unit,
    Bool,
    Int,
    Float,
    String,
    Atom,
    List,
    Map,
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unit => write!(f, "unit"),
            Self::Bool => write!(f, "bool"),
            Self::Int => write!(f, "int"),
            Self::Float => write!(f, "float"),
            Self::String => write!(f, "string"),
            Self::Atom => write!(f, "atom"),
            Self::List => write!(f, "list"),
            Self::Map => write!(f, "map"),
        }
    }
}
