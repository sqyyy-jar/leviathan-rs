use crate::Span;

pub struct UpperLayer {
    pub block: CodeBlock,
}

pub struct CodeBlock {
    pub span: Span,
    pub elements: Vec<CodeElement>,
}

pub enum CodeElement {
    If {
        span: Span,
        cond: Condition,
        block: CodeBlock,
    },
    While {
        span: Span,
        cond: Condition,
        block: CodeBlock,
    },
    For {},
    Let {},
    Return {
        span: Span,
        expr: Option<Expr>,
    },
    Assign {
        span: Span,
        expr: Expr,
    },
    Call {},
}

pub enum Condition {
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
        a: Box<Condition>,
        b: Box<Condition>,
    },
    Or {
        span: Span,
        a: Box<Condition>,
        b: Box<Condition>,
    },
}

pub enum Expr {
    Static {},
    Variable {},
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
    Addition {
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Subtraction {
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Multiplication {
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Division {
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Call {},
}
