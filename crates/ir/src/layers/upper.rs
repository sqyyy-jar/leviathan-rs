use crate::{
    layers::{destructure::Op, error::Error},
    Span,
};

use super::{
    destructure::DestructureLayer, error::Result, BinaryOpType, CompareType, Coord, NextCoord, Type,
};

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
    CastInt {
        span: Span,
        expr: Box<Expr>,
    },
    CastUInt {
        span: Span,
        expr: Box<Expr>,
    },
    CastFloat {
        span: Span,
        expr: Box<Expr>,
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

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::Static { span, .. }
            | Expr::Variable { span, .. }
            | Expr::Int { span, .. }
            | Expr::UInt { span, .. }
            | Expr::Float { span, .. }
            | Expr::String { span, .. }
            | Expr::CastInt { span, .. }
            | Expr::CastUInt { span, .. }
            | Expr::CastFloat { span, .. }
            | Expr::Add { span, .. }
            | Expr::Sub { span, .. }
            | Expr::Mul { span, .. }
            | Expr::Div { span, .. }
            | Expr::Rem { span, .. }
            | Expr::BitAnd { span, .. }
            | Expr::BitOr { span, .. }
            | Expr::BitXor { span, .. }
            | Expr::ShiftLeft { span, .. }
            | Expr::ShiftRight { span, .. }
            | Expr::BitNot { span, .. }
            | Expr::Call { span, .. } => span.clone(),
        }
    }

    pub fn is_const(&self) -> bool {
        matches!(
            self,
            Expr::Int { .. } | Expr::UInt { .. } | Expr::Float { .. } | Expr::String { .. }
        )
    }

    pub fn const_type_match(&self, other: &Self) -> bool {
        if !self.is_const() || !other.is_const() {
            return false;
        }
        match self {
            Expr::Int { .. } => matches!(other, Expr::Int { .. }),
            Expr::UInt { .. } => matches!(other, Expr::UInt { .. }),
            Expr::Float { .. } => matches!(other, Expr::Float { .. }),
            Expr::String { .. } => matches!(other, Expr::String { .. }),
            _ => unreachable!(),
        }
    }

    pub fn const_eval(self) -> Result<Self> {
        match self {
            Expr::CastInt { span, expr } => {
                let expr = expr.const_eval()?;
                if !expr.is_const() {
                    return Ok(Expr::CastInt {
                        span,
                        expr: Box::new(expr),
                    });
                }
                if let Expr::UInt { span, value } = expr {
                    return Ok(Expr::Int {
                        span,
                        value: value as i64,
                    });
                }
                if let Expr::Float { span, value } = expr {
                    return Ok(Expr::Int {
                        span,
                        value: value as i64,
                    });
                }
                Err(Error::InvalidCast { span })
            }
            Expr::CastUInt { span, expr } => {
                let expr = expr.const_eval()?;
                if !expr.is_const() {
                    return Ok(Expr::CastUInt {
                        span,
                        expr: Box::new(expr),
                    });
                }
                if let Expr::Int { span, value } = expr {
                    return Ok(Expr::UInt {
                        span,
                        value: value as u64,
                    });
                }
                if let Expr::Float { span, value } = expr {
                    return Ok(Expr::UInt {
                        span,
                        value: value as u64,
                    });
                }
                Err(Error::InvalidCast { span })
            }
            Expr::CastFloat { span, expr } => {
                let expr = expr.const_eval()?;
                if !expr.is_const() {
                    return Ok(Expr::CastFloat {
                        span,
                        expr: Box::new(expr),
                    });
                }
                if let Expr::Int { span, value } = expr {
                    return Ok(Expr::Float {
                        span,
                        value: value as f64,
                    });
                }
                if let Expr::UInt { span, value } = expr {
                    return Ok(Expr::Float {
                        span,
                        value: value as f64,
                    });
                }
                Err(Error::InvalidCast { span })
            }
            Expr::Add { span, left, right } => {
                const_eval_binop(BinaryOpType::Add, span, *left, *right)
            }
            Expr::Sub { span, left, right } => {
                const_eval_binop(BinaryOpType::Sub, span, *left, *right)
            }
            Expr::Mul { span, left, right } => {
                const_eval_binop(BinaryOpType::Mul, span, *left, *right)
            }
            Expr::Div { span, left, right } => {
                const_eval_binop(BinaryOpType::Div, span, *left, *right)
            }
            Expr::Rem { span, left, right } => {
                const_eval_binop(BinaryOpType::Rem, span, *left, *right)
            }
            Expr::BitAnd { span, left, right } => {
                const_eval_binop(BinaryOpType::BitAnd, span, *left, *right)
            }
            Expr::BitOr { span, left, right } => {
                const_eval_binop(BinaryOpType::BitOr, span, *left, *right)
            }
            Expr::BitXor { span, left, right } => {
                const_eval_binop(BinaryOpType::BitXor, span, *left, *right)
            }
            Expr::ShiftLeft { span, left, right } => {
                const_eval_binop(BinaryOpType::ShiftLeft, span, *left, *right)
            }
            Expr::ShiftRight { span, left, right } => {
                const_eval_binop(BinaryOpType::ShiftRight, span, *left, *right)
            }
            Expr::BitNot { span, expr } => {
                let expr = expr.const_eval()?;
                if !expr.is_const() {
                    return Ok(Expr::BitNot {
                        span,
                        expr: Box::new(expr),
                    });
                }
                if let Expr::Int { value: expr, .. } = expr {
                    return Ok(Expr::Int { span, value: !expr });
                }
                if let Expr::UInt { value: expr, .. } = expr {
                    return Ok(Expr::UInt { span, value: !expr });
                }
                Err(Error::InvalidBitNot { span })
            }
            Expr::Call {
                span,
                coord,
                params,
            } => {
                let mut new_params = Vec::with_capacity(params.len());
                for param in params {
                    new_params.push(param.const_eval()?);
                }
                Ok(Expr::Call {
                    span,
                    coord,
                    params: new_params,
                })
            }
            other => Ok(other),
        }
    }
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

