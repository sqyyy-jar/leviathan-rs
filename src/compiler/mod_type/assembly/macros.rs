#![allow(clippy::ptr_arg)]

use phf::{phf_map, Map};
use urban_common::opcodes::L5_HALT;

use crate::{
    compiler::{
        error::{Error, Result},
        intermediary::Insn,
        CompileTask,
    },
    parser::Node,
    util::source::Span,
};

pub type Macro = fn(
    task: &mut CompileTask,
    module_index: usize,
    ir: &mut Vec<Insn>,
    span: Span,
    sub_nodes: Vec<Node>,
) -> Result<()>;

pub const MACROS: Map<&'static str, Macro> = phf_map! {
    "halt" => halt,
};

fn halt(
    _task: &mut CompileTask,
    _module_index: usize,
    ir: &mut Vec<Insn>,
    span: Span,
    sub_nodes: Vec<Node>,
) -> Result<()> {
    if sub_nodes.len() != 1 {
        return Err(Error::InvalidCallSignature { src: None, span });
    }
    ir.push(Insn::Raw(L5_HALT));
    Ok(())
}
