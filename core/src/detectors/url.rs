use regex::Regex;

use super::{Detector, prev_char, scan};
use crate::{EntityType, Result, Span, Token};

pub struct UrlDetector {
    re: Regex,
}

impl UrlDetector {
    pub fn new() -> Result<Self> {
        Ok(Self {
            re: Regex::new(r#"(?i)(?:https?://|www\.)[^\s<>"'`{}|\\^\[\]]+"#)?,
        })
    }
}

impl Detector for UrlDetector {
    fn detect(&self, text: &str, _: &[Token]) -> Vec<Span> {
        scan(&self.re, text, |m| {
            if prev_char(text, m.start()).is_some_and(char::is_alphanumeric) {
                return None;
            }
            let url = trim_trailing(m.as_str());
            let host = url.split_once("://").map_or(url, |(_, rest)| rest);
            let host = host.split(['/', '?', '#']).next().unwrap_or_default();
            let valid = host.contains('.') && !host.starts_with('.') && !host.ends_with('.');
            valid.then(|| Span::rule(text, EntityType::Url, m.start(), m.start() + url.len(), 0.95))
        })
    }
}

/// Drops sentence punctuation after a URL, and a closing parenthesis unless the URL opened one.
fn trim_trailing(url: &str) -> &str {
    let mut url = url;
    loop {
        let trimmed = url.trim_end_matches(['.', ',', ';', ':', '!', '?', '\'', '"']);
        let trimmed = match trimmed.strip_suffix(')') {
            Some(t) if trimmed.matches('(').count() < trimmed.matches(')').count() => t,
            _ => trimmed,
        };
        if trimmed.len() == url.len() {
            return url;
        }
        url = trimmed;
    }
}
