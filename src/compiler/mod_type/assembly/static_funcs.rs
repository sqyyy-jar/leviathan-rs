use phf::{phf_map, Map};

use crate::{
    compiler::{
        error::{Error, Result},
        intermediary::IntermediaryStaticValue,
    },
    parser::Node,
    util::source::Span,
};

pub type StaticFunc<'a> = fn(span: Span, nodes: Vec<Node>) -> Result<IntermediaryStaticValue>;

pub const STATIC_FUNCS: Map<&'static str, StaticFunc> = phf_map! {
    "buffer" => static_buffer,
};

fn static_buffer(span: Span, mut nodes: Vec<Node>) -> Result<IntermediaryStaticValue> {
    if nodes.len() != 2 {
        return Err(Error::InvalidCallSignature { src: None, span });
    }
    let size = match nodes.pop().unwrap() {
        Node::Int { span, value } => {
            if value < 1 {
                return Err(Error::NotInSizeRangeFrom {
                    src: None,
                    span,
                    range: 1..,
                });
            }
            value as usize
        }
        Node::UInt { span, value } => {
            if value < 1 {
                return Err(Error::NotInSizeRangeFrom {
                    src: None,
                    span,
                    range: 1..,
                });
            }
            value as usize
        }
        _ => return Err(Error::InvalidCallSignature { src: None, span }),
    };
    Ok(IntermediaryStaticValue::Buffer { size })
}
