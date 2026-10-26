use std::net::Ipv6Addr;

use regex::Regex;

use super::{Detector, has_keyword, next_char, number_bounded, prev_char, scan, window_before};
use crate::{EntityType, Result, Span, Token, resolve};

const VERSION_WORDS: &[&str] = &["versio*", "ver", "v", "build", "release", "firmware", "rev*"];

pub struct IpDetector {
    v4: Regex,
    v6: Regex,
}

impl IpDetector {
    pub fn new() -> Result<Self> {
        Ok(Self {
            v4: Regex::new(r"\d{1,3}(?:\.\d{1,3}){3}")?,
            v6: Regex::new(r"(?i)(?:[0-9a-f]{0,4}:){2,7}(?:(?:\d{1,3}\.){3}\d{1,3}|[0-9a-f]{1,4})?")?,
        })
    }

    fn ipv4(&self, text: &str) -> Vec<Span> {
        scan(&self.v4, text, |m| {
            let octets_ok = m
                .as_str()
                .split('.')
                .all(|o| o.parse::<u8>().is_ok() && (o == "0" || !o.starts_with('0')));
            let versioned = has_keyword(&window_before(text, m.start(), 12), VERSION_WORDS);
            (octets_ok && !versioned && number_bounded(text, m.start(), m.end()))
                .then(|| Span::rule(text, EntityType::IpAddress, m.start(), m.end(), 0.9))
        })
    }

    fn ipv6(&self, text: &str) -> Vec<Span> {
        let edge = |c: Option<char>| c.is_some_and(|c| c.is_alphanumeric() || c == ':');
        scan(&self.v6, text, |m| {
            let s = m.as_str();
            let groups: Vec<_> = s.split(':').filter(|g| !g.is_empty()).collect();
            // Short forms like `a::b` are far more often code or times than addresses.
            let plausible = groups.len() >= 3 || groups.iter().any(|g| g.len() == 4);
            let valid = plausible && s.parse::<Ipv6Addr>().is_ok();
            let bounded = !edge(prev_char(text, m.start())) && !edge(next_char(text, m.end()));
            (valid && bounded).then(|| Span::rule(text, EntityType::IpAddress, m.start(), m.end(), 0.95))
        })
    }
}

impl Detector for IpDetector {
    fn detect(&self, text: &str, _: &[Token]) -> Vec<Span> {
        let mut spans = self.ipv4(text);
        spans.extend(self.ipv6(text));
        resolve(spans)
    }
}
