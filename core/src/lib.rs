pub mod config;
pub mod detectors;
pub mod error;
pub mod gazetteer;
mod names;
#[cfg(feature = "python")]
mod python;
pub mod redact;
pub mod resolve;
pub mod span;
pub mod tagger;
pub mod tokenizer;
#[cfg(feature = "wasm")]
mod wasm;

pub use config::{Config, DetectorKind};
pub use error::{Error, Result};
pub use redact::{Mode, redact};
pub use resolve::resolve;
pub use span::{EntityType, Source, Span};
pub use tagger::Tagger;
pub use tokenizer::{Token, tokenize};
