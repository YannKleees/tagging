use regex::Regex;

use super::{Detector, digits, has_keyword, next_char, number_bounded, prev_char, scan, window_before};
use crate::{EntityType, Result, Span, Token, resolve};

/// Words that say the number after them identifies a person.
#[rustfmt::skip]
const ID_WORDS: &[&str] = &[
    "passport*", "reisepass*", "pass-nr", "passeport*", "ausweis*", "personalausweis*", "identit*", "id-karte",
    "licen*", "führerschein*", "permis", "social", "sozialversicherung*", "sv-nummer", "sécurité", "ssn", "nir",
    "steuer*", "idnr", "tin", "tax", "fiscal*", "rijksregister*", "registre", "matricule*", "matrikel*",
    "versicherungsnummer", "rentenversicherung*", "insurance",
];
/// Words that say a code names a thing, not a person.
#[rustfmt::skip]
const NOT_ID_WORDS: &[&str] = &[
    "order*", "commande*", "bestell*", "rechnung*", "invoice*", "factur*", "ref", "reference", "réf", "référence",
    "referenz", "serial*", "série", "serien*", "model", "modell", "modèle", "version*", "versioun", "sku", "isbn",
    "tracking*", "colis", "parcel*", "ticket*", "artikelnummer", "flight", "flugnummer", "booking*", "buchung*",
    "réservation", "aktenzeichen",
];

/// National and tax numbers of Luxembourg's neighbours, checked by their check digits, the shape of
/// a US social security number, and opaque codes that mix letters and digits the way passport and
/// licence numbers do. The Luxembourg matricule has its own detector.
pub struct IdNumberDetector {
    french: Regex,
    belgian: Regex,
    german_pension: Regex,
    german_tax: Regex,
    us_social: Regex,
    code: Regex,
    swiss: Regex,
}

impl IdNumberDetector {
    pub fn new() -> Result<Self> {
        let s = r"[ .\u{a0}]?";
        Ok(Self {
            // Sex, year, month, department, commune, serial, key: `2 55 08 14 168 025 38`.
            french: Regex::new(&format!(
                r"\b[1278]{s}\d{{2}}{s}\d{{2}}{s}(?:\d{{2}}|2[ABab]){s}\d{{3}}{s}\d{{3}}{s}\d{{2}}"
            ))?,
            // `85.03.12-123.62`
            belgian: Regex::new(r"\b\d{2}[.\- ]?\d{2}[.\- ]?\d{2}[.\- ]?\d{3}[.\- ]?\d{2}")?,
            // Area, birth date, initial, serial and check digit: `65 170839 J 00 3`.
            german_pension: Regex::new(r"\b\d{2}[ \u{a0}]?\d{6}[ \u{a0}]?[A-Z][ \u{a0}]?\d{2}[ \u{a0}]?\d")?,
            german_tax: Regex::new(r"\b[1-9]\d(?:[ \u{a0}]?\d{3}){3}")?,
            us_social: Regex::new(r"\b\d{3}-\d{2}-\d{4}")?,
            code: Regex::new(r"\b[A-Z0-9][A-Z0-9\-.]{5,38}[A-Z0-9]")?,
            // Swiss AHV number: `756.1234.5678.97`.
            swiss: Regex::new(r"\b756\.\d{4}\.\d{4}\.\d{2}")?,
        })
    }

    fn checked(&self, text: &str, re: &Regex, valid: fn(&str) -> bool, score: f64) -> Vec<Span> {
        scan(re, text, |m| {
            (number_bounded(text, m.start(), m.end()) && valid(m.as_str()))
                .then(|| Span::rule(text, EntityType::IdNumber, m.start(), m.end(), score))
        })
    }

    /// Format alone is weak evidence; an ID word in front makes it strong.
    fn by_context(&self, text: &str, re: &Regex, valid: fn(&str) -> bool, plain: f64) -> Vec<Span> {
        scan(re, text, |m| {
            if !number_bounded(text, m.start(), m.end()) || !valid(m.as_str()) {
                return None;
            }
            let score = if has_keyword(&window_before(text, m.start(), 40), ID_WORDS) {
                0.9
            } else {
                plain
            };
            Some(Span::rule(text, EntityType::IdNumber, m.start(), m.end(), score))
        })
    }

