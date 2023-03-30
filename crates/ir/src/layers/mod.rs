use crate::Span;

use self::upper::Expr;

pub mod destructure;
pub mod error;
pub mod lower;
pub mod upper;

#[derive(Debug)]
pub struct Coord {
    pub module: usize,
    pub element: usize,
}

#[derive(Debug)]
pub enum NextCoord {
    Success,
    Failure,
    Unknown,
}

#[derive(Debug)]
pub enum Type {
    Unit,
    Int,
    UInt,
    Float,
    String,
}

#[derive(Debug)]
pub enum CompareType {
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
}

impl CompareType {
    pub fn inverted(&self) -> Self {
        match self {
            Self::Equal => Self::NotEqual,
            Self::NotEqual => Self::Equal,
            Self::Less => Self::GreaterEqual,
            Self::Greater => Self::LessEqual,
            Self::LessEqual => Self::Greater,
            Self::GreaterEqual => Self::Less,
        }
    }
}

#[derive(Debug)]
pub enum BinaryOpType {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    BitAnd,
    BitOr,
    BitXor,
    ShiftLeft,
    ShiftRight,
}

impl BinaryOpType {
    pub fn constructor(&self) -> fn(Span, Box<Expr>, Box<Expr>) -> Expr {
        match self {
            BinaryOpType::Add => |span, left, right| Expr::Add { span, left, right },
            BinaryOpType::Sub => |span, left, right| Expr::Sub { span, left, right },
            BinaryOpType::Mul => |span, left, right| Expr::Mul { span, left, right },
            BinaryOpType::Div => |span, left, right| Expr::Div { span, left, right },
            BinaryOpType::Rem => |span, left, right| Expr::Rem { span, left, right },
            BinaryOpType::BitAnd => |span, left, right| Expr::BitAnd { span, left, right },
            BinaryOpType::BitOr => |span, left, right| Expr::BitOr { span, left, right },
            BinaryOpType::BitXor => |span, left, right| Expr::BitXor { span, left, right },
            BinaryOpType::ShiftLeft => |span, left, right| Expr::ShiftLeft { span, left, right },
            BinaryOpType::ShiftRight => |span, left, right| Expr::ShiftRight { span, left, right },
        }
    }
}

#[cfg(test)]
mod test {
    #![allow(unused)]

    use std::{
        fmt::Debug,
        io::{stderr, Write},
    };

    use crate::layers::upper::{Cond, Expr, Stmnt};

    use super::upper::{Block, UpperLayer};

    fn block(elements: Vec<Stmnt>) -> Block {
        Block {
            span: 0..0,
            elements,
        }
    }

    fn r#while(cond: Cond, block: Block) -> Stmnt {
        Stmnt::While {
            span: 0..0,
            cond,
            block,
        }
    }

    fn r#if(cond: Cond, block: Block) -> Stmnt {
        Stmnt::If {
            span: 0..0,
            cond,
            block,
        }
    }

    fn or(left: Cond, right: Cond) -> Cond {
        Cond::Or {
            span: 0..0,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    fn equal(left: Expr, right: Expr) -> Cond {
        Cond::NotEqual {
            span: 0..0,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    fn not_equal(left: Expr, right: Expr) -> Cond {
        Cond::NotEqual {
            span: 0..0,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    fn num(value: i64) -> Expr {
        Expr::Int { span: 0..0, value }
    }

    fn add(left: Expr, right: Expr) -> Expr {
        Expr::Add {
            span: 0..0,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    fn sub(left: Expr, right: Expr) -> Expr {
        Expr::Sub {
            span: 0..0,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    fn mul(left: Expr, right: Expr) -> Expr {
        Expr::Mul {
            span: 0..0,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    fn div(left: Expr, right: Expr) -> Expr {
        Expr::Div {
            span: 0..0,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    fn rem(left: Expr, right: Expr) -> Expr {
        Expr::Rem {
            span: 0..0,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    fn debug(value: &impl Debug) {
        let _ = stderr().write_fmt(format_args!("{value:#?}\n"));
    }

    #[test]
    fn test_debug() {
        let layer = UpperLayer::Block {
            vars: vec![],
            block: block(vec![r#if(
                or(not_equal(num(1), num(2)), not_equal(num(1), num(3))),
                block(vec![Stmnt::Assign {
                    span: 0..0,
                    index: 0,
                    expr: mul(num(21), num(2)),
                }]),
            )]),
        };
        let layer = layer.destructure().const_eval();
        debug(&layer);
    }
}
