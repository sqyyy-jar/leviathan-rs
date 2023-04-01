use crate::binary::BinaryStatic;

use super::Coord;

pub struct LowerLayer {
    pub locals: Vec<BinaryStatic>,
    pub ops: Vec<LowOp>,
}

#[derive(Clone, Copy)]
pub struct Reg {
    value: u8,
}

impl Reg {
    pub const fn new(value: u8) -> Self {
        assert!(matches!(value, 0..=31));
        Self { value }
    }

    #[inline(always)]
    pub const fn value(&self) -> u32 {
        self.value as u32
    }
}

pub enum LowOp {
    PutCoord { coord: usize },
    BranchCoord { coord: usize },
    BranchCoordIfNonZero { reg: Reg, coord: usize },
    BranchCoordIfZero { reg: Reg, coord: usize },
    BranchCoordEqual { reg: Reg, coord: usize },
    BranchCoordNonEqual { reg: Reg, coord: usize },
    BranchCoordLess { reg: Reg, coord: usize },
    BranchCoordGreater { reg: Reg, coord: usize },
    BranchCoordLessEqual { reg: Reg, coord: usize },
    BranchCoordGreaterEqual { reg: Reg, coord: usize },
    Call { coord: Coord },
    LoadStatic64 { dst: Reg, coord: Coord },
    LoadLocalStatic64 { dst: Reg, coord: usize },
    LoadStaticAddress { dst: Reg, coord: Coord },
    LoadLocalStaticAddress { dst: Reg, coord: usize },
    AddImmediate { dst: Reg, lhs: Reg, rhs: u32 },
    SubImmediate { dst: Reg, lhs: Reg, rhs: u32 },
    MulImmediate { dst: Reg, lhs: Reg, rhs: u32 },
    DivImmediate { dst: Reg, lhs: Reg, rhs: u32 },
    RemImmediate { dst: Reg, lhs: Reg, rhs: u32 },
    DivSignedImmediate { dst: Reg, lhs: Reg, rhs: i32 },
    RemSignedImmediate { dst: Reg, lhs: Reg, rhs: i32 },
    MoveImmediate { dst: Reg, immediate: u32 },
    MoveSignedImmediate { dst: Reg, immediate: i32 },
    ShiftLeftImmediate { dst: Reg, lhs: Reg, rhs: u32 },
    ShiftRightImmediate { dst: Reg, lhs: Reg, rhs: u32 },
    ShiftRightSignedImmediate { dst: Reg, lhs: Reg, rhs: u32 },
    Load8 { dst: Reg, src: Reg, offset: i32 },
    Load16 { dst: Reg, src: Reg, offset: i32 },
    Load32 { dst: Reg, src: Reg, offset: i32 },
    Load64 { dst: Reg, src: Reg, offset: i32 },
    Store8 { dst: Reg, src: Reg, offset: i32 },
    Store16 { dst: Reg, src: Reg, offset: i32 },
    Store32 { dst: Reg, src: Reg, offset: i32 },
    Store64 { dst: Reg, src: Reg, offset: i32 },
    InterruptImmediate { id: u16 },
    NativeCallImmediate { id: u32 },
    VirtualCallImmediate { id: u32 },
    Add { dst: Reg, lhs: Reg, rhs: Reg },
    Sub { dst: Reg, lhs: Reg, rhs: Reg },
    Mul { dst: Reg, lhs: Reg, rhs: Reg },
    Div { dst: Reg, lhs: Reg, rhs: Reg },
    Rem { dst: Reg, lhs: Reg, rhs: Reg },
    DivSigned { dst: Reg, lhs: Reg, rhs: Reg },
    RemSigned { dst: Reg, lhs: Reg, rhs: Reg },
    AddFloat { dst: Reg, lhs: Reg, rhs: Reg },
    SubFloat { dst: Reg, lhs: Reg, rhs: Reg },
    MulFloat { dst: Reg, lhs: Reg, rhs: Reg },
    DivFloat { dst: Reg, lhs: Reg, rhs: Reg },
    RemFloat { dst: Reg, lhs: Reg, rhs: Reg },
    And { dst: Reg, lhs: Reg, rhs: Reg },
    Or { dst: Reg, lhs: Reg, rhs: Reg },
    Xor { dst: Reg, lhs: Reg, rhs: Reg },
    ShiftLeft { dst: Reg, lhs: Reg, rhs: Reg },
    ShiftRight { dst: Reg, lhs: Reg, rhs: Reg },
    ShiftRightSigned { dst: Reg, lhs: Reg, rhs: Reg },
    Compare { dst: Reg, lhs: Reg, rhs: Reg },
    CompareSigned { dst: Reg, lhs: Reg, rhs: Reg },
    CompareFloat { dst: Reg, lhs: Reg, rhs: Reg },
    Not { dst: Reg, src: Reg },
    Move { dst: Reg, src: Reg },
    FloatToInt { dst: Reg, src: Reg },
    IntToFloat { dst: Reg, src: Reg },
    NativeCall { id: Reg },
    VirtualCall { id: Reg },
    LoadBaseOffset { dst: Reg },
    LoadProgramCounter { dst: Reg },
    Halt,
    Return,
}
