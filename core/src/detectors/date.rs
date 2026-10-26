use regex::{Captures, Regex};

use super::{Detector, has_keyword, number_bounded, window_after, window_before, word_bounded};
use crate::{EntityType, Result, Span, Token, resolve};

#[rustfmt::skip]
const MONTHS: &[(&str, u32)] = &[
    ("january", 1), ("jan", 1), ("janvier", 1), ("janv", 1), ("januar", 1), ("jänner", 1),
    ("february", 2), ("feb", 2), ("février", 2), ("fevrier", 2), ("févr", 2), ("fév", 2), ("februar", 2),
    ("march", 3), ("mar", 3), ("mars", 3), ("märz", 3), ("maerz", 3), ("mäerz", 3), ("mrz", 3),
    ("april", 4), ("apr", 4), ("avril", 4), ("avr", 4), ("abrëll", 4),
    ("may", 5), ("mai", 5), ("mee", 5),
    ("june", 6), ("jun", 6), ("juin", 6), ("juni", 6),
    ("july", 7), ("jul", 7), ("juillet", 7), ("juil", 7), ("juli", 7),
    ("august", 8), ("aug", 8), ("août", 8), ("aout", 8),
    ("september", 9), ("sep", 9), ("sept", 9), ("septembre", 9),
    ("october", 10), ("oct", 10), ("octobre", 10), ("oktober", 10), ("okt", 10),
    ("november", 11), ("nov", 11), ("novembre", 11),
    ("december", 12), ("dec", 12), ("décembre", 12), ("decembre", 12), ("déc", 12), ("dezember", 12), ("dez", 12),
];
const BIRTH_WORDS: &[&str] = &[
    "born", "birth*", "dob", "d.o.b", "né", "née", "naiss*", "geboren", "geb", "geburt*", "gebuer*",
];

pub struct DateDetector {
    numeric: Regex,
    iso: Regex,
    day_month: Regex,
    month_day: Regex,
}

impl DateDetector {
    pub fn new() -> Result<Self> {
        let mut names: Vec<&str> = MONTHS.iter().map(|(m, _)| *m).collect();
        names.sort_by_key(|m| std::cmp::Reverse(m.chars().count()));
        let months = names.iter().map(|m| regex::escape(m)).collect::<Vec<_>>().join("|");
        Ok(Self {
            numeric: Regex::new(r"(\d{1,2})([./-])(\d{1,2})([./-])(\d{4}|\d{2})")?,
            iso: Regex::new(r"(\d{4})-(\d{2})-(\d{2})")?,
            day_month: Regex::new(&format!(
                r"(?i)(\d{{1,2}})(?:\.|er|st|nd|rd|th)?\s+(?:of\s+)?({months})\.?,?\s+(\d{{4}})"
            ))?,
            month_day: Regex::new(&format!(
                r"(?i)({months})\.?\s+(\d{{1,2}})(?:st|nd|rd|th)?,?\s+(\d{{4}})"
            ))?,
        })
    }
}

impl Detector for DateDetector {
    fn detect(&self, text: &str, _: &[Token]) -> Vec<Span> {
        let mut spans = Vec::new();
        find(text, &self.numeric, number_bounded, &mut spans, |c| {
            (c[2] == c[4]).then(|| (expand_year(&c[5]), num(&c[3]), num(&c[1]), c[5].len() == 2))
        });
        find(text, &self.iso, number_bounded, &mut spans, |c| {
            Some((num(&c[1]), num(&c[2]), num(&c[3]), false))
        });
        find(text, &self.day_month, word_bounded, &mut spans, |c| {
            Some((num(&c[3]), month(&c[2])?, num(&c[1]), false))
        });
        find(text, &self.month_day, word_bounded, &mut spans, |c| {
            Some((num(&c[3]), month(&c[1])?, num(&c[2]), false))
        });
        resolve(spans)
    }
}

/// `parse` returns year, month, day and whether the year was written with two digits.
fn find(
    text: &str,
    re: &Regex,
    bounded: fn(&str, usize, usize) -> bool,
    out: &mut Vec<Span>,
    parse: impl Fn(&Captures) -> Option<(u32, u32, u32, bool)>,
) {
    for c in re.captures_iter(text) {
        let (Some(m), Some((year, month, day, short_year))) = (c.get(0), parse(&c)) else {
            continue;
        };
        if !bounded(text, m.start(), m.end()) || !valid_date(year, month, day) {
            continue;
        }
        // German and Luxembourgish put the verb last: "am 03.02.1961 in Luxemburg geboren".
        let birth = has_keyword(&window_before(text, m.start(), 40), BIRTH_WORDS)
            || has_keyword(&window_after(text, m.end(), 30), BIRTH_WORDS);
        let (kind, score) = match (birth, short_year) {
            (true, false) => (EntityType::DateOfBirth, 0.95),
            (true, true) => (EntityType::DateOfBirth, 0.9),
            (false, false) => (EntityType::Date, 0.4),
            (false, true) => (EntityType::Date, 0.35),
        };
        out.push(Span::rule(text, kind, m.start(), m.end(), score));
    }
}

fn num(s: &str) -> u32 {
    s.parse().unwrap_or(0)
}

fn month(name: &str) -> Option<u32> {
    let name = name.to_lowercase();
    MONTHS.iter().find(|(m, _)| *m == name).map(|(_, n)| *n)
}

/// Two-digit years 00-29 are read as 20xx, the rest as 19xx.
fn expand_year(s: &str) -> u32 {
    let y = num(s);
    match s.len() {
        2 if y < 30 => 2000 + y,
        2 => 1900 + y,
        _ => y,
    }
}

pub(crate) fn valid_date(year: u32, month: u32, day: u32) -> bool {
    let leap = year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400));
    let days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => return false,
    };
    (1900..=2099).contains(&year) && (1..=days).contains(&day)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calendar() {
        assert!(valid_date(2000, 2, 29));
        assert!(!valid_date(1900, 2, 29));
        assert!(!valid_date(1985, 4, 31));
        assert!(!valid_date(1899, 1, 1));
        assert_eq!(expand_year("85"), 1985);
        assert_eq!(expand_year("07"), 2007);
    }
}
