use regex::Regex;

use super::{Detector, digits, luhn_valid, number_bounded, scan};
use crate::{EntityType, Result, Span, Token};

pub struct CardDetector {
    re: Regex,
}

impl CardDetector {
    pub fn new() -> Result<Self> {
        Ok(Self {
            re: Regex::new(r"\b\d(?:[ \-]?\d){12,18}")?,
        })
    }
}

impl Detector for CardDetector {
    fn detect(&self, text: &str, _: &[Token]) -> Vec<Span> {
        scan(&self.re, text, |m| {
            // Try every digit-group boundary, preferring the common 16-digit length, so trailing
            // numbers such as an expiry date are not swallowed.
            let s = m.as_str();
            let ends: Vec<usize> = (1..=s.len())
                .filter(|&i| i == s.len() || s[i..].starts_with([' ', '-']))
                .map(|i| m.start() + i)
                .filter(|&e| number_bounded(text, m.start(), e) && card_valid(&digits(&text[m.start()..e])))
                .collect();
            let end = ends
                .iter()
                .find(|&&e| digits(&text[m.start()..e]).len() == 16)
                .or(ends.last())?;
            Some(Span::rule(text, EntityType::CreditCard, m.start(), *end, 0.98))
        })
    }
}

/// Luhn plus a known issuer prefix and length, which removes most Luhn-valid order numbers.
fn card_valid(d: &[u8]) -> bool {
    let prefix = |n: usize| d.iter().take(n).fold(0u32, |acc, &x| acc * 10 + u32::from(x));
    let len = d.len();
    let brand_ok = match d.first() {
        Some(4) => matches!(len, 13 | 16 | 19),
        Some(2) => (2221..=2720).contains(&prefix(4)) && len == 16,
        Some(5) => match prefix(2) {
            51..=55 => len == 16,
            50 | 56..=59 => (12..=19).contains(&len),
            _ => false,
        },
        Some(3) => match prefix(2) {
            34 | 37 => len == 15,
            36 | 38 | 39 => (14..=19).contains(&len),
            30 => (300..=305).contains(&prefix(3)) && (14..=19).contains(&len),
            35 => (3528..=3589).contains(&prefix(4)) && (16..=19).contains(&len),
            _ => false,
        },
        Some(6) => (12..=19).contains(&len),
        _ => false,
    };
    brand_ok && luhn_valid(d)
}
