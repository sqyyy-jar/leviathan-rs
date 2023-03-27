pub mod destructure;
pub mod lower;
pub mod upper;

pub struct ModuleCoord {
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
