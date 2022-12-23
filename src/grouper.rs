use crate::tokenizer::{self, Token, TokenType};
use std::ops::Range;

#[derive(Debug)]
pub enum Error {
    GroupMissmatch(TokenType, GroupType),
    UnopenedBracket(),
    UnclosedBracket(),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GroupType {
    Root,
    Round,
    Square,
    Curly,
}

pub struct OpenGroup {
    group_type: GroupType,
    start_index: usize,
    elements: Vec<GroupElement>,
}

#[derive(Debug)]
pub struct Group {
    pub group_type: GroupType,
    pub range: Range<usize>,
    pub elements: Vec<GroupElement>,
}

#[derive(Debug)]
pub enum GroupElement {
    TokenRange(Range<usize>),
    Group(Group),
}

pub fn group(tokens: &[Token]) -> Result<Group, Error> {
    let mut groups = vec![OpenGroup {
        group_type: GroupType::Root,
        start_index: 0,
        elements: Vec::with_capacity(0),
    }];
    let mut normal_index = -1_isize;
    for (index, token) in tokens.iter().enumerate() {
        match token.token_type {
            tokenizer::TokenType::LeftParen
            | tokenizer::TokenType::LeftBracket
            | tokenizer::TokenType::LeftBrace => {
                if normal_index != -1 {
                    groups
                        .last_mut()
                        .unwrap()
                        .elements
                        .push(GroupElement::TokenRange(normal_index as usize..index));
                    normal_index = -1;
                }
                groups.push(OpenGroup {
                    group_type: match token.token_type {
                        tokenizer::TokenType::LeftParen => GroupType::Round,
                        tokenizer::TokenType::LeftBracket => GroupType::Square,
                        tokenizer::TokenType::LeftBrace => GroupType::Curly,
                        _ => unreachable!(),
                    },
                    start_index: index,
                    elements: Vec::with_capacity(0),
                });
            }
            tokenizer::TokenType::RightParen
            | tokenizer::TokenType::RightBracket
            | tokenizer::TokenType::RightBrace => {
                if normal_index != -1 {
                    groups
                        .last_mut()
                        .unwrap()
                        .elements
                        .push(GroupElement::TokenRange(normal_index as usize..index));
                    normal_index = -1;
                }
                let Some(OpenGroup { group_type, start_index, elements }) = groups.pop() else {
                    return Err(Error::UnopenedBracket());
                };
                if !match token.token_type {
                    tokenizer::TokenType::RightParen => group_type == GroupType::Round,
                    tokenizer::TokenType::RightBracket => group_type == GroupType::Square,
                    tokenizer::TokenType::RightBrace => group_type == GroupType::Curly,
                    _ => unreachable!(),
                } {
                    return Err(Error::GroupMissmatch(token.token_type, group_type));
                }
                let Some(last) = groups.last_mut() else {
                    panic!("Internal error")
                };
                last.elements.push(GroupElement::Group(Group {
                    group_type,
                    range: start_index..index + 1,
                    elements,
                }));
            }
            _ => {
                if normal_index == -1 {
                    normal_index = index as isize;
                }
            }
        }
    }
    if normal_index != -1 {
        groups
            .last_mut()
            .unwrap()
            .elements
            .push(GroupElement::TokenRange(
                normal_index as usize..tokens.len(),
            ));
    }
    if groups.len() > 1 {
        return Err(Error::UnclosedBracket());
    }
    if groups.is_empty() {
        panic!("Internal error")
    }
    Ok(Group {
        group_type: GroupType::Root,
        range: 0..tokens.len(),
        elements: groups.pop().unwrap().elements,
    })
}
