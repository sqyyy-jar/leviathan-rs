#![allow(clippy::ptr_arg)]

use phf::{phf_map, Map};

use crate::{
    compiler::{
        error::{Error, Result},
        intermediary::{Insn, IntermediaryDependencyPath, IntermediaryStatic},
    },
    parser::Node,
    util::source::Span,
};

pub type Macro = fn(
    src: &str,
    ir: &mut Vec<Insn>,
    deps: &mut Vec<IntermediaryDependencyPath>,
    ir_statics: &mut Vec<IntermediaryStatic>,
    span: Span,
    sub_nodes: Vec<Node>,
) -> Result<()>;

pub const MACROS: Map<&'static str, Macro> = phf_map! {
    "halt" => halt,
};

fn halt(
    _src: &str,
    ir: &mut Vec<Insn>,
    _deps: &mut Vec<IntermediaryDependencyPath>,
    _ir_statics: &mut Vec<IntermediaryStatic>,
    span: Span,
    sub_nodes: Vec<Node>,
) -> Result<()> {
    if sub_nodes.len() != 1 {
        return Err(Error::InvalidCallSignature { src: None, span });
    }
    ir.push(Insn::Halt);
    Ok(())
}
