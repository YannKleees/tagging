use regex::Regex;

use super::{Detector, digits, luhn_valid, number_bounded, scan};
use crate::{EntityType, Result, Span, Token};

pub struct NationalIdDetector {
    re: Regex,
}

impl NationalIdDetector {
    pub fn new() -> Result<Self> {
        let sp = r"[ \u{a0}]?";
        Ok(Self {
            re: Regex::new(&format!(
                r"\b(?:19|20)\d{{2}}{sp}\d{{2}}{sp}\d{{2}}{sp}\d{{3}}{sp}\d{{2}}"
            ))?,
        })
    }
}

impl Detector for NationalIdDetector {
    fn detect(&self, text: &str, _: &[Token]) -> Vec<Span> {
        scan(&self.re, text, |m| {
            (number_bounded(text, m.start(), m.end()) && matricule_valid(&digits(m.as_str())))
                .then(|| Span::rule(text, EntityType::NationalId, m.start(), m.end(), 0.99))
        })
    }
}

/// Luxembourg matricule: YYYYMMDD, three serial digits, a Luhn digit and a Verhoeff digit, both
/// computed over the first eleven digits.
fn matricule_valid(d: &[u8]) -> bool {
    let &[y1, y2, y3, y4, m1, m2, d1, d2, _, _, _, luhn, verhoeff] = d else {
        return false;
    };
    let year = [y1, y2, y3, y4].iter().fold(0u32, |acc, &x| acc * 10 + u32::from(x));
    let month = u32::from(m1 * 10 + m2);
    let day = u32::from(d1 * 10 + d2);
    let base = &d[..11];
    super::date::valid_date(year, month, day)
        && luhn_valid(&[base, &[luhn]].concat())
        && verhoeff_valid(&[base, &[verhoeff]].concat())
}

const D: [[u8; 10]; 10] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    [1, 2, 3, 4, 0, 6, 7, 8, 9, 5],
    [2, 3, 4, 0, 1, 7, 8, 9, 5, 6],
    [3, 4, 0, 1, 2, 8, 9, 5, 6, 7],
    [4, 0, 1, 2, 3, 9, 5, 6, 7, 8],
    [5, 9, 8, 7, 6, 0, 4, 3, 2, 1],
    [6, 5, 9, 8, 7, 1, 0, 4, 3, 2],
    [7, 6, 5, 9, 8, 2, 1, 0, 4, 3],
    [8, 7, 6, 5, 9, 3, 2, 1, 0, 4],
    [9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
];
const P: [[u8; 10]; 8] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    [1, 5, 7, 6, 2, 8, 3, 0, 9, 4],
    [5, 8, 0, 3, 7, 9, 6, 1, 4, 2],
    [8, 9, 1, 6, 0, 4, 3, 5, 2, 7],
    [9, 4, 5, 3, 1, 2, 6, 8, 7, 0],
    [4, 2, 8, 6, 5, 7, 3, 9, 0, 1],
    [2, 7, 9, 3, 8, 0, 6, 4, 1, 5],
    [7, 0, 4, 6, 9, 1, 3, 2, 5, 8],
];

fn verhoeff_valid(d: &[u8]) -> bool {
    d.iter().rev().enumerate().fold(0u8, |c, (i, &x)| {
        D[usize::from(c)][usize::from(P[i % 8][usize::from(x)])]
    }) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verhoeff() {
        assert!(verhoeff_valid(&digits("2363")));
        assert!(!verhoeff_valid(&digits("2364")));
    }

    #[test]
    fn matricule() {
        assert!(matricule_valid(&digits("1985031212307")));
        assert!(!matricule_valid(&digits("198503121230")));
    }
}
