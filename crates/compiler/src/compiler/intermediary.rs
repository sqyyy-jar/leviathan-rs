#[derive(Debug)]
pub enum IntermediaryStaticValue {
    Int(i64),
    UInt(u64),
    Float(f64),
    String(String),
    Buffer { size: usize },
}

#[derive(Debug)]
pub enum Insn {
    Raw(u32),
    LdStaticAbsAddr {
        dst: Reg,
        index: usize,
    },
    LoadStatic {
        dst: Reg,
        index: usize,
    },
    BranchLabelLinked {
        module_index: usize,
        func_index: usize,
    },
    CreatePoint {
        pos: usize,
    },
    BranchPoint {
        pos: usize,
    },
    BranchPointIfEq {
        pos: usize,
        reg: Reg,
    },
    BranchPointIfNeq {
        pos: usize,
        reg: Reg,
    },
    BranchPointIfLt {
        pos: usize,
        reg: Reg,
    },
    BranchPointIfGt {
        pos: usize,
        reg: Reg,
    },
    BranchPointIfLeq {
        pos: usize,
        reg: Reg,
    },
    BranchPointIfGeq {
        pos: usize,
        reg: Reg,
    },
    BranchPointIfNz {
        pos: usize,
        reg: Reg,
    },
    BranchPointIfZr {
        pos: usize,
        reg: Reg,
    },
    Ret,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
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

impl From<usize> for Reg {
    fn from(value: usize) -> Self {
        match value {
            0 => Reg::R0,
            1 => Reg::R1,
            2 => Reg::R2,
            3 => Reg::R3,
            4 => Reg::R4,
            5 => Reg::R5,
            6 => Reg::R6,
            7 => Reg::R7,
            8 => Reg::R8,
            9 => Reg::R9,
            10 => Reg::R10,
            11 => Reg::R11,
            12 => Reg::R12,
            13 => Reg::R13,
            14 => Reg::R14,
            15 => Reg::R15,
            16 => Reg::R16,
            17 => Reg::R17,
            18 => Reg::R18,
            19 => Reg::R19,
            20 => Reg::R20,
            21 => Reg::R21,
            22 => Reg::R22,
            23 => Reg::R23,
            24 => Reg::R24,
            25 => Reg::R25,
            26 => Reg::R26,
            27 => Reg::R27,
            28 => Reg::R28,
            29 => Reg::R29,
            30 => Reg::R30,
            31 => Reg::R31,
            _ => panic!("Invalid register"),
        }
    }
}
