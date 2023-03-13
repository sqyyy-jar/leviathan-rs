use super::collecting::CollectedFunction;

#[derive(Debug)]
pub struct IntermediaryModule {
    pub src: String,
    pub funcs: Vec<CollectedFunction>,
    pub ir_funcs: Vec<IntermediaryFunction>,
}

#[derive(Debug)]
pub struct IntermediaryFunction {
    pub func_index: usize,
    pub ir: Vec<Insn>,
    pub deps: Vec<IntermediaryDependencyPath>,
}

#[derive(Debug)]
pub struct IntermediaryDependencyPath {
    pub module_index: usize,
    pub func_index: usize,
}

#[derive(Debug)]
pub enum Insn {
    LdcInt { dst: Reg, value: i64 },
    LdcUInt { dst: Reg, value: u64 },
    LdcFloat { dst: Reg, value: f64 },
    Ret,
}

#[repr(u8)]
#[derive(Debug)]
pub enum Reg {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    R16,
    R17,
    R18,
    R19,
    R20,
    R21,
    R22,
    R23,
    R24,
    R25,
    R26,
    R27,
    R28,
    R29,
    R30,
    R31,
}
