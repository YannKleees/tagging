use regex::Regex;

use super::{Detector, next_char, scan, word_bounded};
use crate::{EntityType, Result, Span, Token};

/// IBAN lengths from the SWIFT IBAN registry.
#[rustfmt::skip]
const LENGTHS: &[(&str, usize)] = &[
    ("AD", 24), ("AE", 23), ("AL", 28), ("AT", 20), ("AZ", 28), ("BA", 20), ("BE", 16), ("BG", 22), ("BH", 22),
    ("BR", 29), ("BY", 28), ("CH", 21), ("CR", 22), ("CY", 28), ("CZ", 24), ("DE", 22), ("DK", 18), ("DO", 28),
    ("EE", 20), ("EG", 29), ("ES", 24), ("FI", 18), ("FO", 18), ("FR", 27), ("GB", 22), ("GE", 22), ("GI", 23),
    ("GL", 18), ("GR", 27), ("GT", 28), ("HR", 21), ("HU", 28), ("IE", 22), ("IL", 23), ("IQ", 23), ("IS", 26),
    ("IT", 27), ("JO", 30), ("KW", 30), ("KZ", 20), ("LB", 28), ("LC", 32), ("LI", 21), ("LT", 20), ("LU", 20),
    ("LV", 21), ("MC", 27), ("MD", 24), ("ME", 22), ("MK", 19), ("MR", 27), ("MT", 31), ("MU", 30), ("NL", 18),
    ("NO", 15), ("PK", 24), ("PL", 28), ("PS", 29), ("PT", 25), ("QA", 29), ("RO", 24), ("RS", 22), ("SA", 24),
    ("SC", 31), ("SE", 24), ("SI", 19), ("SK", 24), ("SM", 27), ("ST", 25), ("SV", 28), ("TL", 23), ("TN", 24),
    ("TR", 26), ("UA", 29), ("VA", 22), ("VG", 24), ("XK", 20),
];

pub struct IbanDetector {
    re: Regex,
}

impl IbanDetector {
    pub fn new() -> Result<Self> {
        Ok(Self {
            re: Regex::new(r"(?i)\b[a-z]{2}\d{2}(?:[ \u{a0}]?[a-z0-9]){11,30}")?,
        })
    }
}

impl Detector for IbanDetector {
    fn detect(&self, text: &str, _: &[Token]) -> Vec<Span> {
        scan(&self.re, text, |m| {
            let country = m.as_str()[..2].to_ascii_uppercase();
            let len = LENGTHS.iter().find(|(c, _)| *c == country)?.1;
            let end = end_after(text, m.start(), len)?;
            let iban: String = text[m.start()..end]
                .chars()
                .filter(char::is_ascii_alphanumeric)
                .collect();
            (word_bounded(text, m.start(), end) && mod97(&iban) == 1)
                .then(|| Span::rule(text, EntityType::Iban, m.start(), end, 0.99))
        })
    }
}

/// Byte offset right after the `n`th alphanumeric character from `start`, allowing single spaces.
fn end_after(text: &str, start: usize, n: usize) -> Option<usize> {
    let mut count = 0;
    for (i, c) in text[start..].char_indices() {
        let at = start + i;
        if c.is_ascii_alphanumeric() {
            count += 1;
            if count == n {
                return Some(at + 1);
            }
        } else if !matches!(c, ' ' | '\u{a0}')
            || !next_char(text, at + c.len_utf8()).is_some_and(|c| c.is_ascii_alphanumeric())
        {
            return None;
        }
    }
    None
}

/// ISO 13616 check: move the first four characters to the end, map letters to 10..35, mod 97.
fn mod97(iban: &str) -> u32 {
    let (head, tail) = iban.split_at(4);
    tail.chars()
        .chain(head.chars())
        .filter_map(|c| c.to_digit(36))
        .fold(0, |acc, d| {
            if d >= 10 {
                (acc * 100 + d) % 97
            } else {
                (acc * 10 + d) % 97
            }
        })
}

#[cfg(test)]
mod tests {
    use super::mod97;

    #[test]
    fn checksum() {
        assert_eq!(mod97("LU280019400644750000"), 1);
        assert_eq!(mod97("GB82WEST12345698765432"), 1);
        assert_ne!(mod97("LU280019400644750001"), 1);
    }
}
