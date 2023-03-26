pub mod lower;
pub mod middle;
pub mod upper;

pub struct ModuleCoord {
    pub module: usize,
    pub element: usize,
}

pub enum Type {
    Int,
    UInt,
    Float,
    String,
}
