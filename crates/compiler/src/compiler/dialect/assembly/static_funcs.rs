use std::mem;

use leviathan_ir::binary::BinaryStatic;
use phf::{phf_map, Map};

use crate::{
    compiler::{
        error::{Error, Result},
        CompileTask,
    },
    parser::Node,
    util::{get_key_by_value, source::Span},
};

use super::AssemblyLanguage;

pub type StaticFunc<'a> = fn(
    dialect: &mut AssemblyLanguage,
    task: &mut CompileTask,
    module_index: usize,
    static_index: usize,
    span: Span,
    nodes: Vec<Node>,
) -> Result<BinaryStatic>;

pub const STATIC_FUNCS: Map<&'static str, StaticFunc> = phf_map! {
    "buffer" => static_buffer,
};

fn static_buffer(
    dialect: &mut AssemblyLanguage,
    task: &mut CompileTask,
    module_index: usize,
    static_index: usize,
    span: Span,
    mut nodes: Vec<Node>,
) -> Result<BinaryStatic> {
    let module = &mut task.modules[module_index];
    let name = if task.collect_offsets {
        get_key_by_value(&dialect.static_indices, &static_index).cloned()
    } else {
        None
    };
    if nodes.len() != 2 {
        return Err(Error::InvalidCallSignature {
            file: mem::take(&mut module.file),
            src: mem::take(&mut module.src),
            span,
        });
    }
    let size = match nodes.pop().unwrap() {
        Node::Int { span, value } => {
            if value < 1 {
                return Err(Error::NotInSizeRangeFrom {
                    file: mem::take(&mut module.file),
                    src: mem::take(&mut module.src),
                    span,
                    range: 1..,
                });
            }
            value as usize
        }
        Node::UInt { span, value } => {
            if value < 1 {
                return Err(Error::NotInSizeRangeFrom {
                    file: mem::take(&mut module.file),
                    src: mem::take(&mut module.src),
                    span,
                    range: 1..,
                });
            }
            value as usize
        }
        _ => {
            return Err(Error::InvalidCallSignature {
                file: mem::take(&mut module.file),
                src: mem::take(&mut module.src),
                span,
            })
        }
    };
    Ok(BinaryStatic::FilledBuffer {
        name,
        size,
        fill: 0,
    })
}
