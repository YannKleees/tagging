use regex::Regex;

use super::{Detector, has_keyword, number_bounded, scan, word_bounded};
use crate::{EntityType, Result, Span, Token, resolve};

/// Street words that come before the name: `rue de la Gare`, `boulevard Royal`.
const LEADING: &str = "rue|route|rte|avenue|av\\.|boulevard|bd\\.?|blvd\\.?|place|pl\\.|allée|allee|chemin|impasse|\
                       montée|montee|cité|cite|quai|square|passage|sentier|val|côte|cote|rond-point|zone";
/// Leading street words that name a street even without a house number.
const STREET_ONLY: &[&str] = &["rue", "avenue", "boulevard", "impasse", "allée"];
/// Street words written after the name or glued to it: `Hauptstraße`, `Escher Strooss`.
const TRAILING: &str = "straße|strasse|strooss|stross|str\\.|gasse|gaass|allee|weg|wee|platz|plaz";
const TRAILING_EN: &str = "Street|Road|Avenue|Lane|Drive|Close|Way|Square|Court|Place|Terrace|Crescent|Boulevard|\
                           Gardens|Grove|Row|Walk";
/// Compounds that end like a street but are not one: `Parkplatz 12`, `Umweg 3`.
#[rustfmt::skip]
const NOT_STREETS: &[&str] = &[
    "parkplatz*", "stellplatz*", "sitzplatz*", "arbeitsplatz*", "spielplatz*", "sportplatz*", "marktplatz*",
    "startplatz*", "umweg*", "ausweg*", "heimweg*", "rückweg*", "hinweg*", "schulweg*", "radweg*", "fußweg*",
    "feldweg*", "waldweg*", "gehweg*",
];
/// Capitalized sentence openers that are not part of a street name: `Die Hauptstraße 12`.
#[rustfmt::skip]
const OPENERS: &[&str] = &[
    "der", "die", "das", "den", "dem", "in", "an", "auf", "bei", "zur", "zum", "von", "aus", "nach", "de", "op",
    "the", "at", "on", "to", "from", "la", "le", "au", "à",
];

/// Bytes searched before and after a street word. The full patterns are slow, so they only run
/// on these windows.
const WINDOW: (usize, usize) = (80, 120);

#[rustfmt::skip]
const US_STATES: &[&str] = &[
    "AL", "AK", "AZ", "AR", "CA", "CO", "CT", "DE", "FL", "GA", "HI", "ID", "IL", "IN", "IA", "KS", "KY", "LA", "ME",
    "MD", "MA", "MI", "MN", "MS", "MO", "MT", "NE", "NV", "NH", "NJ", "NM", "NY", "NC", "ND", "OH", "OK", "OR", "PA",
    "RI", "SC", "SD", "TN", "TX", "UT", "VT", "VA", "WA", "WV", "WI", "WY", "DC",
];

pub struct AddressDetector {
    anchor: Regex,
    leading: Regex,
    trailing: Regex,
    english: Regex,
    postcode: Regex,
    us_zip: Regex,
    uk_postcode: Regex,
    coordinates: Regex,
    unit: Regex,
    rest_of_line: Regex,
}

