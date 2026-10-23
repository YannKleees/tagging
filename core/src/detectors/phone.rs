use regex::Regex;

use super::{Detector, has_keyword, number_bounded, scan, window_after, window_before};
use crate::{EntityType, Result, Span, Token, resolve};

const PHONE_WORDS: &[&str] = &[
    "tel*",
    "tél*",
    "tël*",
    "phone*",
    "téléphone*",
    "gsm",
    "handy",
    "mobil*",
    "portable",
    "fax*",
    "anruf*",
    "appel*",
    "call*",
    "ruff*",
    "uruff*",
    "natel",
    "whatsapp",
    "sms",
];
const NEGATIVE_WORDS: &[&str] = &[
    "order*",
    "commande*",
    "bestell*",
    "rechnung*",
    "invoice*",
    "factur*",
    "nr",
    "no",
    "n°",
    "nº",
    "num*",
    "ref*",
    "réf*",
    "kunden*",
    "client*",
    "customer*",
    "account*",
    "konto*",
    "compte*",
    "contrat*",
    "vertrag*",
    "contract*",
    "iban",
    "bic",
    "matricule*",
    "total",
    "montant",
    "betrag",
    "amount",
    "prix",
    "preis*",
    "price*",
    "isbn",
    "serial*",
    "série",
    "serie*",
    "dossier*",
    "ticket*",
    "tracking*",
    "colis",
    "parcel*",
    "id",
];
const CURRENCIES: &[&str] = &["€", "eur", "euro*", "usd", "chf", "$", "£", "gbp"];
/// Country codes accepted after a `00` prefix, where the prefix alone is weak evidence.
const COMMON_CC: &[&str] = &[
    "352", "33", "49", "32", "31", "41", "43", "39", "34", "44", "45", "46", "47", "48", "351", "353", "420", "30",
    "36", "40", "358", "370", "371", "372", "385", "386", "421", "1",
];

pub struct PhoneDetector {
    intl: Regex,
    national: Regex,
    lu_mobile: Regex,
    lu_fixed: Regex,
}

enum Context {
    Phone,
    Neutral,
    Rejected,
}

impl PhoneDetector {
    pub fn new() -> Result<Self> {
        let sep = r"[\d \u{a0}.\-/()]";
        Ok(Self {
            intl: Regex::new(&format!(r"(?:\+|\b00)\d{sep}{{6,24}}\d"))?,
            national: Regex::new(&format!(r"\(?0[1-9]{sep}{{5,20}}\d"))?,
            lu_mobile: Regex::new(r"\b6[1-9]1[ \u{a0}.]?\d{3}[ \u{a0}.]?\d{3}")?,
            lu_fixed: Regex::new(r"\b[2-9]\d(?:[ \u{a0}]?\d{2,4}){1,3}(?:-\d{1,3})?")?,
        })
    }

    fn intl(&self, text: &str) -> Vec<Span> {
        scan(&self.intl, text, |m| {
            let via_00 = m.as_str().starts_with("00");
            let valid = |d: &str| intl_valid(if via_00 { &d[2..] } else { d }, via_00);
            let end = best_prefix(text, m.start(), m.end(), valid, None)?;
            let score = match context(text, m.start(), end) {
                Context::Phone => 0.95,
                _ => 0.9,
            };
            Some(Span::rule(text, EntityType::Phone, m.start(), end, score))
        })
    }

    fn national(&self, text: &str) -> Vec<Span> {
        scan(&self.national, text, |m| {
            let valid = |d: &str| !d.starts_with("00") && (9..=12).contains(&d.len());
            let end = best_prefix(text, m.start(), m.end(), valid, Some(10))?;
            let separated = text[m.start()..end].contains(|c: char| !c.is_ascii_digit());
            let score = match context(text, m.start(), end) {
                Context::Rejected => return None,
                Context::Phone => 0.95,
                Context::Neutral if separated => 0.85,
                Context::Neutral => 0.7,
            };
            Some(Span::rule(text, EntityType::Phone, m.start(), end, score))
        })
    }

