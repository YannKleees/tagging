use crate::{Result, Span};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Mask,
    Pseudo,
}

pub fn redact(text: &str, _spans: &[Span], _mode: Mode) -> Result<String> {
    Ok(text.to_string())
}
