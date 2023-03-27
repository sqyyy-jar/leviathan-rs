pub mod destructure;
pub mod lower;
pub mod upper;

pub struct Coord {
    pub module: usize,
    pub element: usize,
}

pub enum Type {
    Unit,
    Int,
    UInt,
    Float,
    String,
}
