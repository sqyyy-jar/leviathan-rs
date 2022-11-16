#![cfg_attr(not(feature = "parser_structure"), cfg(never))]

use std::collections::HashMap;

use crate::util::TextPosition;

#[derive(Debug)]
pub struct Structure {
    pub namespace: String,
    pub namespace_arguments: Vec<String>,
    pub imports: Vec<String>,
    pub functions: Vec<Function>,
}

impl Structure {
    pub fn new(
        namespace: String,
        namespace_arguments: Vec<String>,
        imports: Vec<String>,
        functions: Vec<Function>,
    ) -> Self {
        Self {
            namespace,
            namespace_arguments,
            imports,
            functions,
        }
    }
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<(String, String)>,
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
