use std::mem;

use leviathan_ir::layers::lower::{LowOp, LowerLayer, Reg};
use phf::{phf_map, Map};

use crate::{
    compiler::{
        error::{Error, Result},
        CompileTask, Module,
    },
    parser::Node,
    util::source::Span,
};

use Component::*;

macro_rules! parse_comp {
    (reg, $module:expr, $expr:expr) => {
        parse_reg($module, $expr)
    };
    (uint, $module:expr, $expr:expr) => {
        parse_uint($expr)
    };
    (int, $module:expr, $expr:expr) => {
        parse_int($expr)
    };
    (u16, $module:expr, $expr:expr) => {
        parse_u16($expr)
    };
    ($_:ident, $expr:expr) => {
        compile_error!("Invalid component type")
    };
}

macro_rules! insn {
    ($low_op:expr, $($comp_name:ident:$component:ident:$comp_expr:expr),*) => {
        (&[$($comp_expr),*],
        |module: &mut Module, bin_func: &mut LowerLayer, mut nodes: Vec<Node>| {
            $(
                let $comp_name = parse_comp!($component, module, nodes.remove(1));
            )*
            bin_func.ops.push($low_op);
        })
    };
}

type InsnMacro = fn(&mut Module, &mut LowerLayer, Vec<Node>);