    fn codes(&self, text: &str) -> Vec<Span> {
        // Part of an email, a URL, a file name or an IPv6 address.
        let glued = |c: Option<char>| c.is_some_and(|c| matches!(c, '@' | '/' | '.' | '_' | '\\' | '#'));
        scan(&self.code, text, |m| {
            let after = next_char(text, m.end());
            let inside = glued(prev_char(text, m.start())) || after.is_some_and(|c| matches!(c, '@' | '/' | '_'));
            if inside || !number_bounded(text, m.start(), m.end()) || !code_shaped(m.as_str()) {
                return None;
            }
            let before = window_before(text, m.start(), 40);
            let same_line = window_before(text, m.start(), 24);
            if has_keyword(same_line.rsplit('\n').next().unwrap_or_default(), NOT_ID_WORDS) {
                return None;
            }
            let score = if has_keyword(&before, ID_WORDS) { 0.85 } else { 0.55 };
            Some(Span::rule(text, EntityType::IdNumber, m.start(), m.end(), score))
        })
    }
}

impl Detector for IdNumberDetector {
    fn detect(&self, text: &str, _: &[Token]) -> Vec<Span> {
        let mut spans = self.checked(text, &self.french, french_valid, 0.95);
        spans.extend(self.checked(text, &self.belgian, belgian_valid, 0.9));
        spans.extend(self.checked(text, &self.german_pension, german_pension_valid, 0.9));
        spans.extend(self.by_context(text, &self.german_tax, german_tax_valid, 0.55));
        spans.extend(self.by_context(text, &self.us_social, us_social_valid, 0.6));
        spans.extend(self.checked(text, &self.swiss, |_| true, 0.85));
        spans.extend(self.codes(text));
        resolve(spans)
    }
}

/// One to three letters and at least seven digits (`KP2212837`, `I74R9803F`), or a longer mix of
/// letters and digits (`C85GR6998227`, `T2-FRA-86MV0710-1`). Years, sizes and product names with a
/// few digits (`ISO9001`, `A320`, `2024-11`) do not qualify.
fn code_shaped(code: &str) -> bool {
    let letters = code.bytes().filter(u8::is_ascii_uppercase).count();
    let numbers = code.bytes().filter(u8::is_ascii_digit).count();
    let longest_letters = code
        .split(|c: char| !c.is_ascii_uppercase())
        .map(str::len)
        .max()
        .unwrap_or(0);
    let compact = letters + numbers;
    let passport_like = (1..=3).contains(&letters) && numbers >= 7 && compact <= 16;
    let mixed = letters >= 2 && numbers >= 4 && compact >= 9;
    // A long run of letters is a word with a number attached.
    let dotted_number = code.split('.').all(|part| part.bytes().all(|b| b.is_ascii_digit()));
    (passport_like || mixed) && longest_letters <= 6 && !code.contains("--") && !code.contains("..") && !dotted_number
}

fn number(d: &[u8]) -> u64 {
    d.iter().fold(0, |acc, &x| acc * 10 + u64::from(x))
}

/// French NIR: the key is 97 minus the first thirteen digits modulo 97. Corsica writes its
/// department as 2A or 2B, which count as 19 and 18.
fn french_valid(s: &str) -> bool {
    let compact: String = s.chars().filter(char::is_ascii_alphanumeric).collect();
    let compact = compact.to_uppercase().replace("2A", "19").replace("2B", "18");
    let d = digits(&compact);
    let month = d.get(3..5).map_or(0, number);
    d.len() == 15 && (1..=12).contains(&month) && 97 - number(&d[..13]) % 97 == number(&d[13..])
}