fn const_eval_binop(op: BinaryOpType, span: Span, left: Expr, right: Expr) -> Result<Expr> {
    match op {
        BinaryOpType::Add
        | BinaryOpType::Sub
        | BinaryOpType::Mul
        | BinaryOpType::Div
        | BinaryOpType::Rem
        | BinaryOpType::BitAnd
        | BinaryOpType::BitOr
        | BinaryOpType::BitXor => {
            let left = left.const_eval()?;
            let right = right.const_eval()?;
            if !left.is_const() || !right.is_const() {
                return Ok(op.constructor()(span, Box::new(left), Box::new(right)));
            }
            if !left.const_type_match(&right) {
                return Ok(op.constructor()(span, Box::new(left), Box::new(right)));
                // return Err(Error::NonMatchingTypes {
                //     left: left.span(),
                //     right: right.span(),
                // });
            }
            match op {
                BinaryOpType::Add => {
                    if let Expr::Int { value: left, .. } = left {
                        let Expr::Int { value: right, .. } = right else {unreachable!()};
                        return Ok(Expr::Int {
                            span,
                            value: left + right,
                        });
                    }
                    if let Expr::UInt { value: left, .. } = left {
                        let Expr::UInt { value: right, .. } = right else {unreachable!()};
                        return Ok(Expr::UInt {
                            span,
                            value: left + right,
                        });
                    }
                    if let Expr::Float { value: left, .. } = left {
                        let Expr::Float { value: right, .. } = right else {unreachable!()};
                        return Ok(Expr::Float {
                            span,
                            value: left + right,
                        });
                    }
                }
                BinaryOpType::Sub => {
                    if let Expr::Int { value: left, .. } = left {
                        let Expr::Int { value: right, .. } = right else {unreachable!()};
                        return Ok(Expr::Int {
                            span,
                            value: left - right,
                        });
                    }
                    if let Expr::UInt { value: left, .. } = left {
                        let Expr::UInt { value: right, .. } = right else {unreachable!()};
                        return Ok(Expr::UInt {
                            span,
                            value: left - right,
                        });
                    }
                    if let Expr::Float { value: left, .. } = left {
                        let Expr::Float { value: right, .. } = right else {unreachable!()};
                        return Ok(Expr::Float {
                            span,
                            value: left - right,
                        });
                    }
                }
                BinaryOpType::Mul => {
                    if let Expr::Int { value: left, .. } = left {
                        let Expr::Int { value: right, .. } = right else {unreachable!()};
                        return Ok(Expr::Int {
                            span,
                            value: left * right,
                        });
                    }
                    if let Expr::UInt { value: left, .. } = left {
                        let Expr::UInt { value: right, .. } = right else {unreachable!()};
                        return Ok(Expr::UInt {
                            span,
                            value: left * right,
                        });
                    }
                    if let Expr::Float { value: left, .. } = left {
                        let Expr::Float { value: right, .. } = right else {unreachable!()};
                        return Ok(Expr::Float {
                            span,
                            value: left * right,
                        });
                    }
                }
                BinaryOpType::Div => {
                    if let Expr::Int { value: left, .. } = left {
                        let Expr::Int { value: right, .. } = right else {unreachable!()};
                        return Ok(Expr::Int {
                            span,
                            value: left / right,
                        });
                    }
                    if let Expr::UInt { value: left, .. } = left {
                        let Expr::UInt { value: right, .. } = right else {unreachable!()};
                        return Ok(Expr::UInt {
                            span,
                            value: left / right,
                        });
                    }
                    if let Expr::Float { value: left, .. } = left {
                        let Expr::Float { value: right, .. } = right else {unreachable!()};
                        return Ok(Expr::Float {
                            span,
                            value: left / right,
                        });
                    }
                }
                BinaryOpType::Rem => {
                    if let Expr::Int { value: left, .. } = left {
                        let Expr::Int { value: right, .. } = right else {unreachable!()};
                        return Ok(Expr::Int {
                            span,
                            value: left % right,
                        });
                    }
                    if let Expr::UInt { value: left, .. } = left {
                        let Expr::UInt { value: right, .. } = right else {unreachable!()};
                        return Ok(Expr::UInt {
                            span,
                            value: left % right,
                        });
                    }
                    if let Expr::Float { value: left, .. } = left {
                        let Expr::Float { value: right, .. } = right else {unreachable!()};
                        return Ok(Expr::Float {
                            span,
                            value: left % right,
                        });
                    }
                }
                BinaryOpType::BitAnd => {
                    if let Expr::Int { value: left, .. } = left {
                        let Expr::Int { value: right, .. } = right else {unreachable!()};
                        return Ok(Expr::Int {
                            span,
                            value: left & right,
                        });
                    }
                    if let Expr::UInt { value: left, .. } = left {
                        let Expr::UInt { value: right, .. } = right else {unreachable!()};
                        return Ok(Expr::UInt {
                            span,
                            value: left & right,
                        });
                    }
                }
                BinaryOpType::BitOr => {
                    if let Expr::Int { value: left, .. } = left {
                        let Expr::Int { value: right, .. } = right else {unreachable!()};
                        return Ok(Expr::Int {
                            span,
                            value: left | right,
                        });
                    }
                    if let Expr::UInt { value: left, .. } = left {
                        let Expr::UInt { value: right, .. } = right else {unreachable!()};
                        return Ok(Expr::UInt {
                            span,
                            value: left | right,
                        });
                    }
                }
                BinaryOpType::BitXor => {
                    if let Expr::Int { value: left, .. } = left {
                        let Expr::Int { value: right, .. } = right else {unreachable!()};
                        return Ok(Expr::Int {
                            span,
                            value: left ^ right,
                        });
                    }
                    if let Expr::UInt { value: left, .. } = left {
                        let Expr::UInt { value: right, .. } = right else {unreachable!()};
                        return Ok(Expr::UInt {
                            span,
                            value: left ^ right,
                        });
                    }
                }
                _ => unreachable!(),
            }
            Err(Error::InvalidBinOp {
                left: left.span(),
                right: right.span(),
            })
        }
        BinaryOpType::ShiftLeft => {
            let left = left.const_eval()?;
            let right = right.const_eval()?;
            if !left.is_const() || !right.is_const() {
                return Ok(op.constructor()(span, Box::new(left), Box::new(right)));
            }
            if let Expr::Int { value: left, .. } = left {
                if let Expr::Int { value: right, .. } = right {
                    return Ok(Expr::Int {
                        span,
                        value: left << right,
                    });
                }
                if let Expr::UInt { value: right, .. } = right {
                    return Ok(Expr::Int {
                        span,
                        value: left << right,
                    });
                }
            }
            if let Expr::UInt { value: left, .. } = left {
                if let Expr::Int { value: right, .. } = right {
                    return Ok(Expr::UInt {
                        span,
                        value: left << right,
                    });
                }
                if let Expr::UInt { value: right, .. } = right {
                    return Ok(Expr::UInt {
                        span,
                        value: left << right,
                    });
                }
            }
            Err(Error::InvalidBinOp {
                left: left.span(),
                right: right.span(),
            })
        }
        BinaryOpType::ShiftRight => {
            let left = left.const_eval()?;
            let right = right.const_eval()?;
            if !left.is_const() || !right.is_const() {
                return Ok(op.constructor()(span, Box::new(left), Box::new(right)));
            }
            if let Expr::Int { value: left, .. } = left {
                if let Expr::Int { value: right, .. } = right {
                    return Ok(Expr::Int {
                        span,
                        value: left >> right,
                    });
                }
                if let Expr::UInt { value: right, .. } = right {
                    return Ok(Expr::Int {
                        span,
                        value: left >> right,
                    });
                }
            }
            if let Expr::UInt { value: left, .. } = left {
                if let Expr::Int { value: right, .. } = right {
                    return Ok(Expr::UInt {
                        span,
                        value: left >> right,
                    });
                }
                if let Expr::UInt { value: right, .. } = right {
                    return Ok(Expr::UInt {
                        span,
                        value: left >> right,
                    });
                }
            }
            Err(Error::InvalidBinOp {
                left: left.span(),
                right: right.span(),
            })
        }
    }
}