#[allow(unused)]
pub const INSN_MACROS: Map<&'static str, &[(&[Component], InsnMacro)]> = phf_map! {
    "add" => &[
        insn!(
            LowOp::AddImmediate { dst, lhs, rhs },
            dst:reg:Reg, lhs:reg:Reg, rhs:uint:U(17)
        ),
        insn!(
            LowOp::Add { dst, lhs, rhs },
            dst:reg:Reg, lhs:reg:Reg, rhs:reg:Reg
        ),
    ],
    "sub" => &[
        insn!(
            LowOp::SubImmediate { dst, lhs, rhs },
            dst:reg:Reg, lhs:reg:Reg, rhs:uint:U(17)
        ),
        insn!(
            LowOp::Sub { dst, lhs, rhs },
            dst:reg:Reg, lhs:reg:Reg, rhs:reg:Reg
        ),
    ],
    "mul" => &[
        insn!(
            LowOp::MulImmediate { dst, lhs, rhs },
            dst:reg:Reg, lhs:reg:Reg, rhs:uint:U(17)
        ),
        insn!(
            LowOp::Mul { dst, lhs, rhs },
            dst:reg:Reg, lhs:reg:Reg, rhs:reg:Reg
        ),
    ],
    "div" => &[
        insn!(
            LowOp::DivImmediate { dst, lhs, rhs },
            dst:reg:Reg, lhs:reg:Reg, rhs:uint:U(17)
        ),
        insn!(
            LowOp::Div { dst, lhs, rhs },
            dst:reg:Reg, lhs:reg:Reg, rhs:reg:Reg
        ),
    ],
    "rem" => &[
        insn!(
            LowOp::RemImmediate { dst, lhs, rhs },
            dst:reg:Reg, lhs:reg:Reg, rhs:uint:U(17)
        ),
        insn!(
            LowOp::Rem { dst, lhs, rhs },
            dst:reg:Reg, lhs:reg:Reg, rhs:reg:Reg
        ),
    ],
    "divs" => &[
        insn!(
            LowOp::DivSignedImmediate { dst, lhs, rhs },
            dst:reg:Reg, lhs:reg:Reg, rhs:int:I(17)
        ),
        insn!(
            LowOp::DivSigned { dst, lhs, rhs },
            dst:reg:Reg, lhs:reg:Reg, rhs:reg:Reg
        ),
    ],
    "rems" => &[
        insn!(
            LowOp::RemSignedImmediate { dst, lhs, rhs },
            dst:reg:Reg, lhs:reg:Reg, rhs:int:I(17)
        ),
        insn!(
            LowOp::RemSigned { dst, lhs, rhs },
            dst:reg:Reg, lhs:reg:Reg, rhs:reg:Reg
        ),
    ],
    "ldr" => &[
        insn!(
            LowOp::Load64 { dst, src, offset },
            dst:reg:Reg, src:reg:Reg, offset:int:I(11)
        ),
    ],
    "str" => &[
        insn!(
            LowOp::Store64 { dst, src, offset },
            dst:reg:Reg, src:reg:Reg, offset:int:I(11)
        ),
    ],
    "mov" => &[
        insn!(
            LowOp::MoveImmediate { dst, immediate },
            dst:reg:Reg, immediate:uint:U(22)
        ),
        insn!(
            LowOp::Move { dst, src },
            dst:reg:Reg, src:reg:Reg
        ),
    ],
    "movs" => &[
        insn!(
            LowOp::MoveSignedImmediate { dst, immediate },
            dst:reg:Reg, immediate:int:I(22)
        ),
    ],
    "shl" => &[
        insn!(
            LowOp::ShiftLeftImmediate { dst, lhs, rhs },
            dst:reg:Reg, lhs:reg:Reg, rhs:uint:U(11)
        ),
        insn!(
            LowOp::ShiftLeft { dst, lhs, rhs },
            dst:reg:Reg, lhs:reg:Reg, rhs:reg:Reg
        ),
    ],
    "shr" => &[
        insn!(
            LowOp::ShiftRightImmediate { dst, lhs, rhs },
            dst:reg:Reg, lhs:reg:Reg, rhs:uint:U(11)
        ),
        insn!(
            LowOp::ShiftRight { dst, lhs, rhs },
            dst:reg:Reg, lhs:reg:Reg, rhs:reg:Reg
        ),
    ],
    "shrs" => &[
        insn!(
            LowOp::ShiftRightSignedImmediate { dst, lhs, rhs },
            dst:reg:Reg, lhs:reg:Reg, rhs:uint:U(11)
        ),
        insn!(
            LowOp::ShiftRightSigned { dst, lhs, rhs },
            dst:reg:Reg, lhs:reg:Reg, rhs:reg:Reg
        ),
    ],
    "ldrb" => &[
        insn!(
            LowOp::Load8 { dst, src, offset },
            dst:reg:Reg, src:reg:Reg, offset:int:I(11)
        ),
    ],
    "ldrh" => &[
        insn!(
            LowOp::Load16 { dst, src, offset },
            dst:reg:Reg, src:reg:Reg, offset:int:I(11)
        ),
    ],
    "ldrw" => &[
        insn!(
            LowOp::Load32 { dst, src, offset },
            dst:reg:Reg, src:reg:Reg, offset:int:I(11)
        ),
    ],
    "strb" => &[
        insn!(
            LowOp::Store8 { dst, src, offset },
            dst:reg:Reg, src:reg:Reg, offset:int:I(11)
        ),
    ],
    "strh" => &[
        insn!(
            LowOp::Store16 { dst, src, offset },
            dst:reg:Reg, src:reg:Reg, offset:int:I(11)
        ),
    ],
    "strw" => &[
        insn!(
            LowOp::Store32 { dst, src, offset },
            dst:reg:Reg, src:reg:Reg, offset:int:I(11)
        ),
    ],
    "int" => &[
        insn!(
            LowOp::InterruptImmediate { id },
            id:u16:U(16)
        ),
    ],
    "ncall" => &[
        insn!(
            LowOp::NativeCallImmediate { id },
            id:uint:U(21)
        ),
        insn!(
            LowOp::NativeCall { id },
            id:reg:Reg
        ),
    ],
    "vcall" => &[
        insn!(
            LowOp::VirtualCallImmediate { id },
            id:uint:U(21)
        ),
        insn!(
            LowOp::VirtualCall { id },
            id:reg:Reg
        ),
    ],
    "addf" => &[
        insn!(
            LowOp::AddFloat { dst, lhs, rhs },
            dst:reg:Reg, lhs:reg:Reg, rhs:reg:Reg
        ),
    ],
    "subf" => &[
        insn!(
            LowOp::SubFloat { dst, lhs, rhs },
            dst:reg:Reg, lhs:reg:Reg, rhs:reg:Reg
        ),
    ],
    "mulf" => &[
        insn!(
            LowOp::MulFloat { dst, lhs, rhs },
            dst:reg:Reg, lhs:reg:Reg, rhs:reg:Reg
        ),
    ],
    "divf" => &[
        insn!(
            LowOp::DivFloat { dst, lhs, rhs },
            dst:reg:Reg, lhs:reg:Reg, rhs:reg:Reg
        ),
    ],
    "and" => &[
        insn!(
            LowOp::And { dst, lhs, rhs },
            dst:reg:Reg, lhs:reg:Reg, rhs:reg:Reg
        ),
    ],
    "or" => &[
        insn!(
            LowOp::Or { dst, lhs, rhs },
            dst:reg:Reg, lhs:reg:Reg, rhs:reg:Reg
        ),
    ],
    "xor" => &[
        insn!(
            LowOp::Xor { dst, lhs, rhs },
            dst:reg:Reg, lhs:reg:Reg, rhs:reg:Reg
        ),
    ],
    "cmp" => &[
        insn!(
            LowOp::Compare { dst, lhs, rhs },
            dst:reg:Reg, lhs:reg:Reg, rhs:reg:Reg
        ),
    ],
    "cmps" => &[
        insn!(
            LowOp::CompareSigned { dst, lhs, rhs },
            dst:reg:Reg, lhs:reg:Reg, rhs:reg:Reg
        ),
    ],
    "cmpf" => &[
        insn!(
            LowOp::CompareFloat { dst, lhs, rhs },
            dst:reg:Reg, lhs:reg:Reg, rhs:reg:Reg
        ),
    ],
    "not" => &[
        insn!(
            LowOp::Not { dst, src },
            dst:reg:Reg, src:reg:Reg
        ),
    ],
    "fti" => &[
        insn!(
            LowOp::FloatToInt { dst, src },
            dst:reg:Reg, src:reg:Reg
        ),
    ],
    "itf" => &[
        insn!(
            LowOp::IntToFloat { dst, src },
            dst:reg:Reg, src:reg:Reg
        ),
    ],
    "ldbo" => &[
        insn!(
            LowOp::LoadBaseOffset { dst },
            dst:reg:Reg
        ),
    ],
    "ldpc" => &[
        insn!(
            LowOp::LoadProgramCounter { dst },
            dst:reg:Reg
        ),
    ],
    "nop" => &[insn!(LowOp::Nop,)],
    "halt" => &[insn!(LowOp::Halt,)],
    "ret" => &[insn!(LowOp::Return,)],
    "crash" => &[insn!(LowOp::InvalidInstruction,)],
};

