pub struct LowerLayer {
    pub ops: Vec<LowOp>,
}

pub struct Reg {
    value: u8,
}

impl Reg {
    pub const fn new(value: u8) -> Self {
        assert!(matches!(value, 0..=31));
        Self { value }
    }

    #[inline(always)]
    pub const fn value(&self) -> u8 {
        self.value
    }
}

pub enum LowOp {
    AddImmediate { dst: Reg, src: Reg, immediate: u32 },
    SubImmediate { dst: Reg, src: Reg, immediate: u32 },
    MulImmediate { dst: Reg, src: Reg, immediate: u32 },
    DivImmediate { dst: Reg, src: Reg, immediate: u32 },
    RemImmediate { dst: Reg, src: Reg, immediate: u32 },
    DivSignedImmediate { dst: Reg, src: Reg, immediate: i32 },
    RemSignedImmediate { dst: Reg, src: Reg, immediate: i32 },
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
