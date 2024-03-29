use std::mem;

use leviathan_ir::layers::{
    lower::{LowOp, LowerLayer, Reg},
    Coord,
};
use phf::{phf_map, Map};

use crate::{
    compiler::{
        error::{Error, Result},
        CompileTask,
    },
    parser::Node,
    util::source::Span,
};

use super::AssemblyLanguage;

pub type Macro = fn(
    data: &mut AssemblyLanguage,
    task: &mut CompileTask,
    module_index: usize,
    binary_func: &mut LowerLayer,
    span: Span,
    sub_nodes: Vec<Node>,
) -> Result<()>;

pub const MACROS: Map<&'static str, Macro> = phf_map! {
    "lea" => r#ref,
    "ref" => r#ref,
};

fn r#ref(
    dialect: &mut AssemblyLanguage,
    task: &mut CompileTask,
    module_index: usize,
    binary_func: &mut LowerLayer,
    span: Span,
    sub_nodes: Vec<Node>,
) -> Result<()> {
    let module = &mut task.modules[module_index];
    if sub_nodes.len() != 3 {
        return Err(Error::InvalidStatement {
            file: mem::take(&mut module.file),
            src: mem::take(&mut module.src),
            span,
        });
    }
    let Node::Ident { span: dst_span } = &sub_nodes[1] else {
        return Err(Error::UnexpectedToken {
            file: mem::take(&mut module.file),
            src: mem::take(&mut module.src),
            span: sub_nodes[1].span(),
        });
    };
    let dst = &module.src[dst_span.clone()];
    if !dst.starts_with('r') && !dst.starts_with('R') {
        return Err(Error::InvalidRegister {
            file: mem::take(&mut module.file),
            src: mem::take(&mut module.src),
            span: dst_span.clone(),
        });
    }
    let Ok(dst) = dst[1..].parse::<usize>() else {
        return Err(Error::InvalidRegister {
            file: mem::take(&mut module.file),
            src: mem::take(&mut module.src),
            span: dst_span.clone(),
        });
    };
    if dst > 31 {
        return Err(Error::InvalidRegister {
            file: mem::take(&mut module.file),
            src: mem::take(&mut module.src),
            span: dst_span.clone(),
        });
    }
    let dst = Reg::new(dst as u8);
    let Node::Ident { span: static_span } = &sub_nodes[2] else {
        return Err(Error::UnexpectedToken {
            file: mem::take(&mut module.file),
            src: mem::take(&mut module.src),
            span: sub_nodes[2].span(),
        });
    };
    let static_ = &module.src[static_span.clone()];
    let Some(static_) = dialect.static_indices.get(static_) else {
        return Err(Error::UnknownStaticVariable {
            file: mem::take(&mut module.file),
            src: mem::take(&mut module.src),
            span: static_span.clone(),
        });
    };
    binary_func.ops.push(LowOp::LoadStaticAddress {
        dst,
        coord: Coord {
            module: module_index,
            element: *static_,
        },
    });
    Ok(())
}