#[derive(Debug)]
pub struct Op {
    pub b: u32,
    pub c: &'static [Component],
}

#[derive(Debug)]
pub enum Component {
    Reg,
    U(usize),
    I(usize),
}

pub fn find(
    task: &mut CompileTask,
    module_index: usize,
    insns: &'static [(&'static [Component], InsnMacro)],
    _span: Span,
    sub_nodes: &[Node],
) -> Result<Option<InsnMacro>> {
    let module = &mut task.modules[module_index];
    let args_len = sub_nodes.len() - 1;
    'outer: for insn in insns {
        if insn.0.len() != args_len {
            continue;
        }
        for i in 0..args_len {
            let component = &insn.0[i];
            let node = &sub_nodes[1 + i];
            match component {
                Component::Reg => {
                    let Node::Ident { span } = node else {
                        continue 'outer;
                    };
                    let s = &module.src[span.clone()];
                    if !s.starts_with('r') && !s.starts_with('R') {
                        continue 'outer;
                    }
                    let s = &s[1..];
                    let Ok(num) = s.parse::<usize>() else {
                        continue 'outer;
                    };
                    if num > 31 {
                        continue 'outer;
                    }
                }
                Component::U(bits) => {
                    let Node::UInt { span, value } = node else {
                        continue 'outer;
                    };
                    let max_value = (1 << *bits) - 1;
                    if *value > max_value {
                        return Err(Error::NotInSizeRange {
                            file: mem::take(&mut module.file),
                            src: mem::take(&mut module.src),
                            span: span.clone(),
                            range: 0..max_value as usize,
                        });
                    }
                }
                Component::I(bits) => {
                    let Node::Int { span, value } = node else {
                        continue 'outer;
                    };
                    let max_value = (1 << (*bits - 1)) - 1;
                    let min_value = -(1 << (*bits - 1));
                    if *value > max_value || *value < min_value {
                        return Err(Error::NotInI64Range {
                            file: mem::take(&mut module.file),
                            src: mem::take(&mut module.src),
                            span: span.clone(),
                            range: min_value..max_value,
                        });
                    }
                }
            }
        }
        return Ok(Some(insn.1));
    }
    Ok(None)
}

fn parse_reg(module: &mut Module, node: Node) -> Reg {
    let Node::Ident { span } = node else {
        panic!()
    };
    Reg::new((&module.src[span])[1..].parse().unwrap())
}

fn parse_uint(node: Node) -> u32 {
    let Node::UInt { value, .. } = node else {
        panic!()
    };
    value as u32
}

fn parse_int(node: Node) -> i32 {
    let Node::Int { value, .. } = node else {
        panic!()
    };
    value as i32
}

fn parse_u16(node: Node) -> u16 {
    let Node::UInt { value, .. } = node else {
        panic!()
    };
    value as u16
}
