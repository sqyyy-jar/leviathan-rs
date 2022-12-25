pub mod ast;
pub mod grouper;
pub mod tokenizer;

use std::{env, fs};

fn main() {
    let tokens =
        tokenizer::parse(&fs::read_to_string(env::args().nth(1).unwrap()).unwrap()).unwrap();
    let ast = grouper::group(&tokens).unwrap();
    println!("{:#?}", ast);
}

#[cfg(test)]
mod tests {
    mod tokenizer {
        mod number {
            #[test]
            fn test_number_positive_integer() {
                use crate::tokenizer::{self, TokenType};
                let res = tokenizer::parse(r#"1"#);
                let res = res.unwrap();
                assert!(res.len() == 1);
                assert!(res[0].token_type == TokenType::Int);
                assert!(res[0].chars.len() == 1);
            }

            #[test]
            fn test_number_negative_integer() {
                use crate::tokenizer::{self, TokenType};
                let res = tokenizer::parse(r#"-1"#);
                let res = res.unwrap();
                assert!(res.len() == 1);
                assert!(res[0].token_type == TokenType::Int);
                assert!(res[0].chars.len() == 2);
            }

            #[test]
            fn test_number_positive_float() {
                use crate::tokenizer::{self, TokenType};
                let res = tokenizer::parse(r#"1.2"#);
                let res = res.unwrap();
                assert!(res.len() == 1);
                assert!(res[0].token_type == TokenType::Float);
                assert!(res[0].chars.len() == 3);
            }

            #[test]
            fn test_number_negative_float() {
                use crate::tokenizer::{self, TokenType};
                let res = tokenizer::parse(r#"-1.2"#);
                let res = res.unwrap();
                assert!(res.len() == 1);
                assert!(res[0].token_type == TokenType::Float);
                assert!(res[0].chars.len() == 4);
            }

            #[test]
            fn test_number_positive_leading_float() {
                use crate::tokenizer::{self, TokenType};
                let res = tokenizer::parse(r#".1"#);
                let res = res.unwrap();
                assert!(res.len() == 1);
                assert!(res[0].token_type == TokenType::Float);
                assert!(res[0].chars.len() == 2);
            }

            #[test]
            fn test_number_negative_leading_float() {
                use crate::tokenizer::{self, TokenType};
                let res = tokenizer::parse(r#"-.1"#);
                let res = res.unwrap();
                assert!(res.len() == 1);
                assert!(res[0].token_type == TokenType::Float);
                assert!(res[0].chars.len() == 3);
            }

            #[test]
            fn test_number_positive_trailing_float() {
                use crate::tokenizer::{self, TokenType};
                let res = tokenizer::parse(r#"1."#);
                let res = res.unwrap();
                assert!(res.len() == 1);
                assert!(res[0].token_type == TokenType::Float);
                assert!(res[0].chars.len() == 2);
            }

            #[test]
            fn test_number_negative_trailing_float() {
                use crate::tokenizer::{self, TokenType};
                let res = tokenizer::parse(r#"-1."#);
                let res = res.unwrap();
                assert!(res.len() == 1);
                assert!(res[0].token_type == TokenType::Float);
                assert!(res[0].chars.len() == 3);
            }
        }

        #[test]
        fn test_brackets() {
            use crate::tokenizer::{self, TokenType};
            let res = tokenizer::parse(r#"()[]{}"#);
            let res = res.unwrap();
            assert!(res.len() == 6);
            assert!(res[0].token_type == TokenType::LeftParen);
            assert!(res[1].token_type == TokenType::RightParen);
            assert!(res[2].token_type == TokenType::LeftBracket);
            assert!(res[3].token_type == TokenType::RightBracket);
            assert!(res[4].token_type == TokenType::LeftBrace);
            assert!(res[5].token_type == TokenType::RightBrace);
        }

        #[test]
        fn test_whitespace() {
            use crate::tokenizer::{self, TokenType};
            let res = tokenizer::parse("0 0\t0\n0,0");
            let res = res.unwrap();
            assert!(res.len() == 5);
            assert!(res[0].token_type == TokenType::Int);
            assert!(res[1].token_type == TokenType::Int);
            assert!(res[2].token_type == TokenType::Int);
            assert!(res[3].token_type == TokenType::Int);
            assert!(res[3].line == 2);
            assert!(res[4].token_type == TokenType::Int);
        }

        mod atom {
            #[test]
            fn test_atom_ident() {
                use crate::tokenizer::{self, TokenType};
                let res = tokenizer::parse(":test :");
                let res = res.unwrap();
                assert!(res.len() == 2);
                assert!(res[0].token_type == TokenType::Atom);
                assert!(res[0].chars.len() == 5);
                assert!(res[1].token_type == TokenType::Ident);
                assert!(res[1].chars.len() == 1);
            }

            #[test]
            fn test_atom_semicolon() {
                use crate::tokenizer::{self, TokenType};
                let res = tokenizer::parse(":a;b");
                let res = res.unwrap();
                assert!(res.len() == 1);
                assert!(res[0].token_type == TokenType::Atom);
                assert!(res[0].chars.len() == 4);
            }

            #[test]
            fn test_atom_whitespace() {
                use crate::tokenizer::{self, TokenType};
                let res = tokenizer::parse(":test :test\t:test\n:test,:test");
                let res = res.unwrap();
                assert!(res.len() == 5);
                for token in res.iter().take(5) {
                    assert!(token.token_type == TokenType::Atom);
                    assert!(token.chars.len() == 5);
                }
            }

            #[test]
            fn test_atom_unicode() {
                use crate::tokenizer::{self, TokenType};
                let res = tokenizer::parse(":äöüß");
                let res = res.unwrap();
                assert!(res.len() == 1);
                assert!(res[0].token_type == TokenType::Atom);
                assert!(res[0].chars.len() == 9);
            }
        }

        mod ident {
            #[test]
            fn test_ident() {
                use crate::tokenizer::{self, TokenType};
                let res = tokenizer::parse("abc def");
                let res = res.unwrap();
                assert!(res.len() == 2);
                assert!(res[0].token_type == TokenType::Ident);
                assert!(res[0].chars.len() == 3);
                assert!(res[1].token_type == TokenType::Ident);
                assert!(res[1].chars.len() == 3);
            }

            #[test]
            fn test_ident_hyphen() {
                use crate::tokenizer::{self, TokenType};
                let res = tokenizer::parse("-");
                let res = res.unwrap();
                assert!(res.len() == 1);
                assert!(res[0].token_type == TokenType::Ident);
                assert!(res[0].chars.len() == 1);
            }

            #[test]
            fn test_ident_hyphen_dot() {
                use crate::tokenizer::{self, TokenType};
                let res = tokenizer::parse("-.");
                let res = res.unwrap();
                assert!(res.len() == 1);
                assert!(res[0].token_type == TokenType::Ident);
                assert!(res[0].chars.len() == 2);
            }

            #[test]
            fn test_ident_dot() {
                use crate::tokenizer::{self, TokenType};
                let res = tokenizer::parse(".");
                let res = res.unwrap();
                assert!(res.len() == 1);
                assert!(res[0].token_type == TokenType::Ident);
                assert!(res[0].chars.len() == 1);
            }

            #[test]
            fn test_ident_dot_text() {
                use crate::tokenizer::{self, TokenType};
                let res = tokenizer::parse(".field");
                let res = res.unwrap();
                assert!(res.len() == 1);
                assert!(res[0].token_type == TokenType::Ident);
                assert!(res[0].chars.len() == 6);
            }

            #[test]
            fn test_ident_true() {
                use crate::tokenizer::{self, TokenType};
                let res = tokenizer::parse("true");
                let res = res.unwrap();
                assert!(res.len() == 1);
                assert!(res[0].token_type == TokenType::True);
                assert!(res[0].chars.len() == 4);
            }

            #[test]
            fn test_ident_false() {
                use crate::tokenizer::{self, TokenType};
                let res = tokenizer::parse("false");
                let res = res.unwrap();
                assert!(res.len() == 1);
                assert!(res[0].token_type == TokenType::False);
                assert!(res[0].chars.len() == 5);
            }

            #[test]
            fn test_ident_unicode() {
                use crate::tokenizer::{self, TokenType};
                let res = tokenizer::parse("äöüß");
                let res = res.unwrap();
                assert!(res.len() == 1);
                assert!(res[0].token_type == TokenType::Ident);
                assert!(res[0].chars.len() == 8);
            }
        }

        #[test]
        fn test_semicolon() {
            use crate::tokenizer::{self, TokenType};
            let res = tokenizer::parse(";");
            let res = res.unwrap();
            assert!(res.len() == 1);
            assert!(res[0].token_type == TokenType::Semicolon);
            assert!(res[0].chars.len() == 1);
        }

        #[test]
        fn test_statement_add() {
            use crate::tokenizer::{self, TokenType};
            let res = tokenizer::parse("(+ 1 2)");
            let res = res.unwrap();
            assert!(res.len() == 5);
            assert!(res[0].token_type == TokenType::LeftParen);
            assert!(res[1].token_type == TokenType::Ident);
            assert!(res[2].token_type == TokenType::Int);
            assert!(res[3].token_type == TokenType::Int);
            assert!(res[4].token_type == TokenType::RightParen);
        }

        #[test]
        fn test_list() {
            use crate::tokenizer::{self, TokenType};
            let res = tokenizer::parse("[1 2 3 4 5]");
            let res = res.unwrap();
            assert!(res.len() == 7);
            assert!(res[0].token_type == TokenType::LeftBracket);
            for token in res.iter().take(6).skip(1) {
                assert!(token.token_type == TokenType::Int);
                assert!(token.chars.len() == 1);
            }
            assert!(res[6].token_type == TokenType::RightBracket);
        }

        #[test]
        fn test_map() {
            use crate::tokenizer::{self, TokenType};
            let res = tokenizer::parse("{:a 1, :b true}");
            let res = res.unwrap();
            assert!(res.len() == 6);
            assert!(res[0].token_type == TokenType::LeftBrace);
            assert!(res[1].token_type == TokenType::Atom);
            assert!(res[2].token_type == TokenType::Int);
            assert!(res[3].token_type == TokenType::Atom);
            assert!(res[4].token_type == TokenType::True);
            assert!(res[5].token_type == TokenType::RightBrace);
        }
    }

    mod grouper {
        use core::panic;

        use crate::grouper::{self, Error, GroupElement, GroupType};

        #[test]
        fn test_statement_add() {
            use crate::tokenizer;
            let res = tokenizer::parse("(+ 1 2)");
            let res = res.unwrap();
            let res = grouper::group(&res);
            let res = res.unwrap();
            assert!(res.group_type == GroupType::Root);
            assert!(res.range == (0..5));
            assert!(res.elements.len() == 1);
            let GroupElement::Group(group) = &res.elements[0] else {
                panic!("Element is TokenRange should be Group");
            };
            assert!(group.group_type == GroupType::Round);
            assert!(group.range == (0..5));
            assert!(group.elements.len() == 1);
            let GroupElement::TokenRange(range) = &group.elements[0] else {
                panic!("Element is Group should be TokenRange");
            };
            assert!(*range == (1..4));
        }

        #[test]
        fn test_list() {
            use crate::tokenizer;
            let res = tokenizer::parse("[1 2 3 4 5]");
            let res = res.unwrap();
            let res = grouper::group(&res);
            let res = res.unwrap();
            assert!(res.group_type == GroupType::Root);
            assert!(res.range == (0..7));
            assert!(res.elements.len() == 1);
            let GroupElement::Group(group) = &res.elements[0] else {
                panic!("Element is TokenRange should be Group");
            };
            assert!(group.group_type == GroupType::Square);
            assert!(group.range == (0..7));
            assert!(group.elements.len() == 1);
            let GroupElement::TokenRange(range) = &group.elements[0] else {
                panic!("Element is Group should be TokenRange");
            };
            assert!(*range == (1..6));
        }

        #[test]
        fn test_map() {
            use crate::tokenizer;
            let res = tokenizer::parse("{:a 1, :b true}");
            let res = res.unwrap();
            let res = grouper::group(&res);
            let res = res.unwrap();
            assert!(res.group_type == GroupType::Root);
            assert!(res.range == (0..6));
            assert!(res.elements.len() == 1);
            let GroupElement::Group(group) = &res.elements[0] else {
                panic!("Element is TokenRange should be Group");
            };
            assert!(group.group_type == GroupType::Curly);
            assert!(group.range == (0..6));
            assert!(group.elements.len() == 1);
            let GroupElement::TokenRange(range) = &group.elements[0] else {
                panic!("Element is Group should be TokenRange");
            };
            assert!(*range == (1..5));
        }

        #[test]
        fn test_unclosed() {
            use crate::tokenizer;
            let res = tokenizer::parse("{:a 1, :b true");
            let res = res.unwrap();
            let res = grouper::group(&res);
            let Err(Error::UnclosedBracket()) = res else {
                panic!("Error should be UnclosedBracket");
            };
        }

        #[test]
        fn test_unopened() {
            use crate::tokenizer;
            let res = tokenizer::parse(":a 1, :b true}");
            let res = res.unwrap();
            let res = grouper::group(&res);
            let Err(Error::UnopenedBracket()) = res else {
                panic!("Error should be UnopenedBracket");
            };
        }
    }
}
