use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cannot read {}: {source}", path.display())]
    Io { path: PathBuf, source: std::io::Error },
    #[error("invalid config: {0}")]
    Config(#[from] serde_json::Error),
    #[error("invalid pattern: {0}")]
    Pattern(#[from] regex::Error),
    #[error("span {start}..{end} is out of bounds or not on a character boundary")]
    InvalidSpan { start: usize, end: usize },
    #[error("spans {}..{} and {}..{} overlap", .0.0, .0.1, .1.0, .1.1)]
    OverlappingSpans((usize, usize), (usize, usize)),
}

pub type Result<T> = std::result::Result<T, Error>;
