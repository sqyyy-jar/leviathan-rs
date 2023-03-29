pub mod destructure;
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

#[cfg(test)]
mod test {
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

    #[test]
    fn test() {
        let layer = UpperLayer::Block {
            vars: vec![],
            block: block(vec![r#if(
                or(not_equal(num(1), num(2)), not_equal(num(1), num(3))),
                block(vec![Stmnt::Assign {
                    span: 0..0,
                    index: 0,
                    expr: Expr::Int {
                        span: 0..0,
                        value: 0,
                    },
                }]),
            )]),
        };
        let layer = layer.destructure();
        dbg!(layer);
    }
}