impl AddressDetector {
    pub fn new() -> Result<Self> {
        // A name word starts with a capital, a digit-free initial (`J.F.`) or is a saint's day.
        let word = r"(?:(?:\p{Lu}\.){1,3}|\p{Lu}[\p{L}'’]+(?:-\p{L}[\p{L}'’]*)*)";
        let link = r"(?:(?i:de la|de l['’]|du|des|de|d['’]|l['’]|la|le|les|et|vum|vun|van|von|der|den) ?)";
        let name = format!(r"{link}{{0,3}}{word}(?:[ \u{{a0}}]{link}{{0,3}}{word}){{0,3}}");
        let number =
            r"\d{1,4}(?:[ \u{a0}]?[-–/][ \u{a0}]?\d{1,4})?(?:[ \u{a0}]?[A-Za-z]\b)?(?:[ \u{a0}](?:bis|ter)\b)?";
        Ok(Self {
            anchor: Regex::new(&format!(
                r"(?i)\b(?:{LEADING})[ \u{{a0}}]|{TRAILING}|\b(?:{TRAILING_EN})\b"
            ))?,
            leading: Regex::new(&format!(
                r"(?:(?P<before>{number})[ \u{{a0}}]?,?[ \u{{a0}}]+)?(?P<kind>(?i:{LEADING}))[ \u{{a0}}]+{name}(?:,?[ \u{{a0}}]+(?P<after>{number}))?"
            ))?,
            trailing: Regex::new(&format!(
                r"(?:(?P<before>{number})[ \u{{a0}}]?,?[ \u{{a0}}]+)?(?:\p{{Lu}}[\p{{L}}'’-]+[ \u{{a0}}]){{0,2}}\p{{Lu}}[\p{{L}}'’-]*[ \u{{a0}}-]?(?i:{TRAILING})(?:[ \u{{a0}}]+(?P<after>{number}))?"
            ))?,
            english: Regex::new(&format!(
                r"(?P<before>\d{{1,5}}[A-Za-z]?),?[ \u{{a0}}]+(?:\p{{Lu}}[\p{{L}}'’.-]+[ \u{{a0}}]+){{1,3}}(?:{TRAILING_EN})\b"
            ))?,
            // The state follows the town after a comma: `Falmouth, KY, 41040-7980`.
            us_zip: Regex::new(r",[ \u{a0}]([A-Z]{2}),?[ \u{a0}](\d{5}(?:-\d{4})?)")?,
            uk_postcode: Regex::new(r"\b[A-Z]{1,2}\d[A-Z\d]?[ \u{a0}]\d[A-Z]{2}")?,
            // `Apt 4B`, `Suite 288`, `Wohnung 12`. Rooms and floors are in offices too and stay.
            unit: Regex::new(
                r"(?i)\b(?:apartment|apt|suite|flat|maisonette|duplex|penthouse|appartement|app|wohnung|whg)\.?[ \u{a0}](?:n°[ \u{a0}]?)?\d{1,4}[A-Za-z]?\b",
            )?,
            // What follows the street on one line: `, Kettering, ENG, NN16 8QP` or `, Trier, 54290`.
            rest_of_line: Regex::new(
                r"^,[ \u{a0}](\p{Lu}[\p{L}'’.\- ]{1,40}?)(?:,[ \u{a0}][\p{L}\- ]{2,25})?,[ \u{a0}](\d{4,5}(?:-\d{4})?|[A-Z]{1,2}\d[A-Z\d]?(?:[ \u{a0}]\d[A-Z]{2})?)\b",
            )?,
            // Latitude and longitude in brackets: `[49.6117, 6.1319]`.
            coordinates: Regex::new(r"[\[(](-?\d{1,2}\.\d{1,7}),[ \u{a0}]?(-?\d{1,3}\.\d{1,7})[\])]")?,
            // Luxembourg writes `L-1234`; letters from neighbouring countries need five digits.
            postcode: Regex::new(r"\b(?:L[-–]\d{4}|[DF][-–]\d{5})\b")?,
        })
    }

    /// Stretches of text around street words, merged where they overlap and cut at whitespace so
    /// that no number or word is seen in part.
    fn windows(&self, text: &str) -> Vec<(usize, usize)> {
        let mut out: Vec<(usize, usize)> = Vec::new();
        for m in self.anchor.find_iter(text) {
            // Byte arithmetic can land inside a multi-byte character such as `é`.
            let mut start = m.start().saturating_sub(WINDOW.0);
            while start > 0 && !(text.is_char_boundary(start) && text[..start].ends_with(char::is_whitespace)) {
                start -= 1;
            }
            let mut end = (m.end() + WINDOW.1).min(text.len());
            while end < text.len() && !(text.is_char_boundary(end) && text[end..].starts_with(char::is_whitespace)) {
                end += 1;
            }
            match out.last_mut() {
                Some(last) if start <= last.1 => last.1 = end,
                _ => out.push((start, end)),
            }
        }
        out
    }

