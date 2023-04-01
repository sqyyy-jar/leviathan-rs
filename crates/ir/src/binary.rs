use std::{
    collections::HashMap,
    io::{Result, Seek, Write},
};

use byteorder::{LittleEndian, WriteBytesExt};
use urban_common::{
    binary::EXECUTABLE,
    opcodes::{
        L0_DIV, L0_DIVS, L0_MOVS, L0_MUL, L0_REM, L0_REMS, L0_SUB, L1_INT, L1_LDR, L1_LDRB,
        L1_LDRH, L1_LDRW, L1_NCALL, L1_SHL, L1_SHR, L1_SHRS, L1_STR, L1_STRB, L1_STRH, L1_STRW,
        L1_VCALL, L2_ADD, L2_ADDF, L2_AND, L2_CMP, L2_CMPF, L2_CMPS, L2_DIV, L2_DIVF, L2_DIVS,
        L2_MUL, L2_MULF, L2_OR, L2_REM, L2_REMF, L2_REMS, L2_SHL, L2_SHR, L2_SHRS, L2_SUB, L2_SUBF,
        L2_XOR, L3_FTI, L3_ITF, L3_MOV, L3_NOT, L4_LDBO, L4_LDPC, L4_NCALL, L4_VCALL, L5_HALT,
        L5_RET,
    },
};

use crate::{
    layers::lower::LowOp,
    util::{alignment, MaxBitsU32},
};

pub struct Binary {
    pub modules: HashMap<usize, BinaryModule>,
}

