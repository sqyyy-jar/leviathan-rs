use super::{
    error::{Error, Result},
    BareModule, Node, Token, TokenList,
};

struct StackFrame {
    index: usize,
    nodes: Vec<Node>,
}

pub fn build_ast(name: String, TokenList { src, tokens }: TokenList) -> Result<BareModule> {
    let mut stack = vec![StackFrame {
        index: 0,
        nodes: Vec::with_capacity(0),
    }];
    for token in tokens {
        match token {
            Token::LeftParen { span } => stack.push(StackFrame {
                index: span.start,
                nodes: Vec::with_capacity(0),
            }),
            Token::RightParen { span } => {
                if stack.len() < 2 {
                    return Err(Error::IllegalTokenAtRootLevel { span });
                }
                let last = stack.pop().unwrap();
                stack.last_mut().unwrap().nodes.push(Node::Node {
                    span: last.index..span.end,
                    sub_nodes: last.nodes,
                });
            }
            Token::Ident { span } => {
                if stack.len() < 2 {
                    return Err(Error::IllegalTokenAtRootLevel { span });
                }
                stack.last_mut().unwrap().nodes.push(Node::Ident { span });
            }
            Token::Int { span, value } => {
                if stack.len() < 2 {
                    return Err(Error::IllegalTokenAtRootLevel { span });
                }
                stack
                    .last_mut()
                    .unwrap()
                    .nodes
                    .push(Node::Int { span, value });
            }
            Token::UInt { span, value } => {
                if stack.len() < 2 {
                    return Err(Error::IllegalTokenAtRootLevel { span });
                }
                stack
                    .last_mut()
                    .unwrap()
                    .nodes
                    .push(Node::UInt { span, value });
            }
            Token::Float { span, value } => {
                if stack.len() < 2 {
                    return Err(Error::IllegalTokenAtRootLevel { span });
                }
                stack
                    .last_mut()
                    .unwrap()
                    .nodes
                    .push(Node::Float { span, value });
            }
            Token::String { span, value } => {
                if stack.len() < 2 {
                    return Err(Error::IllegalTokenAtRootLevel { span });
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
            span: last.index..last.index + 1,
        });
    }
    Ok(BareModule {
        name,
        src,
        root: stack.pop().unwrap().nodes,
    })
}
