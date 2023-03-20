use std::mem;

use phf::{phf_map, Map};

use crate::{
    compiler::{
        error::{Error, Result},
        intermediary::IntermediaryStaticValue,
        CompileTask,
    },
    parser::Node,
    util::source::Span,
};

pub type StaticFunc<'a> = fn(
    task: &mut CompileTask,
    module_index: usize,
    span: Span,
    nodes: Vec<Node>,
) -> Result<IntermediaryStaticValue>;

pub const STATIC_FUNCS: Map<&'static str, StaticFunc> = phf_map! {
    "buffer" => static_buffer,
};

fn static_buffer(
    task: &mut CompileTask,
    module_index: usize,
    span: Span,
    mut nodes: Vec<Node>,
) -> Result<IntermediaryStaticValue> {
    let module = &mut task.modules[module_index];
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
    Ok(IntermediaryStaticValue::Buffer { size })
}