    fn streets(&self, text: &str, (from, to): (usize, usize), re: &Regex, out: &mut Vec<Span>) {
        for c in re.captures_iter(&text[from..to]) {
            let Some(m) = c.get(0) else { continue };
            let group = |name: &str| c.name(name).map(|g| (from + g.start(), from + g.end()));
            // `26 12 34 56, rue des Bains`: the last group of a phone number is not a house number.
            let after_digits = |at: usize| {
                text[..at]
                    .trim_end_matches([' ', '\u{a0}'])
                    .ends_with(|c: char| c.is_ascii_digit())
            };
            let before = group("before").filter(|&(a, b)| number_bounded(text, a, b) && !after_digits(a));
            let after = group("after").filter(|&(a, b)| number_bounded(text, a, b));
            // A trailing number that is really the next phone number or year is left out.
            let end = match (after, group("after")) {
                (None, Some((a, _))) => text[..a].trim_end_matches([' ', '\u{a0}', ',']).len(),
                _ => from + m.end(),
            };
            let start = match (before, group("before")) {
                (None, Some((_, b))) => b + text[b..].len() - text[b..].trim_start_matches([' ', '\u{a0}', ',']).len(),
                _ => from + m.start(),
            };
            let start = start + opener_len(&text[start..end]);
            if start >= end || !word_bounded(text, start, end) {
                continue;
            }
            let street = text[start..end].to_lowercase();
            let numbered = before.is_some() || after.is_some();
            let score = match c.name("kind") {
                Some(_) if numbered => 0.9,
                Some(k) if STREET_ONLY.contains(&k.as_str().to_lowercase().as_str()) => 0.6,
                Some(_) => continue,
                None if !numbered || bare(&street) || has_keyword(&street, NOT_STREETS) => continue,
                None => 0.9,
            };
            out.push(Span::rule(text, EntityType::Address, start, end, score));
        }
    }
}

/// Bytes taken by capitalized openers in front of a `Hauptstraße`-style name.
fn opener_len(street: &str) -> usize {
    let mut n = 0;
    while let Some((word, rest)) = street[n..].split_once([' ', '\u{a0}']) {
        if !OPENERS.contains(&word.to_lowercase().as_str()) || rest.is_empty() {
            break;
        }
        n += word.len() + 1;
    }
    n
}

/// `Straße 12` or `Platz 3` without a name is not an address.
fn bare(street: &str) -> bool {
    let name = street.trim_matches(|c: char| !c.is_alphabetic());
    let words: Vec<&str> = name
        .split([' ', '\u{a0}'])
        .filter(|w| w.chars().any(char::is_alphabetic))
        .collect();
    words.len() == 1
        && TRAILING
            .split('|')
            .any(|t| t.trim_end_matches("\\.") == words[0].trim_end_matches('.'))
}

impl Detector for AddressDetector {
    fn detect(&self, text: &str, _: &[Token]) -> Vec<Span> {
        let mut spans = Vec::new();
        for window in self.windows(text) {
            self.streets(text, window, &self.leading, &mut spans);
            self.streets(text, window, &self.trailing, &mut spans);
            self.streets(text, window, &self.english, &mut spans);
        }
        spans.extend(scan(&self.postcode, text, |m| {
            number_bounded(text, m.start(), m.end())
                .then(|| Span::rule(text, EntityType::Address, m.start(), m.end(), 0.85))
        }));
        // A street with a house number opens a postal address; the town and the postcode follow.
        for street in spans.clone().iter().filter(|s| s.score >= 0.9) {
            let Some(c) = self.rest_of_line.captures(&text[street.end..]) else {
                continue;
            };
            let (Some(town), Some(code)) = (c.get(1), c.get(2)) else {
                continue;
            };
            let (a, b) = (street.end + town.start(), street.end + town.end());
            let (x, y) = (street.end + code.start(), street.end + code.end());
            spans.push(Span::rule(text, EntityType::Location, a, b, 0.75));
            spans.push(Span::rule(text, EntityType::Address, x, y, 0.7));
        }
        spans.extend(scan(&self.unit, text, |m| {
            word_bounded(text, m.start(), m.end())
                .then(|| Span::rule(text, EntityType::Address, m.start(), m.end(), 0.6))
        }));
        // `KY, 41040-7980`: only the ZIP code is reported, the state is no address.
        for c in self.us_zip.captures_iter(text) {
            let (Some(state), Some(zip)) = (c.get(1), c.get(2)) else {
                continue;
            };
            if US_STATES.contains(&state.as_str()) && number_bounded(text, zip.start(), zip.end()) {
                spans.push(Span::rule(text, EntityType::Address, zip.start(), zip.end(), 0.7));
            }
        }
        spans.extend(scan(&self.uk_postcode, text, |m| {
            word_bounded(text, m.start(), m.end())
                .then(|| Span::rule(text, EntityType::Address, m.start(), m.end(), 0.75))
        }));
        for c in self.coordinates.captures_iter(text) {
            let (Some(m), Some(lat), Some(lon)) = (c.get(0), c.get(1), c.get(2)) else {
                continue;
            };
            let within = |s: &str, max: f64| s.parse::<f64>().is_ok_and(|x| x.abs() <= max);
            if within(lat.as_str(), 90.0) && within(lon.as_str(), 180.0) {
                spans.push(Span::rule(text, EntityType::Location, m.start(), m.end(), 0.8));
            }
        }
        resolve(spans)
    }
}
