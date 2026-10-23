use regex::Regex;

use super::{Detector, scan, word_bounded};
use crate::{EntityType, Result, Span, Token};

pub struct EmailDetector {
    re: Regex,
}

impl EmailDetector {
    pub fn new() -> Result<Self> {
        let label = r"[\p{L}\p{N}](?:[\p{L}\p{N}-]*[\p{L}\p{N}])?";
        let re = Regex::new(&format!(
            r"[\p{{L}}\p{{N}}][\p{{L}}\p{{N}}._%+-]*@{label}(?:\.{label})*\.\p{{L}}{{2,24}}"
        ))?;
        Ok(Self { re })
    }
}

impl Detector for EmailDetector {
    fn detect(&self, text: &str, _: &[Token]) -> Vec<Span> {
        scan(&self.re, text, |m| {
            let bounded = word_bounded(text, m.start(), m.end()) && !text[m.end()..].starts_with('@');
            bounded.then(|| Span::rule(text, EntityType::Email, m.start(), m.end(), 0.99))
        })
    }
}
