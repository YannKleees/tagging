use std::collections::HashMap;

use regex::Regex;

use super::{Detector, prev_char};
use crate::{EntityType, Result, Span, Token, resolve};

#[derive(Clone, Copy, PartialEq)]
enum Field {
    Id,
    /// German `Pass` is a passport, English `pass` a password.
    Pass,
    Secret,
    Username,
    Phone,
    Birth,
    DateTime,
    Person,
    Place,
    Street,
    Building,
    Postcode,
    SecondLine,
}

/// Field names of forms, exports and letter heads in English, French, German and Luxembourgish.
/// A space stands for a space, hyphen, underscore or nothing, so `date of birth` also matches
/// `Date_of_Birth` and `DateOfBirth`.
#[rustfmt::skip]
const KEYS: &[(Field, &[&str])] = &[
    (Field::Id, &[
        "passport", "reisepass", "reisepassnummer", "passeport", "pass nr",
        "id card", "identity card", "national id", "id number", "ausweis", "ausweisnummer", "personalausweis",
        "id karte", "carte d'identité", "carte d’identité", "numéro d'identité", "identitéitskaart",
        "driver license", "driver's license", "drivers license", "driving licence", "driving license",
        "führerschein", "führerscheinnummer", "permis de conduire", "permis", "führerschäin",
        "social security number", "social security", "social number", "ssn", "sozialversicherungsnummer",
        "sozialnummer", "sv nummer", "numéro de sécurité sociale", "sécurité sociale", "matricule", "matrikel",
        "matrikelnummer", "tax id", "steuernummer", "steuer id", "numéro fiscal",
        "patient id", "customer id", "client id", "kundennummer", "numéro client", "student id", "employee id",
        "personalnummer", "member id", "mitgliedsnummer", "donor id", "license plate", "kennzeichen",
        "plaque d'immatriculation",
    ]),
    (Field::Pass, &["pass"]),
    (Field::Secret, &["password", "passwort", "kennwort", "mot de passe", "passcode", "pwd", "passwuert"]),
    (Field::Phone, &[
        "telephone", "phone", "tel", "téléphone", "tél", "telefon", "telefonnummer", "numéro de téléphone", "mobile",
        "handy", "gsm", "fax", "contact number", "telefonsnummer",
    ]),
    (Field::Username, &[
        "username", "user name", "benutzername", "nom d'utilisateur", "nom d’utilisateur", "nom utilisateur", "login", "userid",
        "user id", "identifiant", "pseudo", "benotzernumm",
    ]),
    (Field::Birth, &[
        "date of birth", "birth date", "birthdate", "dob", "geburtsdatum", "geboren am", "date de naissance",
        "gebuertsdatum", "gebuertsdag",
    ]),
    (Field::DateTime, &["date", "datum", "time", "zeit", "uhrzeit", "heure", "timestamp"]),
    (Field::Person, &[
        "first name", "given name", "forename", "middle name", "vorname", "zweiter vorname", "prénom",
        "deuxième prénom", "virnumm", "last name", "surname", "family name", "nachname", "familienname",
        "nom de famille", "familljennumm", "nom", "name", "full name", "numm", "maiden name", "geburtsname",
        "nom de jeune fille",
    ]),
    (Field::Place, &[
        "city", "town", "stadt", "ort", "wohnort", "ville", "uertschaft", "place of birth", "geburtsort",
        "lieu de naissance", "gebuertsplaz", "country", "land", "pays", "state", "province", "bundesland",
        "kanton", "canton", "région", "region", "county", "lieu", "location",
    ]),
    (Field::Street, &["street", "street name", "street address", "strasse", "straße", "rue", "strooss"]),
    (Field::Building, &[
        "building", "gebäude", "bâtiment", "immeuble", "house number", "hausnummer", "numéro de rue",
    ]),
    (Field::Postcode, &[
        "postcode", "postal code", "post code", "zip", "zip code", "plz", "postleitzahl", "code postal",
        "postleitzuel",
    ]),
    (Field::SecondLine, &[
        "secondary address", "second address", "sec address", "address line 2", "zweite adresse",
        "sekundäre adresse", "adresszusatz",
        "adresse secondaire", "deuxième adresse", "complément d'adresse", "apartment", "appartement", "wohnung",
    ]),
];
/// A word in front of `name` that makes it the name of a thing.
#[rustfmt::skip]
const NOT_A_PERSON: &[&str] = &[
    "company", "firma", "firmen", "société", "entreprise", "file", "datei", "fichier", "product", "produkt",
    "produit", "project", "projekt", "projet", "host", "domain", "brand", "marke", "event", "course", "kurs",
    "form", "table", "field", "column", "class", "function", "variable", "street", "bank", "school", "team",
];
const SCORE: f64 = 0.8;
const MAX_VALUE: usize = 60;

