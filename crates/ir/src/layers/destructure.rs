use super::{
    error::Result,
    upper::{Expr, Var},
    CompareType, Coord,
};

#[derive(Debug)]
pub struct DestructureLayer {
    coord_index: usize,
    pub vars: Vec<Var>,
    pub ops: Vec<Op>,
}

impl DestructureLayer {
    pub fn alloc_coord(&mut self) -> usize {
        let coord = self.coord_index;
        self.coord_index += 1;
        coord
    }

    pub fn put_coord(&mut self, coord: usize) {
        self.ops.push(Op::PutCoord { coord });
    }

    pub fn branch(&mut self, coord: usize) {
        self.ops.push(Op::BranchCoord { coord });
    }

    pub fn branch_if(&mut self, coord: usize, op: CompareType, left: Expr, right: Expr) {
        self.ops.push(Op::BranchCoordIf {
            coord,
            op,
            left,
            right,
        });
    }

    pub fn const_eval(self) -> Result<Self> {
        let mut layer = DestructureLayer {
            coord_index: self.coord_index,
            vars: self.vars,
            ops: Vec::with_capacity(self.ops.len()),
        };
        for op in self.ops {
            layer.ops.push(op.const_eval()?);
        }
        Ok(layer)
    }
}

impl Default for DestructureLayer {
    fn default() -> Self {
        Self {
            coord_index: 0,
            vars: Vec::with_capacity(0),
            ops: Vec::with_capacity(0),
        }
    }
}

#[derive(Debug)]
pub enum Op {
    PutCoord {
        coord: usize,
    },
    BranchCoord {
        coord: usize,
    },
    BranchCoordIf {
        coord: usize,
        op: CompareType,
        left: Expr,
        right: Expr,
    },
    Let {
        index: usize,
        expr: Expr,
    },
    Return {
        expr: Option<Expr>,
    },
    Assign {
        index: usize,
        expr: Expr,
    },
    Call {
        coord: Coord,
        params: Vec<Expr>,
    },
}

impl Op {
    pub fn const_eval(self) -> Result<Self> {
        match self {
            Op::BranchCoordIf {
                coord,
                op,
                left,
                right,
            } => Ok(Op::BranchCoordIf {
                coord,
                op,
                left: left.const_eval()?,
                right: right.const_eval()?,
            }),
            Op::Let { index, expr } => Ok(Op::Let {
                index,
                expr: expr.const_eval()?,
            }),
            Op::Return { expr } => Ok(Op::Return {
                expr: if let Some(expr) = expr {
                    Some(expr.const_eval()?)
                } else {
                    None
                },
            }),
            Op::Assign { index, expr } => Ok(Op::Assign {
                index,
                expr: expr.const_eval()?,
            }),
            Op::Call { coord, params } => Ok(Op::Call {
                coord,
                params: {
                    let mut res = Vec::with_capacity(params.len());
                    for param in params {
                        res.push(param.const_eval()?);
                    }
                    res
                },
            }),
            other => Ok(other),
        }
    }
}
