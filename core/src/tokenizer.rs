#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'a> {
    pub text: &'a str,
    pub start: usize,
    pub end: usize,
}

pub fn tokenize(_text: &str) -> Vec<Token<'_>> {
    Vec::new()
}
