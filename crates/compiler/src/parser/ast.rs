use super::{
    error::{Error, Result},
    BareModule, BracketType, Node, Token, TokenList,
};

struct StackFrame {
    index: usize,
    type_: BracketType,
    nodes: Vec<Node>,
}

pub fn build_ast(
    TokenList {
        name,
        file,
        src,
        tokens,
    }: TokenList,
) -> Result<BareModule> {
    let mut stack = vec![StackFrame {
        index: 0,
        type_: BracketType::Round,
        nodes: Vec::with_capacity(0),
    }];
    for token in tokens {
        match token {
            Token::LeftBracket { span, type_ } => {
                if stack.len() < 2 && type_ != BracketType::Round {
                    return Err(Error::IllegalTokenAtRootLevel { file, src, span });
                }
                stack.push(StackFrame {
                    index: span.start,
                    type_,
                    nodes: Vec::with_capacity(0),
                })
            }
            Token::RightBracket { span, type_ } => {
                if stack.len() < 2 {
                    return Err(Error::IllegalTokenAtRootLevel { file, src, span });
                }
                let last = stack.pop().unwrap();
                if last.type_ != type_ {
                    return Err(Error::MissmatchBrackets {
                        file,
                        src,
                        span_a: last.index..last.index + 1,
                        span_b: span,
                    });
                }
                stack.last_mut().unwrap().nodes.push(Node::Node {
                    span: last.index..span.end,
                    type_,
                    sub_nodes: last.nodes,
                });
            }
            Token::Ident { span } => {
                if stack.len() < 2 {
                    return Err(Error::IllegalTokenAtRootLevel { file, src, span });
                }
                stack.last_mut().unwrap().nodes.push(Node::Ident { span });
            }
            Token::Int { span, value } => {
                if stack.len() < 2 {
                    return Err(Error::IllegalTokenAtRootLevel { file, src, span });
                }
                stack
                    .last_mut()
                    .unwrap()
                    .nodes
                    .push(Node::Int { span, value });
            }
            Token::UInt { span, value } => {
                if stack.len() < 2 {
                    return Err(Error::IllegalTokenAtRootLevel { file, src, span });
                }
                stack
                    .last_mut()
                    .unwrap()
                    .nodes
                    .push(Node::UInt { span, value });
            }
            Token::Float { span, value } => {
                if stack.len() < 2 {
                    return Err(Error::IllegalTokenAtRootLevel { file, src, span });
                }
                stack
                    .last_mut()
                    .unwrap()
                    .nodes
                    .push(Node::Float { span, value });
            }
            Token::String { span, value } => {
                if stack.len() < 2 {
                    return Err(Error::IllegalTokenAtRootLevel { file, src, span });
                }
                stack
                    .last_mut()
                    .unwrap()
                    .nodes
                    .push(Node::String { span, value });
            }
        }
    }
    if stack.len() != 1 {
        let last = stack.pop().unwrap();
        return Err(Error::UnclosedParenthesis {
            file,
            src,
            span: last.index..last.index + 1,
        });
    }
    Ok(BareModule {
        file,
        name,
        src,
        root: stack.pop().unwrap().nodes,
    })
}
