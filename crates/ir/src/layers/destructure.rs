use super::{
    upper::{Expr, Variable},
    ModuleCoord,
};

pub struct DestructureLayer {
    pub variables: Vec<Variable>,
    pub ops: Vec<Op>,
}

pub enum Op {
    PutCoord {
        coord: usize,
    },
    BranchCoord {
        coord: usize,
    },
    BranchCoordIfNonZero {
        coord: usize,
    },
    BranchCoordIfZero {
        coord: usize,
    },
    BranchCoordEqual {
        coord: usize,
    },
    BranchCoordNonEqual {
        coord: usize,
    },
    BranchCoordLess {
        coord: usize,
    },
    BranchCoordGreater {
        coord: usize,
    },
    BranchCoordLessEqual {
        coord: usize,
    },
    BranchCoordGreaterEqual {
        coord: usize,
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
        coord: ModuleCoord,
        params: Vec<Expr>,
    },
}
