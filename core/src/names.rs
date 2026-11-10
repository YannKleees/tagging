//! Name grammar on top of the word lists: a title or a first name followed by a surname, and
//! businesses named after a person or a place (`Garage Weis`, `Muller & Fils S.à r.l.`).

use crate::{EntityType, Source, Span, Token};

#[rustfmt::skip]
const TITLES: &[&str] = &[
    "mr", "mrs", "ms", "miss", "mister", "sir", "dr", "prof", "professor",
    "monsieur", "madame", "mademoiselle", "mme", "mlle", "docteur", "docteure", "maître",
    "herr", "herrn", "frau", "doktor", "fräulein",
    "här", "joffer", "dokter", "doktesch", "madamm",
    "president", "président", "présidente", "präsident", "präsidentin", "minister", "ministre", "ministerin",
    "director", "directeur", "directrice", "direktor", "direktorin", "direkter", "direktesch",
];
/// `M. Dupont`, `Me. Weber`: only a title when written with its period.
const DOTTED_TITLES: &[&str] = &["m", "mm", "me"];
#[rustfmt::skip]
const PARTICLES: &[&str] = &[
    "de", "du", "des", "del", "della", "di", "da", "dos", "das", "van", "von", "vom", "der", "den", "la", "le",
    "ter", "ten", "d'", "l'",
];
/// Capitalized words that follow a first name without being a surname: `Jhemp Méindeg`.
#[rustfmt::skip]
const NOT_SURNAMES: &[&str] = &[
    "monday", "tuesday", "wednesday", "thursday", "friday", "saturday", "sunday",
    "lundi", "mardi", "mercredi", "jeudi", "vendredi", "samedi", "dimanche",
    "montag", "dienstag", "mittwoch", "donnerstag", "freitag", "samstag", "sonntag",
    "méindeg", "dënschdeg", "mëttwoch", "donneschdeg", "freideg", "samschdeg", "sonndeg",
    "january", "february", "march", "april", "may", "june", "july", "august", "september", "october", "november",
    "december", "januar", "februar", "märz", "mai", "juni", "juli", "oktober", "dezember",
    "janvier", "février", "mars", "avril", "juin", "juillet", "août", "septembre", "octobre", "novembre", "décembre",
    "mäerz", "abrëll", "mee", "tel", "tél", "email", "mail", "iban", "gsm", "fax",
];
#[rustfmt::skip]
const BUSINESSES: &[&str] = &[
    "garage", "brasserie", "boulangerie", "pâtisserie", "boucherie", "pharmacie", "café", "cafe", "restaurant",
    "hôtel", "hotel", "banque", "librairie", "menuiserie", "fiduciaire", "étude", "cabinet", "ets", "établissements",
    "etablissements", "maison", "domaine", "caves", "imprimerie", "entreprise", "société", "agence", "clinique",
    "auto-école", "taxi", "taxis", "transports", "voyages", "assurances", "bijouterie", "optique", "coiffure",
    "bäckerei", "metzgerei", "apotheke", "schreinerei", "fahrschule", "firma", "weingut", "autohaus", "gasthaus",
    "apdikt", "bäckerei", "metzlerei", "buttek", "schräinerei",
];
/// Company forms, compared without dots and in lowercase. Two-letter forms must be uppercase.
#[rustfmt::skip]
const LEGAL_FORMS: &[&str] = &[
    "sa", "sarl", "sàrl", "sàrl-s", "sca", "scs", "snc", "sas", "sci", "se", "gie", "asbl", "sprl", "srl",
    "gmbh", "ag", "kg", "ohg", "ug", "ev", "ltd", "limited", "inc", "llc", "plc", "corp", "bv", "nv",
];
const LINKS: &[&str] = &["&", "et", "und", "an", "a", "and", "de", "du", "vum", "vun"];
/// Links that may follow the business word directly: `Café de Paris`.
const OF: &[&str] = &["de", "du", "vum", "vun"];
/// Links inside a company name: `Muller & Fils`, `Banque de Luxembourg`.
const COMPANY_LINKS: &[&str] = &["&", "et", "und", "and", "de", "du", "des"];
/// Company forms that are also written in lowercase: `Caritas asbl`.
const LOWERCASE_FORMS: &[&str] = &["asbl", "sàrl", "sarl", "sprl", "gmbh"];
/// Words in front of a company name that are not part of it: `Die Firma Muller GmbH`.
const NOT_COMPANY: &[&str] = &[
    "der", "die", "das", "la", "le", "les", "the", "bei", "chez", "by", "von", "de",
];

const TITLED: f64 = 0.85;
const FULL_NAME: f64 = 0.9;
const GUESSED_SURNAME: f64 = 0.7;
const STRONG: f64 = 0.8;
const WEAK: f64 = 0.5;
const BUSINESS: f64 = 0.8;
const COMPANY: f64 = 0.85;
const MAX_NAME_TOKENS: usize = 4;

