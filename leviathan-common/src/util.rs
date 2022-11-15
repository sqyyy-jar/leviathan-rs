use std::fmt::{Debug, Display};

#[derive(Clone, Copy)]
pub struct TextPosition {
    pub line: u32,
    pub column: u32,
}

impl Display for TextPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}:{}", self.line, self.column).as_str())
    }
}

impl Debug for TextPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("(line {}, column {})", self.line, self.column).as_str())
    }
}
