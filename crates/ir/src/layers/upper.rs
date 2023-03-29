use crate::{layers::destructure::Op, Span};

use super::{destructure::DestructureLayer, CompareType, Coord, NextCoord, Type};

#[derive(Debug)]
pub enum UpperLayer {
    Expr { expr: Expr },
    Block { vars: Vec<Var>, block: Block },
}

impl UpperLayer {
    pub fn destructure(self) -> DestructureLayer {
        let mut layer = DestructureLayer::default();
        match self {
            UpperLayer::Expr { expr } => {
                layer.ops.push(Op::Return { expr: Some(expr) });
            }
            UpperLayer::Block { vars, block } => {
                layer.vars = vars;
                block.expand(&mut layer);
            }
        }
        layer
    }
}

#[derive(Debug)]
pub struct Var {
    pub type_: Type,
}

#[derive(Debug)]
pub struct Block {
    pub span: Span,
    pub elements: Vec<Stmnt>,
}

impl Block {
    pub fn expand(self, layer: &mut DestructureLayer) {
        for element in self.elements {
            element.expand(layer);
        }
    }
}

#[derive(Debug)]
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

impl Stmnt {
    pub fn expand(self, layer: &mut DestructureLayer) {
        match self {
            Stmnt::If { cond, block, .. } => {
                let success = layer.alloc_coord();
                let failure = layer.alloc_coord();
                cond.expand(layer, success, failure, NextCoord::Success);
                layer.put_coord(success);
                block.expand(layer);
                layer.put_coord(failure);
            }
            Stmnt::While { cond, block, .. } => {
                let check = layer.alloc_coord();
                let success = layer.alloc_coord();
                let failure = layer.alloc_coord();
                layer.branch(check);
                layer.put_coord(success);
                block.expand(layer);
                layer.put_coord(check);
                cond.expand(layer, success, failure, NextCoord::Failure);
                layer.put_coord(failure);
            }
            Stmnt::For { span: _ } => todo!(),
            Stmnt::Let { index, expr, .. } => layer.ops.push(Op::Let { index, expr }),
            Stmnt::Return { expr, .. } => layer.ops.push(Op::Return { expr }),
            Stmnt::Assign { index, expr, .. } => layer.ops.push(Op::Assign { index, expr }),
            Stmnt::Call { coord, params, .. } => layer.ops.push(Op::Call { coord, params }),
        }
    }
}

#[derive(Debug)]
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
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Sub {
        span: Span,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Mul {
        span: Span,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Div {
        span: Span,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Rem {
        span: Span,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    BitAnd {
        span: Span,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    BitOr {
        span: Span,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    BitXor {
        span: Span,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    ShiftLeft {
        span: Span,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    ShiftRight {
        span: Span,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    SignedShiftRight {
        span: Span,
        left: Box<Expr>,
        right: Box<Expr>,
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

#[derive(Debug)]
pub enum Cond {
    Equal {
        span: Span,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    NotEqual {
        span: Span,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Less {
        span: Span,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Greater {
        span: Span,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    LessEqual {
        span: Span,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    GreaterEqual {
        span: Span,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Not {
        span: Span,
        cond: Box<Cond>,
    },
    And {
        span: Span,
        left: Box<Cond>,
        right: Box<Cond>,
    },
    Or {
        span: Span,
        left: Box<Cond>,
        right: Box<Cond>,
    },
}

impl Cond {
    pub fn expand(
        self,
        layer: &mut DestructureLayer,
        success: usize,
        failure: usize,
        next: NextCoord,
    ) {
        match self {
            Cond::Not { cond, .. } => match next {
                NextCoord::Success => cond.expand(layer, failure, success, NextCoord::Failure),
                NextCoord::Failure => cond.expand(layer, failure, success, NextCoord::Success),
                NextCoord::Unknown => cond.expand(layer, failure, success, NextCoord::Unknown),
            },
            Cond::And { left, right, .. } => match next {
                NextCoord::Success => {
                    let next_success = layer.alloc_coord();
                    left.expand(layer, next_success, failure, NextCoord::Success);
                    layer.put_coord(next_success);
                    right.expand(layer, success, failure, NextCoord::Success);
                }
                NextCoord::Failure => {
                    let next_success = layer.alloc_coord();
                    left.expand(layer, next_success, failure, NextCoord::Success);
                    layer.put_coord(next_success);
                    if right.is_comparison() {
                        right.expand(layer, success, failure, NextCoord::Failure);
                    } else {
                        right.expand(layer, success, failure, NextCoord::Success);
                        layer.branch(success);
                    }
                }
                NextCoord::Unknown => {
                    let next_success = layer.alloc_coord();
                    left.expand(layer, next_success, failure, NextCoord::Success);
                    layer.put_coord(next_success);
                    right.expand(layer, success, failure, NextCoord::Success);
                    layer.branch(success);
                }
            },
            Cond::Or { left, right, .. } => match next {
                NextCoord::Success => {
                    left.expand(layer, success, failure, NextCoord::Failure);
                    // Compression
                    if right.is_comparison() {
                        right.expand(layer, success, failure, NextCoord::Success);
                    } else {
                        right.expand(layer, success, failure, NextCoord::Failure);
                        layer.branch(failure);
                    }
                }
                NextCoord::Failure => {
                    left.expand(layer, success, failure, NextCoord::Failure);
                    right.expand(layer, success, failure, NextCoord::Failure);
                }
                NextCoord::Unknown => {
                    left.expand(layer, success, failure, NextCoord::Failure);
                    right.expand(layer, success, failure, NextCoord::Failure);
                    layer.branch(failure);
                }
            },
            comparison => {
                let compare_type = comparison.compare_type().unwrap();
                let (left, right) = match comparison {
                    Cond::Equal { left, right, .. }
                    | Cond::NotEqual { left, right, .. }
                    | Cond::Less { left, right, .. }
                    | Cond::Greater { left, right, .. }
                    | Cond::LessEqual { left, right, .. }
                    | Cond::GreaterEqual { left, right, .. } => (left, right),
                    _ => unreachable!(),
                };
                match next {
                    NextCoord::Success => {
                        layer.branch_if(failure, compare_type.inverted(), *left, *right);
                    }
                    NextCoord::Failure => {
                        layer.branch_if(success, compare_type, *left, *right);
                    }
                    NextCoord::Unknown => {
                        layer.branch_if(success, compare_type, *left, *right);
                        layer.branch(failure);
                    }
                }
            }
        }
    }

    pub fn is_comparison(&self) -> bool {
        !matches!(self, Cond::Not { .. } | Cond::And { .. } | Cond::Or { .. })
    }

    pub fn compare_type(&self) -> Option<CompareType> {
        match self {
            Self::Equal { .. } => Some(CompareType::Equal),
            Self::NotEqual { .. } => Some(CompareType::NotEqual),
            Self::Less { .. } => Some(CompareType::Less),
            Self::Greater { .. } => Some(CompareType::Greater),
            Self::LessEqual { .. } => Some(CompareType::LessEqual),
            Self::GreaterEqual { .. } => Some(CompareType::GreaterEqual),
            _ => None,
        }
    }
}