impl Binary {
    pub fn assemble(&self, out: &mut (impl Write + Seek)) -> Result<()> {
        const _FLAGS_OFFSET: u64 = 4;
        const _ENTRYPOINT_OFFSET: u64 = 8;
        const _HEADER_LENGTH: u64 = 16;
        out.write_all(b"\0urb")?;
        out.write_u32::<LittleEndian>(EXECUTABLE)?;
        out.write_u64::<LittleEndian>(0)?;
        let mut ptr = 0usize;
        for (module_index, module) in &self.modules {
            let mut statics = HashMap::with_capacity(module.statics.len());
            let mut funcs = HashMap::with_capacity(module.funcs.len());
            for (static_index, static_) in &module.statics {
                statics.insert(*static_index, ptr);
                static_.assemble(&mut ptr, out)?;
            }
            for (func_index, func) in &module.funcs {
                let mut locals = HashMap::with_capacity(func.locals.len());
                for (local_index, local) in func.locals.iter().enumerate() {
                    locals.insert(local_index, ptr);
                    local.assemble(&mut ptr, out)?;
                }
                funcs.insert(*func_index, ptr);
                for op in &func.ops {
                    match op {
                        LowOp::PutCoord { coord } => todo!(),
                        LowOp::BranchCoord { coord } => todo!(),
                        LowOp::BranchCoordIfNonZero { reg, coord } => todo!(),
                        LowOp::BranchCoordIfZero { reg, coord } => todo!(),
                        LowOp::BranchCoordEqual { reg, coord } => todo!(),
                        LowOp::BranchCoordNonEqual { reg, coord } => todo!(),
                        LowOp::BranchCoordLess { reg, coord } => todo!(),
                        LowOp::BranchCoordGreater { reg, coord } => todo!(),
                        LowOp::BranchCoordLessEqual { reg, coord } => todo!(),
                        LowOp::BranchCoordGreaterEqual { reg, coord } => todo!(),
                        LowOp::LoadStatic64 { dst, coord } => todo!(),
                        LowOp::LoadLocalStatic64 { dst, coord } => todo!(),
                        LowOp::LoadStaticAddress { dst, coord } => todo!(),
                        LowOp::LoadLocalStaticAddress { dst, coord } => todo!(),
                        LowOp::AddImmediate {
                            dst,
                            lhs: src,
                            rhs: immediate,
                        } => emit(
                            &mut ptr,
                            out,
                            L0_SUB | dst.value() | src.value() << 5 | immediate.cut(17) << 10,
                        )?,
                        LowOp::SubImmediate {
                            dst,
                            lhs: src,
                            rhs: immediate,
                        } => emit(
                            &mut ptr,
                            out,
                            L0_SUB | dst.value() | src.value() << 5 | immediate.cut(17) << 10,
                        )?,
                        LowOp::MulImmediate {
                            dst,
                            lhs: src,
                            rhs: immediate,
                        } => emit(
                            &mut ptr,
                            out,
                            L0_MUL | dst.value() | src.value() << 5 | immediate.cut(17) << 10,
                        )?,
                        LowOp::DivImmediate {
                            dst,
                            lhs: src,
                            rhs: immediate,
                        } => emit(
                            &mut ptr,
                            out,
                            L0_DIV | dst.value() | src.value() << 5 | immediate.cut(17) << 10,
                        )?,
                        LowOp::RemImmediate {
                            dst,
                            lhs: src,
                            rhs: immediate,
                        } => emit(
                            &mut ptr,
                            out,
                            L0_REM | dst.value() | src.value() << 5 | immediate.cut(17) << 10,
                        )?,
                        LowOp::DivSignedImmediate {
                            dst,
                            lhs: src,
                            rhs: immediate,
                        } => emit(
                            &mut ptr,
                            out,
                            L0_DIVS | dst.value() | src.value() << 5 | immediate.cut(17) << 10,
                        )?,
                        LowOp::RemSignedImmediate {
                            dst,
                            lhs: src,
                            rhs: immediate,
                        } => emit(
                            &mut ptr,
                            out,
                            L0_REMS | dst.value() | src.value() << 5 | immediate.cut(17) << 10,
                        )?,
                        LowOp::MoveImmediate { dst, immediate } => emit(
                            &mut ptr,
                            out,
                            L0_MOVS | dst.value() | immediate.cut(22) << 5,
                        )?,
                        LowOp::MoveSignedImmediate { dst, immediate } => emit(
                            &mut ptr,
                            out,
                            L0_MOVS | dst.value() | immediate.cut(22) << 5,
                        )?,
                        LowOp::ShiftLeftImmediate { dst, lhs, rhs } => emit(
                            &mut ptr,
                            out,
                            L1_SHL | dst.value() | lhs.value() << 5 | rhs.cut(11) << 10,
                        )?,
                        LowOp::ShiftRightImmediate { dst, lhs, rhs } => emit(
                            &mut ptr,
                            out,
                            L1_SHR | dst.value() | lhs.value() << 5 | rhs.cut(11) << 10,
                        )?,
                        LowOp::ShiftRightSignedImmediate { dst, lhs, rhs } => emit(
                            &mut ptr,
                            out,
                            L1_SHRS | dst.value() | lhs.value() << 5 | rhs.cut(11) << 10,
                        )?,
                        LowOp::Load8 { dst, src, offset } => emit(
                            &mut ptr,
                            out,
                            L1_LDRB | dst.value() | src.value() << 5 | offset.cut(11) << 10,
                        )?,
                        LowOp::Load16 { dst, src, offset } => emit(
                            &mut ptr,
                            out,
                            L1_LDRH | dst.value() | src.value() << 5 | offset.cut(11) << 10,
                        )?,
                        LowOp::Load32 { dst, src, offset } => emit(
                            &mut ptr,
                            out,
                            L1_LDRW | dst.value() | src.value() << 5 | offset.cut(11) << 10,
                        )?,
                        LowOp::Load64 { dst, src, offset } => emit(
                            &mut ptr,
                            out,
                            L1_LDR | dst.value() | src.value() << 5 | offset.cut(11) << 10,
                        )?,
                        LowOp::Store8 { dst, src, offset } => emit(
                            &mut ptr,
                            out,
                            L1_STRB | dst.value() | src.value() << 5 | offset.cut(11) << 10,
                        )?,
                        LowOp::Store16 { dst, src, offset } => emit(
                            &mut ptr,
                            out,
                            L1_STRH | dst.value() | src.value() << 5 | offset.cut(11) << 10,
                        )?,
                        LowOp::Store32 { dst, src, offset } => emit(
                            &mut ptr,
                            out,
                            L1_STRW | dst.value() | src.value() << 5 | offset.cut(11) << 10,
                        )?,
                        LowOp::Store64 { dst, src, offset } => emit(
                            &mut ptr,
                            out,
                            L1_STR | dst.value() | src.value() << 5 | offset.cut(11) << 10,
                        )?,
                        LowOp::InterruptImmediate { id } => {
                            emit(&mut ptr, out, L1_INT | *id as u32)?
                        }
                        LowOp::NativeCallImmediate { id } => {
                            emit(&mut ptr, out, L1_NCALL | id.cut(21))?
                        }
                        LowOp::VirtualCallImmediate { id } => {
                            emit(&mut ptr, out, L1_VCALL | id.cut(21))?
                        }
                        LowOp::Add { dst, lhs, rhs } => emit(
                            &mut ptr,
                            out,
                            L2_ADD | dst.value() | lhs.value() << 5 | rhs.value() << 10,
                        )?,
                        LowOp::Sub { dst, lhs, rhs } => emit(
                            &mut ptr,
                            out,
                            L2_SUB | dst.value() | lhs.value() << 5 | rhs.value() << 10,
                        )?,
                        LowOp::Mul { dst, lhs, rhs } => emit(
                            &mut ptr,
                            out,
                            L2_MUL | dst.value() | lhs.value() << 5 | rhs.value() << 10,
                        )?,
                        LowOp::Div { dst, lhs, rhs } => emit(
                            &mut ptr,
                            out,
                            L2_DIV | dst.value() | lhs.value() << 5 | rhs.value() << 10,
                        )?,
                        LowOp::Rem { dst, lhs, rhs } => emit(
                            &mut ptr,
                            out,
                            L2_REM | dst.value() | lhs.value() << 5 | rhs.value() << 10,
                        )?,
                        LowOp::DivSigned { dst, lhs, rhs } => emit(
                            &mut ptr,
                            out,
                            L2_DIVS | dst.value() | lhs.value() << 5 | rhs.value() << 10,
                        )?,
                        LowOp::RemSigned { dst, lhs, rhs } => emit(
                            &mut ptr,
                            out,
                            L2_REMS | dst.value() | lhs.value() << 5 | rhs.value() << 10,
                        )?,
                        LowOp::AddFloat { dst, lhs, rhs } => emit(
                            &mut ptr,
                            out,
                            L2_ADDF | dst.value() | lhs.value() << 5 | rhs.value() << 10,
                        )?,
                        LowOp::SubFloat { dst, lhs, rhs } => emit(
                            &mut ptr,
                            out,
                            L2_SUBF | dst.value() | lhs.value() << 5 | rhs.value() << 10,
                        )?,
                        LowOp::MulFloat { dst, lhs, rhs } => emit(
                            &mut ptr,
                            out,
                            L2_MULF | dst.value() | lhs.value() << 5 | rhs.value() << 10,
                        )?,
                        LowOp::DivFloat { dst, lhs, rhs } => emit(
                            &mut ptr,
                            out,
                            L2_DIVF | dst.value() | lhs.value() << 5 | rhs.value() << 10,
                        )?,
                        LowOp::RemFloat { dst, lhs, rhs } => emit(
                            &mut ptr,
                            out,
                            L2_REMF | dst.value() | lhs.value() << 5 | rhs.value() << 10,
                        )?,
                        LowOp::And { dst, lhs, rhs } => emit(
                            &mut ptr,
                            out,
                            L2_AND | dst.value() | lhs.value() << 5 | rhs.value() << 10,
                        )?,
                        LowOp::Or { dst, lhs, rhs } => emit(
                            &mut ptr,
                            out,
                            L2_OR | dst.value() | lhs.value() << 5 | rhs.value() << 10,
                        )?,
                        LowOp::Xor { dst, lhs, rhs } => emit(
                            &mut ptr,
                            out,
                            L2_XOR | dst.value() | lhs.value() << 5 | rhs.value() << 10,
                        )?,
                        LowOp::ShiftLeft { dst, lhs, rhs } => emit(
                            &mut ptr,
                            out,
                            L2_SHL | dst.value() | lhs.value() << 5 | rhs.value() << 10,
                        )?,
                        LowOp::ShiftRight { dst, lhs, rhs } => emit(
                            &mut ptr,
                            out,
                            L2_SHR | dst.value() | lhs.value() << 5 | rhs.value() << 10,
                        )?,
                        LowOp::ShiftRightSigned { dst, lhs, rhs } => emit(
                            &mut ptr,
                            out,
                            L2_SHRS | dst.value() | lhs.value() << 5 | rhs.value() << 10,
                        )?,
                        LowOp::Compare { dst, lhs, rhs } => emit(
                            &mut ptr,
                            out,
                            L2_CMP | dst.value() | lhs.value() << 5 | rhs.value() << 10,
                        )?,
                        LowOp::CompareSigned { dst, lhs, rhs } => emit(
                            &mut ptr,
                            out,
                            L2_CMPS | dst.value() | lhs.value() << 5 | rhs.value() << 10,
                        )?,
                        LowOp::CompareFloat { dst, lhs, rhs } => emit(
                            &mut ptr,
                            out,
                            L2_CMPF | dst.value() | lhs.value() << 5 | rhs.value() << 10,
                        )?,
                        LowOp::Not { dst, src } => {
                            emit(&mut ptr, out, L3_NOT | dst.value() | src.value() << 5)?
                        }
                        LowOp::Move { dst, src } => {
                            emit(&mut ptr, out, L3_MOV | dst.value() | src.value() << 5)?
                        }
                        LowOp::FloatToInt { dst, src } => {
                            emit(&mut ptr, out, L3_FTI | dst.value() | src.value() << 5)?
                        }
                        LowOp::IntToFloat { dst, src } => {
                            emit(&mut ptr, out, L3_ITF | dst.value() | src.value() << 5)?
                        }
                        LowOp::NativeCall { id } => emit(&mut ptr, out, L4_NCALL | id.value())?,
                        LowOp::VirtualCall { id } => emit(&mut ptr, out, L4_VCALL | id.value())?,
                        LowOp::LoadBaseOffset { dst } => {
                            emit(&mut ptr, out, L4_LDBO | dst.value())?
                        }
                        LowOp::LoadProgramCounter { dst } => {
                            emit(&mut ptr, out, L4_LDPC | dst.value())?
                        }
                        LowOp::Halt => emit(&mut ptr, out, L5_HALT)?,
                        LowOp::Return => emit(&mut ptr, out, L5_RET)?,
                    }
                }
            }
        }
        Ok(())
    }
}