/// What the word lists know about each token of a text.
pub(crate) trait Lookup {
    /// Score of a first name; below `WEAK` for one that only counts next to another name part.
    fn first_name(&self, i: usize) -> Option<f64>;
    fn surname(&self, i: usize) -> bool;
    /// A surname that is not also an everyday word (`Weis`, but not `Zimmer`).
    fn distinct_surname(&self, i: usize) -> bool;
    /// Start of any list entry: a place, an organization or a first name.
    fn listed(&self, i: usize) -> bool;
    fn place(&self, i: usize) -> bool;
}

struct Grammar<'a, L> {
    text: &'a str,
    tokens: &'a [Token<'a>],
    folded: &'a [String],
    lists: &'a L,
}

pub(crate) fn detect<L: Lookup>(text: &str, tokens: &[Token], folded: &[String], lists: &L) -> Vec<Span> {
    let g = Grammar {
        text,
        tokens,
        folded,
        lists,
    };
    let mut spans = g.people();
    spans.extend(g.businesses());
    spans.extend(g.companies());
    spans
}

impl<L: Lookup> Grammar<'_, L> {
    fn word(&self, i: usize) -> &str {
        self.folded.get(i).map_or("", String::as_str)
    }

    fn token(&self, i: usize) -> &str {
        self.tokens.get(i).map_or("", |t| t.text)
    }

    /// A capitalized word made of letters, hyphens and apostrophes: `Kieffer`, `O'Donnell`.
    fn name_shaped(&self, i: usize) -> bool {
        let t = self.token(i);
        let mut chars = t.chars();
        chars.next().is_some_and(char::is_uppercase)
            && t.chars().count() >= 2
            && chars.all(|c| c.is_alphabetic() || matches!(c, '-' | '\'' | '’'))
    }

    fn all_caps(&self, i: usize) -> bool {
        !self.token(i).chars().any(char::is_lowercase)
    }

    fn title(&self, i: usize) -> bool {
        let w = self.word(i);
        TITLES.contains(&w) || (DOTTED_TITLES.contains(&w) && self.token(i + 1) == ".")
    }

    /// The period of `Dr.` or `Mme.`, but not the one that ends `ask Mr President.`
    fn abbreviation_period(&self, i: usize) -> bool {
        let glued = i > 0 && self.tokens.get(i).is_some_and(|t| t.start == self.tokens[i - 1].end);
        self.token(i) == "." && glued && self.title(i - 1) && self.word(i - 1).chars().count() <= 4
    }

    fn known(&self, i: usize) -> bool {
        self.name_shaped(i) && (self.lists.first_name(i).is_some() || self.lists.surname(i))
    }

    fn particle(&self, i: usize) -> bool {
        PARTICLES.contains(&self.word(i))
    }

    /// Index of the last token of a surname starting at `i`: `Kieffer`, `de Kieffer`.
    fn surname_end(&self, i: usize) -> Option<usize> {
        if self.name_shaped(i) && self.lists.surname(i) {
            return Some(i);
        }
        // `Jean de Luxembourg` is more often a person in a place than a surname.
        let after = (i..i + 2).find(|&k| !self.particle(k))?;
        let surname = self.name_shaped(after) && self.lists.surname(after) && !self.lists.place(after);
        (after > i && surname).then_some(after)
    }

    fn span(&self, kind: EntityType, first: usize, last: usize, score: f64) -> Span {
        let (start, end) = (self.tokens[first].start, self.tokens[last].end);
        Span::new(self.text, kind, start, end, score, Source::Gazetteer)
    }

    fn people(&self) -> Vec<Span> {
        let mut out = Vec::new();
        let mut i = 0;
        while i < self.tokens.len() {
            let found = self
                .titled(i)
                .or_else(|| self.full_name(i))
                .or_else(|| self.surname_first(i));
            match found {
                Some((first, last, score)) => {
                    out.push(self.span(EntityType::Person, first, last, score));
                    i = last + 1;
                }
                None => i += 1,
            }
        }
        out
    }

    /// `Här Kieffer`, `Dr. Marie Weber`: the name after one or more titles, without the titles.
    fn titled(&self, i: usize) -> Option<(usize, usize, f64)> {
        if !self.title(i) {
            return None;
        }
        let mut first = i + 1;
        while self.title(first) || self.abbreviation_period(first) {
            first += 1;
        }
        if !self.name_shaped(first) || NOT_SURNAMES.contains(&self.word(first)) {
            return None;
        }
        let mut last = first;
        while last + 1 - first < MAX_NAME_TOKENS {
            if self.known(last + 1) {
                last += 1;
            } else if let Some(end) = self.surname_end(last + 1) {
                last = end;
            } else {
                break;
            }
        }
        Some((first, last, TITLED))
    }

    /// `Jhemp Kieffer`, `Marie-Anne de Kieffer`: first names followed by a surname. An unlisted
    /// capitalized word after an unambiguous first name is taken as the surname with a lower score.
    fn full_name(&self, i: usize) -> Option<(usize, usize, f64)> {
        if !self.name_shaped(i) {
            return None;
        }
        let mut score = self.lists.first_name(i)?;
        let mut last = i;
        while last + 1 - i < 2 && self.name_shaped(last + 1) && !self.lists.surname(last + 1) {
            let Some(s) = self.lists.first_name(last + 1) else {
                break;
            };
            score = score.min(s);
            last += 1;
        }
        let given = last;
        while last - given < 2 {
            match self.surname_end(last + 1) {
                Some(end) => last = end,
                None => break,
            }
        }
        if last > given {
            // `May Day`: a first name and a surname that are both everyday words prove nothing.
            let distinct = (given + 1..=last).any(|k| self.lists.distinct_surname(k));
            return (score >= WEAK || distinct).then_some((i, last, FULL_NAME));
        }
        if score < STRONG {
            return None;
        }
        if given == i && self.token(i).contains('-') {
            return Some((i, i, score));
        }
        if self.unlisted_surname(given + 1) {
            return Some((i, given + 1, GUESSED_SURNAME));
        }
        // Several first names in a row are one person: `Jean Pierre`.
        (given > i).then_some((i, given, score))
    }

    /// `KIEFFER Jean`: forms and registers put the surname first, in capitals.
    fn surname_first(&self, i: usize) -> Option<(usize, usize, f64)> {
        let capitals = self.all_caps(i) && self.token(i).chars().count() >= 3;
        if !capitals || !self.name_shaped(i) || !self.lists.surname(i) {
            return None;
        }
        let mut last = i;
        while last - i < 2 && self.name_shaped(last + 1) && self.lists.first_name(last + 1).is_some_and(|s| s >= WEAK) {
            last += 1;
        }
        (last > i).then_some((i, last, FULL_NAME))
    }

    fn unlisted_surname(&self, i: usize) -> bool {
        self.name_shaped(i)
            && self.token(i).chars().count() >= 3
            && !self.all_caps(i)
            && !self.lists.listed(i)
            && !self.title(i)
            && !NOT_SURNAMES.contains(&self.word(i))
    }

    /// `Garage Weis`, `Brasserie Schuman`: a kind of business followed by a listed name.
    fn businesses(&self) -> Vec<Span> {
        let mut out = Vec::new();
        for i in 0..self.tokens.len() {
            let capitalized = self.token(i).chars().next().is_some_and(char::is_uppercase);
            if !capitalized || !BUSINESSES.contains(&self.word(i)) {
                continue;
            }
            let named = |k: usize| {
                let first_name = self.lists.first_name(k).is_some_and(|s| s >= STRONG);
                self.name_shaped(k) && (first_name || self.lists.distinct_surname(k) || self.lists.place(k))
            };
            let mut last = i;
            while last - i < MAX_NAME_TOKENS {
                let link = if last > i { LINKS } else { OF };
                if named(last + 1) {
                    last += 1;
                } else if link.contains(&self.word(last + 1)) && named(last + 2) {
                    last += 2;
                } else {
                    break;
                }
            }
            if last > i {
                out.push(self.span(EntityType::Organization, i, last, BUSINESS));
            }
        }
        out
    }

    /// `Muller & Fils S.à r.l.`, `Arendt GmbH`: capitalized words in front of a company form.
    fn companies(&self) -> Vec<Span> {
        let mut out = Vec::new();
        for i in 1..self.tokens.len() {
            let Some(last) = self.legal_form(i) else { continue };
            let mut first = i;
            while first > 0 && i - first < MAX_NAME_TOKENS {
                let k = first - 1;
                let link = COMPANY_LINKS.contains(&self.word(k)) && k > 0 && first < i;
                let name = self
                    .token(k)
                    .chars()
                    .next()
                    .is_some_and(|c| c.is_uppercase() || c.is_ascii_digit());
                let outside = NOT_COMPANY.contains(&self.word(k)) || self.title(k) || self.legal_form(k).is_some();
                if !(name || link) || outside {
                    break;
                }
                first = k;
            }
            while first < i && COMPANY_LINKS.contains(&self.word(first)) {
                first += 1;
            }
            if first < i {
                out.push(self.span(EntityType::Organization, first, last, COMPANY));
            }
        }
        out
    }

    /// Index of the last token of a company form starting at `i`.
    fn legal_form(&self, i: usize) -> Option<usize> {
        let plain = |k: usize| self.word(k).replace('.', "");
        let dotted = self.token(i).contains('.');
        let glued = |k: usize| self.tokens.get(k + 1).is_some_and(|n| n.start == self.tokens[k].end);
        // The tokenizer splits the final period off `S.A.` and `S.à r.l.`.
        let end = |k: usize| if self.token(k + 1) == "." && glued(k) { k + 1 } else { k };
        if plain(i) == "sà" && plain(i + 1) == "rl" {
            return Some(end(i + 1));
        }
        let form = plain(i);
        let capitalized = self.token(i).chars().next().is_some_and(char::is_uppercase);
        let written = match form.as_str() {
            _ if dotted => true,
            // Without its dots `EV` is an electric vehicle.
            "ev" => false,
            f if f.chars().count() <= 2 => self.all_caps(i),
            f => capitalized || LOWERCASE_FORMS.contains(&f),
        };
        (LEGAL_FORMS.contains(&form.as_str()) && written).then(|| if dotted { end(i) } else { i })
    }
}