    fn lu_mobile(&self, text: &str) -> Vec<Span> {
        scan(&self.lu_mobile, text, |m| {
            if !number_bounded(text, m.start(), m.end()) {
                return None;
            }
            let score = match context(text, m.start(), m.end()) {
                Context::Rejected => return None,
                Context::Phone => 0.95,
                Context::Neutral => 0.85,
            };
            Some(Span::rule(text, EntityType::Phone, m.start(), m.end(), score))
        })
    }

    /// Luxembourg landlines have no trunk prefix and look like any other number, so they need a
    /// phone keyword nearby.
    fn lu_fixed(&self, text: &str) -> Vec<Span> {
        scan(&self.lu_fixed, text, |m| {
            let (base, ext) = m.as_str().split_once('-').unwrap_or((m.as_str(), ""));
            let n = base.bytes().filter(u8::is_ascii_digit).count();
            let valid = if ext.is_empty() {
                (6..=8).contains(&n)
            } else {
                (4..=8).contains(&n)
            };
            let keyword = matches!(context(text, m.start(), m.end()), Context::Phone);
            (valid && keyword && number_bounded(text, m.start(), m.end()))
                .then(|| Span::rule(text, EntityType::Phone, m.start(), m.end(), 0.9))
        })
    }
}

impl Detector for PhoneDetector {
    fn detect(&self, text: &str, _: &[Token]) -> Vec<Span> {
        let mut spans = self.intl(text);
        spans.extend(self.national(text));
        spans.extend(self.lu_mobile(text));
        spans.extend(self.lu_fixed(text));
        resolve(spans)
    }
}

fn context(text: &str, start: usize, end: usize) -> Context {
    let before = window_before(text, start, 30);
    let near = window_before(text, start, 20);
    if has_keyword(&before, PHONE_WORDS) {
        Context::Phone
    } else if has_keyword(&near, NEGATIVE_WORDS)
        || has_keyword(&window_before(text, start, 6), CURRENCIES)
        || has_keyword(&window_after(text, end, 6), CURRENCIES)
    {
        Context::Rejected
    } else {
        Context::Neutral
    }
}

fn intl_valid(d: &str, via_00: bool) -> bool {
    let known = [
        ("352", 6..=11),
        ("33", 9..=9),
        ("49", 6..=13),
        ("32", 8..=9),
        ("31", 9..=9),
    ];
    for (cc, range) in known {
        if let Some(n) = d.strip_prefix(cc) {
            return if cc == "352" && n.starts_with('6') {
                n.len() == 9
            } else {
                range.contains(&n.len())
            };
        }
    }
    (8..=15).contains(&d.len()) && (!via_00 || COMMON_CC.iter().any(|cc| d.starts_with(cc)))
}

/// Splits the candidate into digit groups and returns the end of the longest group prefix whose
/// digits pass `valid` and that ends on a number boundary. A `(0)` trunk group is not counted.
/// `prefer` picks a prefix with exactly that many digits when one exists.
fn best_prefix(
    text: &str,
    start: usize,
    end: usize,
    valid: impl Fn(&str) -> bool,
    prefer: Option<usize>,
) -> Option<usize> {
    let mut runs: Vec<(usize, usize)> = Vec::new();
    let mut run_start = None;
    for (i, c) in text[start..end].char_indices().map(|(i, c)| (start + i, c)) {
        match (c.is_ascii_digit(), run_start) {
            (true, None) => {
                if runs.last().is_some_and(|&(_, e)| i - e > 2) {
                    break;
                }
                run_start = Some(i);
            }
            (false, Some(s)) => {
                runs.push((s, i));
                run_start = None;
            }
            _ => {}
        }
    }
    if let Some(s) = run_start {
        runs.push((s, end));
    }

    let mut digits = String::new();
    let mut candidates = Vec::new();
    for (k, &(s, e)) in runs.iter().enumerate() {
        let trunk = k > 0 && &text[s..e] == "0" && text[..s].ends_with('(') && text[e..].starts_with(')');
        if !trunk {
            digits.push_str(&text[s..e]);
        }
        if valid(&digits) && number_bounded(text, start, e) {
            candidates.push((digits.len(), e));
        }
    }
    prefer
        .and_then(|n| candidates.iter().find(|(len, _)| *len == n))
        .or(candidates.last())
        .map(|&(_, e)| e)
}