/// Reads `Passport: K6974618`, `"Postcode": "EX31"`, `<Username>alex</Username>` and
/// `**Vorname:** Rilon`: a known field name, a separator, and the value up to the end of the field.
pub struct FieldDetector {
    keys: Regex,
    kinds: HashMap<String, Field>,
    suffix: Regex,
    dates_and_times: bool,
}

impl FieldDetector {
    pub fn new(dates_and_times: bool) -> Result<Self> {
        let mut names: Vec<(&str, Field)> = KEYS.iter().flat_map(|(f, ks)| ks.iter().map(|k| (*k, *f))).collect();
        // The regex takes the first alternative that matches, so longer names come first.
        names.sort_by_key(|(k, _)| std::cmp::Reverse(k.chars().count()));
        let pattern: Vec<String> = names
            .iter()
            .map(|(k, _)| k.split(' ').map(regex::escape).collect::<Vec<_>>().join(r"[ _\-]?"))
            .collect();
        Ok(Self {
            keys: Regex::new(&format!("(?i)(?:{})", pattern.join("|")))?,
            kinds: names.iter().map(|(k, f)| (squash(k), *f)).collect(),
            // `Passport Number`, `IDCard_F`, `Lastname2`, `Carte d'identité n°`.
            suffix: Regex::new(
                r"^(?i)[ _\-]?(?:numbers?|nummer|numéro|nr\.?|no\.?|n°|id|code|\d{1,2}|[a-z])?(?:_[a-z])?",
            )?,
            dates_and_times,
        })
    }
}

impl Detector for FieldDetector {
    fn detect(&self, text: &str, _: &[Token]) -> Vec<Span> {
        let mut spans = Vec::new();
        for m in self.keys.find_iter(text) {
            let Some(&field) = self.kinds.get(&squash(m.as_str())) else {
                continue;
            };
            if prev_char(text, m.start()).is_some_and(char::is_alphabetic) {
                continue;
            }
            if field == Field::DateTime && !self.dates_and_times {
                continue;
            }
            if field == Field::Person && thing_before(text, m.start()) {
                continue;
            }
            let after_key = m.end() + self.suffix.find(&text[m.end()..]).map_or(0, |s| s.end());
            // A password may hold any character, so it runs to the next space or closing quote.
            let verbatim = matches!(field, Field::Secret | Field::Pass);
            let Some((start, end)) = value(text, after_key, verbatim) else {
                continue;
            };
            if let Some(kind) = accept(field, &text[start..end]) {
                spans.push(Span::rule(text, kind, start, end, SCORE));
            }
        }
        resolve(spans)
    }
}

/// Lowercase without spaces, hyphens and underscores.
fn squash(key: &str) -> String {
    key.chars()
        .filter(|c| !matches!(c, ' ' | '_' | '-'))
        .flat_map(char::to_lowercase)
        .collect()
}

/// `company name`, `file_name`, `productName`: the word the field name is attached to.
fn thing_before(text: &str, at: usize) -> bool {
    let head = text[..at].trim_end_matches([' ', '_', '-']);
    let word_start = head
        .char_indices()
        .rev()
        .take_while(|(_, c)| c.is_alphabetic())
        .last()
        .map_or(head.len(), |(i, _)| i);
    NOT_A_PERSON.contains(&head[word_start..].to_lowercase().as_str())
}

