use super::{
    upper::{Cond, Expr, Var},
    Coord,
};

pub struct DestructureLayer {
    pub variables: Vec<Var>,
    pub ops: Vec<Op>,
}

pub enum Op {
    PutCoord { coord: usize },
    BranchCoord { coord: usize },
    BranchCoordCond { coord: usize, cond: Cond },
    Let { index: usize, expr: Expr },
    Return { expr: Option<Expr> },
    Assign { index: usize, expr: Expr },
    Call { coord: Coord, params: Vec<Expr> },
}
