use crate::parser::Token;

pub struct TokenGroup {
    bracket: Bracket,
    tokens: Vec<GroupedToken>,
}

pub enum Bracket {
    Round,
    Square,
    Curly,
}

pub enum GroupedToken {
    Token(Token),
    TokenGroup(TokenGroup),
}