pub struct BinaryModule {
    pub statics: HashMap<usize, BinaryStatic>,
    pub funcs: HashMap<usize, BinaryFunc>,
}

pub enum BinaryStatic {
    Int(i64),
    UInt(u64),
    Float(f64),
    String(String),
    FilledBuffer { size: usize, fill: u8 },
}

impl BinaryStatic {
    pub fn assemble(&self, ptr: &mut usize, out: &mut (impl Write + Seek)) -> Result<usize> {
        let mut addr = *ptr;
        match self {
            BinaryStatic::Int(value) => {
                out.write_i64::<LittleEndian>(*value)?;
                *ptr += 8;
            }
            BinaryStatic::UInt(value) => {
                out.write_u64::<LittleEndian>(*value)?;
                *ptr += 8;
            }
            BinaryStatic::Float(value) => {
                out.write_f64::<LittleEndian>(*value)?;
                *ptr += 8;
            }
            BinaryStatic::String(value) => {
                out.write_u64::<LittleEndian>(value.len() as u64)?;
                *ptr += 8;
                addr = *ptr;
                out.write_all(value.as_bytes())?;
                *ptr += value.len();
                for _ in 0..alignment(value.len(), 4) {
                    out.write_u8(0)?;
                    *ptr += 1;
                }
            }
            BinaryStatic::FilledBuffer { size, fill } => {
                *ptr += *size;
                for _ in 0..*size {
                    out.write_u8(*fill)?;
                }
                for _ in 0..alignment(*size, 4) {
                    out.write_u8(0)?;
                    *ptr += 1;
                }
            }
        }
        Ok(addr)
    }
}

pub struct BinaryFunc {
    pub locals: Vec<BinaryStatic>,
    pub ops: Vec<LowOp>,
}

pub fn emit(ptr: &mut usize, out: &mut (impl Write + Seek), insn: u32) -> Result<()> {
    *ptr += 4;
    out.write_u32::<LittleEndian>(insn)
}
