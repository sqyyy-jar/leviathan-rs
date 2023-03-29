use super::{
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
