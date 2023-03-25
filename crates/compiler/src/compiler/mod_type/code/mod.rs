pub mod keywords;

use std::mem;

use phf::{phf_map, Map};

use crate::{
    compiler::{
        cast,
        error::{Error, Result},
        intermediary::{Insn, IntermediaryStaticValue},
        CompileTask, Func, FuncData, Module, ModuleType, ModuleVTable, StaticData,
        UncollectedModule,
    },
    parser::{BracketType, Node},
    util::source::Span,
};

use self::keywords::{collect_fn, collect_static, collect_use};

type KeywordProc = fn(
    task: &mut CompileTask,
    module_index: usize,
    main_module: bool,
    span: Span,
    nodes: Vec<Node>,
) -> Result<()>;

const KEYWORDS: Map<&'static str, KeywordProc> = phf_map! {
    "use" => collect_use,
    "static" => collect_static,
    "fn" => collect_fn,
    "fn!" => collect_fn,
};

pub enum CodeLanguage {
    Collected { imports: Vec<Span> },
    Intermediary { imports: Vec<usize> },
}

impl Default for CodeLanguage {
    fn default() -> Self {
        Self::Collected {
            imports: Vec::with_capacity(0),
        }
    }
}

impl ModuleType for CodeLanguage {
    fn vtable(&self) -> ModuleVTable {
        ModuleVTable {
            collect,
            gen_intermediary,
        }
    }
}

fn collect(
    task: &mut CompileTask,
    module_index: usize,
    UncollectedModule { root }: UncollectedModule,
    main_module: bool,
) -> Result<()> {
    let mut module = &mut task.modules[module_index];
    for stmnt in root.into_iter().skip(1) {
        let Node::Node {
            span: stmnt_span,
            type_: BracketType::Round,
            sub_nodes: stmnt_nodes,
        } = stmnt else
        {
            return Err(Error::UnexpectedToken {
                file: take_file(module),
                src: take_src(module),
                span: stmnt.span(),
            });
        };
        if stmnt_nodes.is_empty() {
            return Err(Error::EmptyNode {
                file: take_file(module),
                src: take_src(module),
                span: stmnt_span,
            });
        }
        let Node::Ident { span: keyword_span } = &stmnt_nodes[0] else {
            return Err(Error::UnexpectedToken {
                file: take_file(module),
                src: take_src(module),
                span: stmnt_nodes[0].span(),
            });
        };
        let keyword = &module.src[keyword_span.clone()];
        let Some(keyword_proc) = KEYWORDS.get(keyword) else {
            return Err(Error::InvalidKeyword {
                file: take_file(module),
                src: take_src(module),
                span: keyword_span.clone(),
            });
        };
        (*keyword_proc)(task, module_index, main_module, stmnt_span, stmnt_nodes)?;
        module = &mut task.modules[module_index];
    }
    Ok(())
}

fn gen_intermediary(task: &mut CompileTask, module_index: usize) -> Result<()> {
    let mut module = &mut task.modules[module_index];
    let CodeLanguage::Collected { imports } = cast::<CodeLanguage>(&mut module.type_).as_mut() else {
        unreachable!()
    };
    let imports = mem::replace(imports, Vec::with_capacity(0));
    let mut new_imports = Vec::with_capacity(imports.len());
    for import_span in imports {
        let import = &module.src[import_span.clone()];
        let Some(import) = task.module_indices.get(import) else {
            return Err(Error::UnknownModule {
                file: take_file(module),
                src: take_src(module),
                span: import_span,
            });
        };
        new_imports.push(*import);
    }
    module.type_ = Box::new(CodeLanguage::Intermediary {
        imports: new_imports,
    });
    for i in 0..module.statics.len() {
        let static_ = &mut module.statics[i];
        let StaticData::Collected { node } = mem::replace(
            &mut static_.data,
            StaticData::Intermediary {
                value: IntermediaryStaticValue::Int(0),
            },
        ) else {
            unreachable!()
        };
        let value = gen_static_intermediary(module, node)?;
        module.statics[i].data = StaticData::Intermediary { value };
    }
    for i in 0..module.funcs.len() {
        let Func { data, .. } = &mut module.funcs[i];
        let FuncData::Collected { node } = mem::replace(
            data,
            FuncData::Intermediary {
                ir: Vec::with_capacity(0),
            },
        ) else {unreachable!()};
        let ir = gen_expr_intermediary(task, module_index, node)?;
        module = &mut task.modules[module_index];
        module.funcs[i].data = FuncData::Intermediary { ir };
    }
    Ok(())
}

fn gen_static_intermediary(module: &mut Module, node: Node) -> Result<IntermediaryStaticValue> {
    match node {
        Node::Ident { span } => Err(Error::UnexpectedToken {
            file: take_file(module),
            src: take_src(module),
            span,
        }),
        Node::Int { value, .. } => Ok(IntermediaryStaticValue::Int(value)),
        Node::UInt { value, .. } => Ok(IntermediaryStaticValue::UInt(value)),
        Node::Float { value, .. } => Ok(IntermediaryStaticValue::Float(value)),
        Node::String { value, .. } => Ok(IntermediaryStaticValue::String(value)),
        Node::Node {
            span: _,
            type_: _,
            sub_nodes: _,
        } => todo!(),
    }
}

fn gen_expr_intermediary(
    _task: &mut CompileTask,
    _module_index: usize,
    _expr: Node,
) -> Result<Vec<Insn>> {
    todo!()
}

fn take_file(module: &mut Module) -> String {
    mem::replace(&mut module.file, String::with_capacity(0))
}

fn take_src(module: &mut Module) -> String {
    mem::replace(&mut module.src, String::with_capacity(0))
}
