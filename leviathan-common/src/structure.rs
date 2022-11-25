#![cfg_attr(not(feature = "parser_structure"), cfg(never))]

use crate::{prelude::Error, util::TextPosition};
use std::{collections::HashMap, fmt::Debug, str::FromStr};

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

pub struct Source {
    pub name: String,
    pub functions: Vec<Function>,
}

impl Source {
    pub fn new(name: String, functions: Vec<Function>) -> Self {
        Self { name, functions }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Namespace(pub Vec<String>);

impl Namespace {
    pub fn new() -> Self {
        Self(Vec::with_capacity(0))
    }

    pub fn merge(&mut self, mut other: Self) {
        self.0.append(&mut other.0);
    }

    pub fn clone_with_package(&self, package: &Namespace) -> Self {
        let mut result = self.clone();
        for package in &package.0 {
            result.0.push(package.clone());
        }
        result
    }

    pub fn clone_merge(&self, other: &Namespace) -> Self {
        let mut result = Self(self.0.clone());
        for package in &other.0 {
            result.0.push(package.clone());
        }
        result
    }
}

impl FromStr for Namespace {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Self::new();
        for namespace_package in s
            .split_terminator('/')
            .filter(|it| !it.is_empty())
            .map(str::to_string)
        {
            result.0.push(namespace_package);
        }
        Ok(result)
    }
}

#[derive(Debug)]
pub struct Function {
    pub name: Namespace,
    pub arguments: Vec<(String, Type)>,
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
        operator: Namespace,
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

impl FromStr for Type {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "unit" => Ok(Self::Unit),
            "bool" => Ok(Self::Bool),
            "int" => Ok(Self::Int),
            "float" => Ok(Self::Float),
            "string" => Ok(Self::String),
            "atom" => Ok(Self::Atom),
            "list" => Ok(Self::List),
            "map" => Ok(Self::Map),
            _ => Err(Error::Generic(format!("Unknown type '{}'", s))),
        }
    }
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
