mod card;
mod date;
mod email;
mod iban;
mod ip;
mod national_id;
mod phone;
mod url;

pub use card::CardDetector;
pub use date::DateDetector;
pub use email::EmailDetector;
pub use iban::IbanDetector;
pub use ip::IpDetector;
pub use national_id::NationalIdDetector;
pub use phone::PhoneDetector;
pub use url::UrlDetector;

use regex::{Match, Regex};

use crate::{Span, Token};

pub trait Detector: Send + Sync {
    fn detect(&self, text: &str, tokens: &[Token]) -> Vec<Span>;
}

/// Runs `re` over `text` and lets `accept` turn each match into a span. A rejected match restarts
/// the search one character later, so a valid entity hidden inside a longer false candidate is
/// still found.
pub(crate) fn scan(re: &Regex, text: &str, mut accept: impl FnMut(Match) -> Option<Span>) -> Vec<Span> {
    let mut out = Vec::new();
    let mut pos = 0;
    while let Some(m) = re.find_at(text, pos) {
        match accept(m) {
            Some(span) => {
                pos = span.end.max(m.start() + 1);
                out.push(span);
            }
            None => pos = m.start() + text[m.start()..].chars().next().map_or(1, char::len_utf8),
        }
        while !text.is_char_boundary(pos) {
            pos += 1;
        }
        if pos >= text.len() {
            break;
        }
    }
    out
}

pub(crate) fn prev_char(text: &str, i: usize) -> Option<char> {
    text[..i].chars().next_back()
}

pub(crate) fn next_char(text: &str, i: usize) -> Option<char> {
    text[i..].chars().next()
}

pub(crate) fn word_bounded(text: &str, start: usize, end: usize) -> bool {
    !prev_char(text, start).is_some_and(char::is_alphanumeric)
        && !next_char(text, end).is_some_and(char::is_alphanumeric)
}

/// Like `word_bounded`, but also rejects numbers glued to other numbers by `.,-/`, as in
/// `1.234.567` or `2023-000123`.
pub(crate) fn number_bounded(text: &str, start: usize, end: usize) -> bool {
    const GLUE: &[char] = &['.', ',', '-', '/'];
    let glued_before = prev_char(text, start)
        .filter(|c| GLUE.contains(c))
        .is_some_and(|c| prev_char(text, start - c.len_utf8()).is_some_and(|d| d.is_ascii_digit()));
    let glued_after = next_char(text, end)
        .filter(|c| GLUE.contains(c))
        .is_some_and(|c| next_char(text, end + c.len_utf8()).is_some_and(|d| d.is_ascii_digit()));
    word_bounded(text, start, end) && !glued_before && !glued_after
}

pub(crate) fn window_before(text: &str, start: usize, bytes: usize) -> String {
    let mut from = start.saturating_sub(bytes);
    while !text.is_char_boundary(from) {
        from += 1;
    }
    text[from..start].to_lowercase()
}

pub(crate) fn window_after(text: &str, end: usize, bytes: usize) -> String {
    let mut to = (end + bytes).min(text.len());
    while !text.is_char_boundary(to) {
        to -= 1;
    }
    text[end..to].to_lowercase()
}

/// Keywords ending in `*` match as word prefixes, all others as whole words.
pub(crate) fn has_keyword(window: &str, keywords: &[&str]) -> bool {
    keywords.iter().any(|k| {
        let (k, prefix) = k.strip_suffix('*').map_or((*k, false), |k| (k, true));
        window.match_indices(k).any(|(i, _)| {
            let starts_word = !prev_char(window, i).is_some_and(char::is_alphabetic);
            let ends_word = prefix || !next_char(window, i + k.len()).is_some_and(char::is_alphabetic);
            starts_word && ends_word
        })
    })
}

pub(crate) fn luhn_valid(digits: &[u8]) -> bool {
    let sum: u32 = digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &d)| {
            let d = u32::from(d);
            if i % 2 == 1 {
                if d * 2 > 9 { d * 2 - 9 } else { d * 2 }
            } else {
                d
            }
        })
        .sum();
    !digits.is_empty() && sum.is_multiple_of(10)
}

pub(crate) fn digits(s: &str) -> Vec<u8> {
    s.bytes().filter(u8::is_ascii_digit).map(|b| b - b'0').collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn luhn() {
        assert!(luhn_valid(&digits("4111 1111 1111 1111")));
        assert!(luhn_valid(&digits("19850312123 0")));
        assert!(!luhn_valid(&digits("4111 1111 1111 1112")));
        assert!(!luhn_valid(&[]));
    }

    #[test]
    fn keywords() {
        assert!(has_keyword("tel.: ", &["tel"]));
        assert!(has_keyword("telefonnummer", &["telefon*"]));
        assert!(!has_keyword("hotel ", &["tel"]));
        assert!(!has_keyword("nécessaire", &["né"]));
        assert!(has_keyword("né(e) le", &["né"]));
    }

    #[test]
    fn number_bounds() {
        let t = "1.234.567 and 12";
        assert!(!number_bounded(t, 2, 5));
        assert!(number_bounded(t, 14, 16));
    }
}
