use regex::{Captures, Regex};

use super::{Detector, has_keyword, next_char, number_bounded, prev_char, window_after, window_before, word_bounded};
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

/// Score of a date that is no birth date, and of a time of day, when the config asks for them.
const REPORTED: f64 = 0.6;

pub struct DateDetector {
    numeric: Regex,
    iso: Regex,
    day_month: Regex,
    month_day: Regex,
    month_year: Regex,
    time: Regex,
    all: bool,
}

impl DateDetector {
    /// With `all`, dates that are no birth dates and times of day are reported too.
    pub fn new(all: bool) -> Result<Self> {
        let mut names: Vec<&str> = MONTHS.iter().map(|(m, _)| *m).collect();
        names.sort_by_key(|m| std::cmp::Reverse(m.chars().count()));
        let months = names.iter().map(|m| regex::escape(m)).collect::<Vec<_>>().join("|");
        Ok(Self {
            numeric: Regex::new(r"(\d{1,2})([./-])(\d{1,2})([./-])(\d{4}|\d{2})")?,
            // The time of an ISO timestamp belongs to the date: `1960-05-20T00:00:00`.
            iso: Regex::new(r"(\d{4})-(\d{2})-(\d{2})(?:T\d{2}:\d{2}(?::\d{2})?)?")?,
            day_month: Regex::new(&format!(
                r"(?i)(\d{{1,2}})(?:\.|er|e|ème|st|nd|rd|th)?\s+(?:of\s+)?({months})\.?,?\s+(\d{{4}})"
            ))?,
            month_day: Regex::new(&format!(
                r"(?i)({months})\.?\s+(\d{{1,2}})(?:st|nd|rd|th|er|e|\.)?,?\s+(\d{{4}})"
            ))?,
            // A month and a year: `June 1942`, `juin/55`.
            month_year: Regex::new(&format!(r"(?i)({months})(?:[ \u{{a0}}](\d{{4}})|/(\d{{4}}|\d{{2}}))"))?,
            // `14:30`, `8:20 PM`, `02:40:19`, `14h30`, `18 Uhr`.
            time: Regex::new(
                r"(?i)\b(?:[01]?\d|2[0-3])(?::[0-5]\d(?::[0-5]\d)?(?:[ \u{a0}]?[ap]\.?m\b\.?)?|h[0-5]\d\b|[ \u{a0}]?(?:[ap]\.?m\b\.?|uhr\b|auer\b|heures\b|o'clock\b))",
            )?,
            all,
        })
    }
}

impl Detector for DateDetector {
    fn detect(&self, text: &str, _: &[Token]) -> Vec<Span> {
        let mut spans = Vec::new();
        let generic = if self.all { REPORTED } else { 0.4 };
        let find = |re: &Regex, bounded: Bounded, out: &mut Vec<Span>, parse: &dyn Fn(&Captures) -> Option<Parsed>| {
            find(text, re, bounded, generic, out, parse)
        };
        find(&self.numeric, number_bounded, &mut spans, &|c| {
            (c[2] == c[4]).then(|| (expand_year(&c[5]), num(&c[3]), num(&c[1]), c[5].len() == 2))
        });
        find(&self.iso, number_bounded, &mut spans, &|c| {
            Some((num(&c[1]), num(&c[2]), num(&c[3]), false))
        });
        find(&self.day_month, word_bounded, &mut spans, &|c| {
            Some((num(&c[3]), month(&c[2])?, num(&c[1]), false))
        });
        find(&self.month_day, word_bounded, &mut spans, &|c| {
            Some((num(&c[3]), month(&c[1])?, num(&c[2]), false))
        });
        find(&self.month_year, word_bounded, &mut spans, &|c| {
            // After a day number this is a full date, and an impossible one if it got here.
            let before = text[..c.get(0)?.start()].trim_end_matches([' ', '\u{a0}', '.']);
            if before.ends_with(|ch: char| ch.is_ascii_digit()) {
                return None;
            }
            let year = c.get(2).or(c.get(3))?.as_str();
            Some((expand_year(year), month(&c[1])?, 1, year.len() == 2))
        });
        if self.all {
            for m in self.time.find_iter(text) {
                // `12:30:45` inside an IPv6 address or a ratio such as `3:1` is no time.
                let glued = |c: Option<char>| c.is_some_and(|c| c.is_alphanumeric() || c == ':');
                if !glued(prev_char(text, m.start())) && !glued(next_char(text, m.end())) {
                    spans.push(Span::rule(text, EntityType::Date, m.start(), m.end(), REPORTED));
                }
            }
        }
        resolve(spans)
    }
}

type Bounded = fn(&str, usize, usize) -> bool;
/// Year, month, day and whether the year was written with two digits.
type Parsed = (u32, u32, u32, bool);

fn find(
    text: &str,
    re: &Regex,
    bounded: Bounded,
    generic: f64,
    out: &mut Vec<Span>,
    parse: &dyn Fn(&Captures) -> Option<Parsed>,
) {
    for c in re.captures_iter(text) {
        let (Some(m), Some((year, month, day, short_year))) = (c.get(0), parse(&c)) else {
            continue;
        };
        if !bounded(text, m.start(), m.end()) || !valid_date(year, month, day) {
            continue;
        }
        // German and Luxembourgish put the verb last: "am 03.02.1961 in Luxemburg geboren".
        let before = window_before(text, m.start(), 40);
        let birth = has_keyword(&before, BIRTH_WORDS)
            || has_keyword(&window_after(text, m.end(), 30), BIRTH_WORDS)
            || abbreviated_birth(&text[..m.start()]);
        let (kind, score) = match (birth, short_year) {
            (true, false) => (EntityType::DateOfBirth, 0.95),
            (true, true) => (EntityType::DateOfBirth, 0.9),
            (false, false) => (EntityType::Date, generic),
            (false, true) => (EntityType::Date, generic - 0.05),
        };
        out.push(Span::rule(text, kind, m.start(), m.end(), score));
    }
}

/// A lowercase `b.` (`b. 08/02/1982`) or the genealogical `*`, directly in front of the date.
fn abbreviated_birth(before: &str) -> bool {
    let before = before.trim_end();
    let born = before
        .strip_suffix("b.")
        .is_some_and(|rest| !rest.ends_with(char::is_alphanumeric));
    born || before.ends_with('*')
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
