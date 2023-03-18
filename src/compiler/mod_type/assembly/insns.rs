use std::mem;

use phf::{phf_map, Map};
use urban_common::opcodes::{
    L0_ADD, L0_DIV, L0_DIVS, L0_LDR, L0_MOV, L0_MOVS, L0_MUL, L0_REM, L0_REMS, L0_STR, L0_SUB,
    L1_LDR, L1_STR, L2_ADD, L2_ADDF, L2_DIV, L2_DIVF, L2_DIVS, L2_MUL, L2_MULF, L2_REM, L2_REMS,
    L2_SUB, L2_SUBF, L3_MOV, L5_HALT,
};

use crate::{
    compiler::{
        error::{Error, Result},
        intermediary::Insn,
        CompileTask,
    },
    parser::Node,
    util::source::Span,
};

use Component::*;

pub const INSN_MACROS: Map<&'static str, &[Op]> = phf_map! {
    "add" =>  &[
        Op { b: L0_ADD, c: &[Reg, Reg, U(17)] },
        Op { b: L2_ADD, c: &[Reg, Reg, Reg] },
    ],
    "sub" => &[
        Op { b: L0_SUB, c: &[Reg, Reg, U(17)] },
        Op { b: L2_SUB, c: &[Reg, Reg, Reg] },
    ],
    "mul" => &[
        Op { b: L0_MUL, c: &[Reg, Reg, U(17)] },
        Op { b: L2_MUL, c: &[Reg, Reg, Reg] },
    ],
    "div" => &[
        Op { b: L0_DIV, c: &[Reg, Reg, U(17)] },
        Op { b: L2_DIV, c: &[Reg, Reg, Reg] },
    ],
    "rem" => &[
        Op { b: L0_REM, c: &[Reg, Reg, U(17)] },
        Op { b: L2_REM, c: &[Reg, Reg, Reg] },
    ],
    "divs" => &[
        Op { b: L0_DIVS, c: &[Reg, Reg, U(17)] },
        Op { b: L2_DIVS, c: &[Reg, Reg, Reg] },
    ],
    "rems" => &[
        Op { b: L0_REMS, c: &[Reg, Reg, U(17)] },
        Op { b: L2_REMS, c: &[Reg, Reg, Reg] },
    ],
    "ldr" => &[
        Op { b: L0_LDR, c: &[Reg, I(22)] },
        Op { b: L1_LDR, c: &[Reg, Reg, I(11)] },
    ],
    "str" => &[
        Op { b: L0_STR, c: &[I(22), Reg] },
        Op { b: L1_STR, c: &[Reg, Reg, I(11)] },
    ],
    "mov" => &[
        Op { b: L0_MOV, c: &[Reg, U(22)] },
        Op { b: L3_MOV, c: &[Reg, Reg] },
    ],
    "movs" => &[Op { b: L0_MOVS, c: &[Reg, I(22)] }],
    "addf" =>  &[Op { b: L2_ADDF, c: &[Reg, Reg, Reg] }],
    "subf" => &[Op { b: L2_SUBF, c: &[Reg, Reg, Reg] }],
    "mulf" => &[Op { b: L2_MULF, c: &[Reg, Reg, Reg] }],
    "divf" => &[Op { b: L2_DIVF, c: &[Reg, Reg, Reg] }],
    "halt" => &[Op { b: L5_HALT, c: &[] }],
};

#[derive(Debug)]
pub struct Op {
    pub b: u32,
    pub c: &'static [Component],
}

impl Op {
    pub fn gen(&self, src: &str, ir: &mut Vec<Insn>, sub_nodes: Vec<Node>) -> Result<()> {
        let mut opc = self.b;
        let mut offset: usize = self.c.iter().map(|it| it.len()).sum();
        for i in 0..self.c.len() {
            let c = &self.c[i];
            let node = &sub_nodes[i + 1];
            offset -= c.len();
            let num = match c {
                Component::Reg => {
                    let Node::Ident { span } = node else {
                        panic!()
                    };
                    let s = &src[span.clone()];
                    let s = &s[1..];
                    s.parse::<usize>().unwrap()
                }
                Component::U(_) => {
                    let Node::UInt { value, .. } = node else {
                        panic!()
                    };
                    *value as usize
                }
                Component::I(_) => {
                    let Node::Int { value, .. } = node else {
                        panic!()
                    };
                    *value as usize
                }
            };
            let pattern = (1 << c.len()) - 1;
            opc |= ((num & pattern) << offset) as u32;
        }
        ir.push(Insn::Raw(opc));
        Ok(())
    }
}

#[derive(Debug)]
pub enum Component {
    Reg,
    U(usize),
    I(usize),
}

impl Component {
    pub fn len(&self) -> usize {
        match self {
            Component::Reg => 5,
            Component::U(bits) => *bits,
            Component::I(bits) => *bits,
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub fn find(
    task: &mut CompileTask,
    module_index: usize,
    insns: &'static [Op],
    _span: Span,
    sub_nodes: &[Node],
) -> Result<Option<&'static Op>> {
    let module = &mut task.modules[module_index];
    let args_len = sub_nodes.len() - 1;
    'outer: for insn in insns {
        if insn.c.len() != args_len {
            continue;
        }
        for i in 0..args_len {
            let component = &insn.c[i];
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
        return Ok(Some(insn));
    }
    Ok(None)
}