/// The value after a field name: over quotes, emphasis marks and closing tags to a `:`, `=` or
/// `>`, then up to the end of the line, cell, tag or quoted string.
fn value(text: &str, from: usize, verbatim: bool) -> Option<(usize, usize)> {
    let decoration = |c: char| {
        matches!(
            c,
            ' ' | '\t' | '\u{a0}' | '"' | '\'' | '*' | '`' | '[' | '(' | ']' | ')'
        )
    };
    let mut at = from;
    let mut separated = false;
    loop {
        let rest = &text[at..];
        let c = rest.chars().next()?;
        if !separated && matches!(c, ':' | '=' | '>') {
            separated = true;
            at += 1;
        } else if c == '<' && rest[1..].starts_with(|c: char| c == '/' || c.is_ascii_alphabetic()) {
            // `<strong>Passeport : </strong>I8114435`
            at += rest.find('>')? + 1;
        } else if decoration(c) && (separated || !matches!(c, '[' | '(')) {
            at += c.len_utf8();
        } else {
            break;
        }
    }
    if !separated {
        return None;
    }
    let quote = prev_char(text, at).filter(|c| matches!(c, '"' | '\''));
    let rest = &text[at..];
    let mut end = rest.len();
    for (i, c) in rest.char_indices() {
        let closes = if verbatim {
            c.is_whitespace() || Some(c) == quote || rest[i..].starts_with("</") || (quote.is_none() && c == '"')
        } else {
            matches!(
                c,
                '\n' | '\r' | '\t' | '<' | '"' | '|' | ';' | ',' | '*' | ']' | '}' | '('
            ) || Some(c) == quote
                || rest[i..].starts_with("  ")
                || rest[i..].starts_with(" - ")
                || rest[i..].starts_with(" – ")
        };
        if closes {
            end = i;
            break;
        }
    }
    let trailing: &[char] = if verbatim { &[] } else { &[' ', '\u{a0}', '.', ')'] };
    let found = rest[..end].trim_end_matches(trailing);
    (!found.is_empty() && found.len() <= MAX_VALUE).then_some((at, at + found.len()))
}

fn accept(field: Field, v: &str) -> Option<EntityType> {
    let has_digit = v.contains(|c: char| c.is_ascii_digit());
    let words = v.split_whitespace().count();
    let capitalized = v.starts_with(char::is_uppercase);
    let name_like = !has_digit
        && v.chars()
            .all(|c| c.is_alphabetic() || matches!(c, ' ' | '-' | '\'' | '’' | '.'));
    let code_like = v
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, ' ' | '.' | '-' | '/'));
    let ok = match field {
        Field::Id => has_digit && code_like && v.len() >= 4,
        Field::Pass | Field::Secret => v.len() >= 3,
        Field::Phone => {
            v.bytes().filter(u8::is_ascii_digit).count() >= 6
                && v.chars()
                    .all(|c| c.is_ascii_digit() || matches!(c, ' ' | '\u{a0}' | '.' | '-' | '+' | '/'))
        }
        Field::Username => words == 1 && v.len() >= 2,
        Field::Birth => has_digit && v.len() <= 30,
        Field::DateTime => has_digit && v.len() <= 30,
        Field::Person => name_like && capitalized && words <= 5 && v.chars().count() >= 2,
        Field::Place => name_like && capitalized && words <= 4 && v.chars().count() >= 2,
        Field::Street => (capitalized || has_digit) && words <= 6 && v.chars().count() >= 3,
        Field::Building => has_digit && v.len() <= 8,
        Field::Postcode => has_digit && code_like && v.len() <= 10,
        Field::SecondLine => words <= 4 && v.len() <= 30,
    };
    ok.then_some(match field {
        Field::Id => EntityType::IdNumber,
        Field::Pass if has_digit && v.chars().all(|c| c.is_ascii_alphanumeric()) => EntityType::IdNumber,
        Field::Pass | Field::Secret | Field::Username => EntityType::Credential,
        Field::Phone => EntityType::Phone,
        Field::Birth => EntityType::DateOfBirth,
        Field::DateTime => EntityType::Date,
        Field::Person => EntityType::Person,
        Field::Place => EntityType::Location,
        Field::Street | Field::Building | Field::Postcode | Field::SecondLine => EntityType::Address,
    })
}
