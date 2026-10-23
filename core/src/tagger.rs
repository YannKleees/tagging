use crate::{Config, Result, Span};

pub struct Tagger;

impl Tagger {
    pub fn new(_config: &Config) -> Result<Self> {
        Ok(Self)
    }
    pub fn detect(&self, _text: &str) -> Vec<Span> {
        Vec::new()
    }
}
