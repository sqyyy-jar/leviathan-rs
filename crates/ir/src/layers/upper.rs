use crate::Span;

use super::{Coord, Type};

pub enum UpperLayer {
    Expr { expr: Expr },
    Block { vars: Vec<Var>, block: Block },
}

pub struct Var {
    pub type_: Type,
}

pub struct Block {
    pub span: Span,
    pub elements: Vec<Stmnt>,
}

pub enum Stmnt {
    If {
        span: Span,
        cond: Cond,
        block: Block,
    },
    While {
        span: Span,
        cond: Cond,
        block: Block,
    },
    For {
        span: Span,
        // TODO
    },
    Let {
        span: Span,
        index: usize,
        expr: Expr,
    },
    Return {
        span: Span,
        expr: Option<Expr>,
    },
    Assign {
        span: Span,
        index: usize,
        expr: Expr,
    },
    Call {
        span: Span,
        coord: Coord,
        params: Vec<Expr>,
    },
}

pub enum Expr {
    Static {
        span: Span,
        coord: Coord,
    },
    Variable {
        span: Span,
        index: usize,
    },
    Int {
        span: Span,
        value: i64,
    },
    UInt {
        span: Span,
        value: u64,
    },
    Float {
        span: Span,
        value: f64,
    },
    String {
        span: Span,
        value: String,
    },
    Add {
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Sub {
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Mul {
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Div {
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Rem {
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    BitAnd {
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    BitOr {
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    BitXor {
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    ShiftLeft {
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    ShiftRight {
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    SignedShiftRight {
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    BitNot {
        span: Span,
        expr: Box<Expr>,
    },
    Call {
        span: Span,
        coord: Coord,
        params: Vec<Expr>,
    },
}

pub enum Cond {
    Equal {
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    NotEqual {
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Less {
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Greater {
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    LessEqual {
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    GreaterEqual {
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    And {
        span: Span,
        a: Box<Cond>,
        b: Box<Cond>,
    },
    Or {
        span: Span,
        a: Box<Cond>,
        b: Box<Cond>,
    },
}