/// Belgian national number: birth date, serial, and 97 minus the first nine digits modulo 97. For
/// people born from 2000 on, a 2 is put in front before dividing.
fn belgian_valid(s: &str) -> bool {
    let d = digits(s);
    if d.len() != 11 || !(1..=12).contains(&number(&d[2..4])) || !(1..=31).contains(&number(&d[4..6])) {
        return false;
    }
    let (base, key) = (number(&d[..9]), number(&d[9..]));
    97 - base % 97 == key || 97 - (2_000_000_000 + base) % 97 == key
}

/// German pension insurance number. The initial counts as its place in the alphabet; each digit is
/// multiplied by its weight and the digit sums of the products are added up.
fn german_pension_valid(s: &str) -> bool {
    const WEIGHTS: [u32; 12] = [2, 1, 2, 5, 7, 1, 2, 1, 2, 1, 2, 1];
    let compact: Vec<char> = s.chars().filter(char::is_ascii_alphanumeric).collect();
    let [_, _, d1, d2, m1, m2, _, _, letter, s1, s2, check] = compact[..] else {
        return false;
    };
    if !letter.is_ascii_uppercase() {
        return false;
    }
    let digit = |c: char| c.to_digit(10);
    let (Some(day), Some(month)) = (
        digit(d1).zip(digit(d2)).map(|(a, b)| a * 10 + b),
        digit(m1).zip(digit(m2)).map(|(a, b)| a * 10 + b),
    ) else {
        return false;
    };
    let position = letter as u32 - 'A' as u32 + 1;
    let mut values: Vec<u32> = compact[..8].iter().filter_map(|&c| digit(c)).collect();
    values.extend([position / 10, position % 10]);
    values.extend([s1, s2].iter().filter_map(|&c| digit(c)));
    let sum: u32 = values.iter().zip(WEIGHTS).map(|(v, w)| v * w / 10 + v * w % 10).sum();
    values.len() == 12 && (1..=31).contains(&day) && (1..=12).contains(&month) && Some(sum % 10) == digit(check)
}

/// German tax ID: eleven digits, the last one a check digit, and among the first ten exactly one
/// digit that occurs two or three times.
fn german_tax_valid(s: &str) -> bool {
    let d = digits(s);
    if d.len() != 11 {
        return false;
    }
    let mut counts = [0u8; 10];
    d[..10].iter().for_each(|&x| counts[usize::from(x)] += 1);
    let repeated = counts.iter().filter(|&&c| c > 1).count() == 1 && counts.iter().all(|&c| c <= 3);
    let product = d[..10].iter().fold(10u32, |p, &x| {
        let sum = (u32::from(x) + p) % 10;
        (if sum == 0 { 10 } else { sum }) * 2 % 11
    });
    repeated && (11 - product) % 10 == u32::from(d[10])
}

fn us_social_valid(s: &str) -> bool {
    let area = &s[..3];
    area != "000" && area != "666" && !area.starts_with('9') && &s[4..6] != "00" && &s[7..] != "0000"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_digits() {
        assert!(french_valid("2 55 08 14 168 025 38"));
        assert!(!french_valid("2 55 08 14 168 025 39"));
        assert!(belgian_valid("85.03.12-123.62"));
        assert!(belgian_valid("01020304526"));
        assert!(!belgian_valid("85.03.12-123.63"));
        assert!(german_pension_valid("65 170839 J 00 3"));
        assert!(!german_pension_valid("65 170839 J 00 4"));
        assert!(german_tax_valid("86 095 742 719"));
        assert!(!german_tax_valid("86 095 742 718"));
        assert!(!german_tax_valid("11 111 111 111"));
    }

    #[test]
    fn code_shapes() {
        for code in [
            "KP2212837",
            "I74R9803F",
            "C85GR6998227",
            "T2-FRA-86MV0710-1",
            "QW63852ZQ",
            "M31416527",
        ] {
            assert!(code_shaped(code), "{code}");
        }
        for code in [
            "ISO9001",
            "A320",
            "A300-600",
            "767-300F",
            "2024-11-05",
            "COVID19",
            "HTML5",
            "123456789",
            "LUXEMBOURG",
            "WINDOWS2000",
        ] {
            assert!(!code_shaped(code), "{code}");
        }
    }
}
