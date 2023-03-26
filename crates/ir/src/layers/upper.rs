use crate::Span;

use super::{ModuleCoord, Type};

pub enum UpperLayer {
    Expr { expr: Expr },
    Block { vars: Vec<Type>, block: CodeBlock },
}

pub struct CodeBlock {
    pub span: Span,
    pub elements: Vec<Stmnt>,
}

pub enum Stmnt {
    If {
        span: Span,
        cond: Cond,
        block: CodeBlock,
    },
    While {
        span: Span,
        cond: Cond,
        block: CodeBlock,
    },
    For {
        span: Span,
        // TODO
    },
    Let {
        span: Span,
        name: Span,
        expr: Expr,
    },
    Return {
        span: Span,
        expr: Option<Expr>,
    },
    Assign {
        span: Span,
        expr: Expr,
    },
    Call {
        span: Span,
        coord: ModuleCoord,
        params: Vec<Expr>,
    },
}

pub enum Expr {
    Static {
        span: Span,
        coord: ModuleCoord,
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
        coord: ModuleCoord,
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
