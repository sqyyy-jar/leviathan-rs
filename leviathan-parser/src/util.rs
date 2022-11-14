pub fn is_valid_ident_char(c: char) -> bool {
    if c.is_whitespace() || c == ',' {
        return false;
    }
    match c {
        '[' | ']' | '{' | '}' | '(' | '"' | '\'' | '`' => false,
        _ => true,
    }
}

pub fn is_closing_bracket(c: char) -> bool {
    c == ')' || c == ']' || c == '}'
}

pub fn is_whitespace(c: char) -> bool {
    c.is_ascii_whitespace() || c == ','
}
