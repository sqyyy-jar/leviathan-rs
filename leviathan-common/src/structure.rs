#![cfg_attr(not(feature = "parser_structure"), cfg(never))]

use std::collections::HashMap;

pub struct Structure {
    _namespace: String,
    _imports: Vec<String>,
    _functions: Vec<Function>,
}

pub struct Function {
    _name: String,
    _arguments: Vec<(String, String)>,
    _code: Expression,
}

pub enum Expression {
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
