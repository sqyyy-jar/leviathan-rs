use leviathan_common::{
    parser::source::{Node, NodeType},
    prelude::*,
    structure::{Expression, ExpressionType, Function, Namespace, Structure, Type},
    util::TextPosition,
};
use std::vec::IntoIter;

pub fn parse(nodes: Vec<Node>) -> Result<Structure> {
    let mut structure = Structure::new(
        Namespace::new(),
        Vec::with_capacity(0),
        Vec::with_capacity(0),
    );
    let mut iter = nodes.into_iter();
    parse_namespace(&mut structure, &mut iter)?;
    parse_body(&mut structure, &mut iter)?;
    Ok(structure)
}

fn parse_namespace(structure: &mut Structure, nodes: &mut IntoIter<Node>) -> Result<()> {
    let mut last_position = TextPosition::new(1, 0);
    loop {
        let Some(next) = nodes.next() else {
            return Err(Error::StructureExpectedNamespace(last_position));
        };
        let Node { position, value } = next;
        match value {
            NodeType::Comment(_) => {
                last_position = position;
                continue;
            }
            NodeType::Node {
                operator,
                mut arguments,
            } => {
                if !operator.eq("ns") {
                    return Err(Error::StructureExpectedNamespaceGot(position, operator));
                }
                if arguments.len() < 1 || arguments.len() > 2 {
                    return Err(Error::StructureInvalidNamespace(position));
                }
                let namespace_node = arguments.remove(0);
                if let NodeType::Identifier(namespace_string) = namespace_node.value {
                    parse_namespace_string(structure, namespace_string)?;
                } else {
                    return Err(Error::StructureInvalidNamespace(position));
                }
                if arguments.len() != 2 {
                    break;
                }
                let namespace_tags = arguments.remove(0);
                let NodeType::List(namespace_tags) = namespace_tags.value else {
                    return Err(Error::StructureInvalidNamespaceArguments(position));
                };
                for namespace_tag in namespace_tags {
                    if let NodeType::Atom(namespace_tag) = namespace_tag.value {
                        structure.namespace.tags.push(namespace_tag);
                    } else {
                        return Err(Error::StructureInvalidNamespaceTags(namespace_tag.position));
                    }
                }
                break;
            }
            _ => {
                return Err(Error::StructureExpectedNamespace(position));
            }
        }
    }
    Ok(())
}

fn parse_namespace_string(structure: &mut Structure, namespace: String) -> Result<()> {
    for namespace_pacakge in namespace
        .split_terminator('/')
        .filter(|it| !it.is_empty())
        .map(str::to_string)
    {
        structure.namespace.packages.push(namespace_pacakge);
    }
    Ok(())
}

fn parse_body(structure: &mut Structure, nodes: &mut IntoIter<Node>) -> Result<()> {
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
                    parse_function(structure, arguments, position)?;
                }
                _ => {
                    return Err(Error::StructureUnknownRootOperator(position, operator));
                }
            },
            _ => {
                return Err(Error::StructureUnexpectedElement(position));
            }
        }
    }
    Ok(())
}

