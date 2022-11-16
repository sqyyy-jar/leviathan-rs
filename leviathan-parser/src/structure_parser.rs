use std::slice::Iter;

use leviathan_common::{
    parser::source::{Node, NodeType},
    prelude::*,
    structure::{Expression, ExpressionType, Function, Structure},
    util::TextPosition,
};

pub fn parse(nodes: Vec<Node>) -> Result<Structure> {
    let mut structure = Structure::new(
        String::new(),
        Vec::with_capacity(0),
        Vec::with_capacity(0),
        Vec::with_capacity(0),
    );
    let mut iter = nodes.iter();
    parse_namespace(&mut structure, &mut iter)?;
    parse_body(&mut structure, &mut iter)?;
    Ok(structure)
}

fn parse_namespace(structure: &mut Structure, nodes: &mut Iter<Node>) -> Result<()> {
    let mut last_position = TextPosition::new(1, 0);
    loop {
        let Some(next) = nodes.next() else {
            return Err(Error::StructureExpectedNamespace(last_position));
        };
        let Node { position, value } = next;
        match value {
            NodeType::Comment(_) => {
                last_position = position.clone();
                continue;
            }
            NodeType::Node {
                operator: op,
                arguments: args,
            } => {
                if !op.eq("ns") {
                    return Err(Error::StructureExpectedNamespaceGot(
                        position.clone(),
                        op.clone(),
                    ));
                }
                if args.len() < 1 || args.len() > 2 {
                    return Err(Error::StructureInvalidNamespace(position.clone()));
                }
                let ns = &args[0];
                if let NodeType::Identifier(ns) = &ns.value {
                    structure.namespace = ns.clone();
                } else {
                    return Err(Error::StructureInvalidNamespace(position.clone()));
                }
                if args.len() != 2 {
                    break;
                }
                let ns_args = &args[1];
                if let NodeType::List(ns_args) = &ns_args.value {
                    for ns_arg in ns_args {
                        if let NodeType::Atom(ns_arg) = &ns_arg.value {
                            structure.namespace_arguments.push(ns_arg.clone());
                        } else {
                            return Err(Error::StructureInvalidNamespaceArgument(ns_arg.position));
                        }
                    }
                } else {
                    return Err(Error::StructureInvalidNamespaceArguments(position.clone()));
                }
                break;
            }
            _ => {
                return Err(Error::StructureExpectedNamespace(position.clone()));
            }
        }
    }
    Ok(())
}

fn parse_body(structure: &mut Structure, nodes: &mut Iter<Node>) -> Result<()> {
    loop {
        let Some(node) = nodes.next() else {
            break;
        };
        let Node { position, value } = node;
        match value {
            NodeType::Node {
                operator,
                arguments,
            } => match operator.as_str() {
                "fn" => {
                    parse_function(structure, arguments, position.clone())?;
                }
                _ => {
                    return Err(Error::StructureUnknownRootOperator(
                        position.clone(),
                        operator.clone(),
                    ));
                }
            },
            _ => {
                return Err(Error::StructureUnexpectedElement(position.clone()));
            }
        }
    }
    Ok(())
}

fn parse_function(
    structure: &mut Structure,
    arguments: &Vec<Node>,
    position: TextPosition,
) -> Result<()> {
    if arguments.len() < 3 || arguments.len() > 4 {
        return Err(Error::StructureWrongFunctionStructure(position));
    }
    let NodeType::Identifier(name) = &arguments[0].value else {
        return Err(Error::StructureWrongFunctionStructure(arguments[0].position.clone()));
    };
    let NodeType::List(argument_nodes) = &arguments[1].value else {
        return Err(Error::StructureWrongFunctionStructure(arguments[1].position.clone()));
    };
    if argument_nodes.len() % 2 != 0 {
        return Err(Error::StructureInvalidFunctionParameters(
            arguments[1].position.clone(),
        ));
    }
    let mut function_arguments = Vec::with_capacity(0);
    let mut argument_nodes = argument_nodes.iter();
    loop {
        let Some(key) = argument_nodes.next() else {
            break;
        };
        let NodeType::Atom(key_atom) = &key.value else {
            return Err(Error::StructureInvalidFunctionParameters(key.position));
        };
        let Some(value) = argument_nodes.next() else {
            return Err(Error::StructureInvalidFunctionParameters(key.position));
        };
        let NodeType::Identifier(value_identifier) = &value.value else {
            return Err(Error::StructureInvalidFunctionParameters(value.position));
        };
        function_arguments.push((key_atom.clone(), value_identifier.clone()));
    }
    let code;
    let mut tags = Vec::with_capacity(0);
    if arguments.len() == 3 {
        code = node_to_expression(&arguments[2]);
        if code.is_none() {
            return Err(Error::StructureWrongFunctionStructure(
                arguments[2].position.clone(),
            ));
        }
    } else {
        let NodeType::List(tags_) = &arguments[2].value else {
            return Err(Error::StructureWrongFunctionStructure(arguments[2].position.clone()));
        };
        tags = tags_
            .iter()
            .map(node_to_expression)
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect();
        code = node_to_expression(&arguments[3]);
        if code.is_none() {
            return Err(Error::StructureWrongFunctionStructure(
                arguments[3].position.clone(),
            ));
        }
    }
    structure.functions.push(Function {
        name: name.clone(),
        arguments: function_arguments,
        tags,
        code: code.unwrap(),
    });
    Ok(())
}

fn node_to_expression(node: &Node) -> Option<Expression> {
    match &node.value {
        NodeType::Node {
            operator,
            arguments,
        } => Some(Expression {
            position: node.position,
            value: ExpressionType::Invoke {
                operator: operator.clone(),
                arguments: arguments
                    .iter()
                    .map(node_to_expression)
                    .filter(Option::is_some)
                    .map(Option::unwrap)
                    .collect(),
            },
        }),
        NodeType::List(value) => Some(Expression {
            position: node.position,
            value: ExpressionType::List(
                value
                    .iter()
                    .map(|it| node_to_expression(it))
                    .filter(Option::is_some)
                    .map(Option::unwrap)
                    .collect(),
            ),
        }),
        NodeType::Map(value) => {
            let map = value
                .iter()
                .map(|(key, value)| (key.clone(), node_to_expression(value)))
                .filter(|(_, value)| value.is_some())
                .map(|(key, value)| (key, value.unwrap()))
                .collect();
            Some(Expression {
                position: node.position,
                value: ExpressionType::Map(map),
            })
        }
        NodeType::Identifier(value) => Some(Expression {
            position: node.position,
            value: ExpressionType::Identifier(value.clone()),
        }),
        NodeType::Atom(value) => Some(Expression {
            position: node.position,
            value: ExpressionType::Atom(value.clone()),
        }),
        NodeType::String(value) => Some(Expression {
            position: node.position,
            value: ExpressionType::String(value.clone()),
        }),
        NodeType::Integer(value) => Some(Expression {
            position: node.position,
            value: ExpressionType::Integer(value.clone()),
        }),
        NodeType::Float(value) => Some(Expression {
            position: node.position,
            value: ExpressionType::Float(value.clone()),
        }),
        NodeType::Bool(value) => Some(Expression {
            position: node.position,
            value: ExpressionType::Bool(value.clone()),
        }),
        _ => None,
    }
}
