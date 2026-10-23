use crate::{EntityType, Span, Token, detectors::Detector};

#[derive(Default)]
pub struct Gazetteer;

impl Gazetteer {
    pub fn new() -> Self {
        Self
    }
    pub fn builtin() -> Self {
        Self
    }
    pub fn add_list(&mut self, _src: &str, _kind: EntityType) {}
    pub fn add_words(&mut self, _words: &[&str], _kind: EntityType) {}
}

impl Detector for Gazetteer {
    fn detect(&self, _: &str, _: &[Token]) -> Vec<Span> {
        Vec::new()
    }
}