fn parse_function(
    structure: &mut Structure,
    mut arguments: Vec<Node>,
    position: TextPosition,
) -> Result<()> {
    if arguments.len() < 3 || arguments.len() > 5 {
        return Err(Error::StructureWrongFunctionStructure(position));
    }
    let arg_identifier = arguments.remove(0);
    let NodeType::Identifier(function_name) = arg_identifier.value else {
                return Err(Error::StructureWrongFunctionStructure(arg_identifier.position));
    };
    let arg_arguments = arguments.remove(0);
    let NodeType::List(argument_nodes) = arg_arguments.value else {
                return Err(Error::StructureWrongFunctionStructure(arg_arguments.position));
    };
    if argument_nodes.len() % 2 != 0 {
        return Err(Error::StructureInvalidFunctionParameters(
            arg_arguments.position,
        ));
    }
    let mut function_arguments = Vec::with_capacity(0);
    let mut argument_nodes = argument_nodes.into_iter();
    loop {
        let Some(key) = argument_nodes.next() else {
            break;
        };
        let NodeType::Atom(key_atom) = key.value else {
            return Err(Error::StructureInvalidFunctionParameters(key.position));
        };
        let Some(value) = argument_nodes.next() else {
            return Err(Error::StructureInvalidFunctionParameters(key.position));
        };
        let NodeType::Identifier(value_identifier) = value.value else {
            return Err(Error::StructureInvalidFunctionParameters(value.position));
        };
        function_arguments.push((key_atom, value_identifier));
    }
    let mut function_return_type = Type::Unit;
    let function_code;
    let mut function_tags = Vec::with_capacity(0);
    if arguments.len() == 1 {
        let argument_code = arguments.remove(0);
        let argument_code_position = argument_code.position;
        function_code = node_to_expression(argument_code);
        if function_code.is_none() {
            return Err(Error::StructureWrongFunctionStructure(
                argument_code_position,
            ));
        }
    } else {
        let first = arguments.remove(0);
        match first.value {
            NodeType::Identifier(return_type_string) => {
                function_return_type = match return_type_string.as_str() {
                    "unit" => Type::Unit,
                    "bool" => Type::Bool,
                    "int" => Type::Int,
                    "float" => Type::Float,
                    "str" => Type::String,
                    "atom" => Type::Atom,
                    "list" => Type::List,
                    "map" => Type::Map,
                    _ => {
                        return Err(Error::StructureUnknownType(
                            first.position,
                            return_type_string,
                        ));
                    }
                };
                let second = arguments.remove(0);
                let argument_code_position;
                if arguments.len() == 0 {
                    argument_code_position = second.position;
                    function_code = node_to_expression(second);
                } else {
                    let NodeType::List(tags) = second.value else {
                        return Err(Error::StructureWrongFunctionStructure(second.position));
                    };
                    function_tags = tags
                        .into_iter()
                        .map(node_to_expression)
                        .filter(Option::is_some)
                        .map(Option::unwrap)
                        .collect();
                    let argument_code = arguments.remove(0);
                    argument_code_position = argument_code.position;
                    function_code = node_to_expression(argument_code);
                }
                if function_code.is_none() {
                    return Err(Error::StructureWrongFunctionStructure(
                        argument_code_position,
                    ));
                }
            }
            NodeType::List(tags) => {
                if arguments.len() != 1 {
                    return Err(Error::StructureWrongFunctionStructure(first.position));
                }
                function_tags = tags
                    .into_iter()
                    .map(node_to_expression)
                    .filter(Option::is_some)
                    .map(Option::unwrap)
                    .collect();
                let argument_code = arguments.remove(0);
                let argument_code_position = argument_code.position;
                function_code = node_to_expression(argument_code);
                if function_code.is_none() {
                    return Err(Error::StructureWrongFunctionStructure(
                        argument_code_position,
                    ));
                }
            }
            _ => {
                return Err(Error::StructureWrongFunctionStructure(first.position));
            }
        }
    }
    structure.functions.push(Function {
        name: function_name,
        arguments: function_arguments,
        return_type: function_return_type,
        tags: function_tags,
        code: function_code.unwrap(),
    });
    Ok(())
}

fn node_to_expression(node: Node) -> Option<Expression> {
    match node.value {
        NodeType::Node {
            operator,
            arguments,
        } => Some(Expression {
            position: node.position,
            value: ExpressionType::Invoke {
                operator,
                arguments: arguments
                    .into_iter()
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
                    .into_iter()
                    .map(node_to_expression)
                    .filter(Option::is_some)
                    .map(Option::unwrap)
                    .collect(),
            ),
        }),
        NodeType::Map(value) => {
            let map = value
                .into_iter()
                .map(|(key, value)| (key, node_to_expression(value)))
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
            value: ExpressionType::Identifier(value),
        }),
        NodeType::Atom(value) => Some(Expression {
            position: node.position,
            value: ExpressionType::Atom(value),
        }),
        NodeType::String(value) => Some(Expression {
            position: node.position,
            value: ExpressionType::String(value),
        }),
        NodeType::Integer(value) => Some(Expression {
            position: node.position,
            value: ExpressionType::Integer(value),
        }),
        NodeType::Float(value) => Some(Expression {
            position: node.position,
            value: ExpressionType::Float(value),
        }),
        NodeType::Bool(value) => Some(Expression {
            position: node.position,
            value: ExpressionType::Bool(value),
        }),
        _ => None,
    }
}
