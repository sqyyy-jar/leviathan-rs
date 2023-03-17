use phf::{phf_map, Map};
use urban_common::opcodes::{L0_ADD, L2_ADD, L5_HALT};

use crate::{
    compiler::{
        error::{Error, Result},
        intermediary::Insn,
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
            opc |= (num << offset) as u32;
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

pub fn find<'a>(
    insns: &'a [Op],
    src: &str,
    _span: Span,
    sub_nodes: &[Node],
) -> Result<Option<&'a Op>> {
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
                    let s = &src[span.clone()];
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
                            src: None,
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
                            src: None,
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
