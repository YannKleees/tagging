use std::collections::HashMap;

use crate::names::{self, Lookup};
use crate::{EntityType, Source, Span, Token, detectors::Detector, resolve, tokenize};

const PLACES: &str = include_str!("../data/lu_places.txt");
const WORLD_PLACES: &str = include_str!("../data/world_places.txt");
const FIRST_NAMES: &str = include_str!("../data/lu_first_names.txt");
const GIVEN_NAMES: &str = include_str!("../data/given_names.txt");
const FAMILY_NAMES: &str = include_str!("../data/family_names.txt");
const ORGANIZATIONS: &str = include_str!("../data/lu_organizations.txt");
const WORLD_ORGANIZATIONS: &str = include_str!("../data/world_organizations.txt");

const SCORE: f64 = 0.85;
const AMBIGUOUS_SCORE: f64 = 0.55;
const LOWERCASE_SCORE: f64 = 0.45;
/// A name that is also an everyday word (`Rose`, `Orange`, `Split`) and stands alone. It stays
/// below the default threshold; a cue word or another name part lifts it.
const CONTEXT_SCORE: f64 = 0.3;
/// An ambiguous place or organization right after a preposition that asks for one.
const CUE_SCORE: f64 = 0.65;
#[rustfmt::skip]
const PLACE_CUES: &[&str] = &[
    "in", "zu", "à", "nach", "aus", "bei", "near", "from", "to", "op", "no", "vu", "vun", "von", "via",
];
const ORGANIZATION_CUES: &[&str] = &["bei", "chez", "at", "for", "fir", "für", "by"];
/// A lowercase match in text written without any capitals, where case says nothing.
const LOWERCASE_TEXT_SCORE: f64 = 0.6;
/// Words that make the next word a thing and not a name: `eng Méck`, `ein Berg`, `du Mett`.
#[rustfmt::skip]
const INDEFINITE: &[&str] = &[
    "e", "en", "eng", "engem", "enger", "kee", "keen", "keng", "ein", "eine", "einen", "einem", "einer", "kein",
    "keine", "keinen", "un", "une", "du", "des", "some", "any", "every", "vill", "viel", "viele", "beaucoup",
    "many", "puer", "paar", "quelques", "kilo", "kg", "gramm", "g", "pond", "pfund", "livre",
];

struct Entry {
    folded: Vec<String>,
    original: Vec<String>,
    kind: EntityType,
    score: f64,
    exact: bool,
}

/// Case-insensitive multi-token name lookup, indexed by the first folded token. Family names are
/// kept apart: they only count next to another name part.
#[derive(Default)]
pub struct Gazetteer {
    index: HashMap<String, Vec<Entry>>,
    family: HashMap<String, bool>,
}

impl Gazetteer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builtin() -> Self {
        let mut g = Self::new();
        g.add_list(PLACES, EntityType::Location);
        g.add_list(WORLD_PLACES, EntityType::Location);
        g.add_list(FIRST_NAMES, EntityType::Person);
        g.add_list(GIVEN_NAMES, EntityType::Person);
        g.add_family_names(FAMILY_NAMES);
        g.add_list(ORGANIZATIONS, EntityType::Organization);
        g.add_list(WORLD_ORGANIZATIONS, EntityType::Organization);
        g
    }

    /// One family name per line; `?` marks names that are also everyday words.
    pub fn add_family_names(&mut self, src: &str) {
        for line in lines(src) {
            let (name, distinct) = line.strip_prefix('?').map_or((line, true), |n| (n, false));
            let known = self.family.entry(fold(name.trim())).or_insert(distinct);
            *known = *known || distinct;
        }
    }

    /// Parses the list format described in `data/README.md`.
    pub fn add_list(&mut self, src: &str, kind: EntityType) {
        for line in lines(src) {
            let (line, score) = match line.split_at_checked(1) {
                Some(("~", rest)) => (rest, AMBIGUOUS_SCORE),
                Some(("?", rest)) => (rest, CONTEXT_SCORE),
                _ => (line, SCORE),
            };
            for name in line.split('|') {
                self.insert(name.trim(), kind, score);
            }
        }
    }

    pub fn add_words<S: AsRef<str>>(&mut self, words: &[S], kind: EntityType) {
        for w in words {
            self.insert(w.as_ref().trim(), kind, SCORE);
        }
    }

    fn insert(&mut self, name: &str, kind: EntityType, score: f64) {
        let original: Vec<String> = tokenize(name).iter().map(|t| t.text.to_string()).collect();
        let Some(first) = original.first() else { return };
        let folded: Vec<String> = original.iter().map(|t| fold(t)).collect();
        let exact = name.chars().any(char::is_alphabetic) && !name.chars().any(char::is_lowercase);
        let bucket = self.index.entry(fold(first)).or_default();
        let same = |e: &&mut Entry| e.kind == kind && e.exact == exact && e.folded == folded;
        match bucket
            .iter_mut()
            .find(|e| same(e) && (!exact || e.original == original))
        {
            Some(e) => e.score = e.score.max(score),
            None => bucket.push(Entry {
                folded,
                original,
                kind,
                score,
                exact,
            }),
        }
    }
}

impl Detector for Gazetteer {
    fn detect(&self, text: &str, tokens: &[Token]) -> Vec<Span> {
        let folded: Vec<String> = tokens.iter().map(|t| fold(t.text)).collect();
        let mut spans = Vec::new();
        for (i, key) in folded.iter().enumerate() {
            for e in self.index.get(key).into_iter().flatten() {
                let Some(window) = tokens.get(i..i + e.folded.len()) else {
                    continue;
                };
                let matches = folded[i..i + e.folded.len()] == e.folded[..]
                    && (!e.exact || window.iter().zip(&e.original).all(|(t, o)| t.text == o));
                if !matches {
                    continue;
                }
                let lowercase = window[0].text.chars().next().is_some_and(char::is_lowercase);
                // In lowercase a name that is also an everyday word is just the word: `an`, `rose`.
                if lowercase && e.score <= CONTEXT_SCORE {
                    continue;
                }
                let ambiguous = e.score < SCORE;
                let before = if i > 0 { folded[i - 1].as_str() } else { "" };
                let cues = match e.kind {
                    EntityType::Location => PLACE_CUES,
                    EntityType::Organization => ORGANIZATION_CUES,
                    _ => &[],
                };
                let score = if lowercase && !ambiguous && uncased(text, window[0].start) {
                    LOWERCASE_TEXT_SCORE
                } else if lowercase || (ambiguous && INDEFINITE.contains(&before)) {
                    e.score.min(LOWERCASE_SCORE)
                } else if ambiguous && cues.contains(&before) {
                    CUE_SCORE
                } else {
                    e.score
                };
                let end = window[window.len() - 1].end;
                spans.push(Span::new(text, e.kind, window[0].start, end, score, Source::Gazetteer));
            }
        }
        let lists = Lists {
            gazetteer: self,
            folded: &folded,
        };
        spans.extend(names::detect(text, tokens, &folded, &lists));
        resolve(spans)
    }
}

struct Lists<'a> {
    gazetteer: &'a Gazetteer,
    folded: &'a [String],
}

impl Lists<'_> {
    fn word(&self, i: usize) -> &str {
        self.folded.get(i).map_or("", String::as_str)
    }

    fn single(&self, word: &str, kind: EntityType) -> Option<f64> {
        let entries = self.gazetteer.index.get(word)?.iter();
        entries
            .filter(|e| e.kind == kind && e.folded.len() == 1 && !e.exact)
            .map(|e| e.score)
            .reduce(f64::max)
    }
}

impl Lookup for Lists<'_> {
    /// A hyphenated name is known when each part is: `Jean-Claude`, `Marie-Josée`.
    fn first_name(&self, i: usize) -> Option<f64> {
        let given = |w: &str| self.single(w, EntityType::Person);
        let parts = |w: &str| w.split('-').map(given).reduce(|a, b| Some(a?.min(b?)))?;
        let word = self.word(i);
        given(word).or_else(|| if word.contains('-') { parts(word) } else { None })
    }

    fn surname(&self, i: usize) -> bool {
        let word = self.word(i);
        !word.is_empty() && word.split('-').all(|p| self.gazetteer.family.contains_key(p))
    }

    fn distinct_surname(&self, i: usize) -> bool {
        self.surname(i) && self.word(i).split('-').any(|p| self.gazetteer.family[p])
    }

    fn listed(&self, i: usize) -> bool {
        self.gazetteer.index.contains_key(self.word(i))
    }

    fn place(&self, i: usize) -> bool {
        self.single(self.word(i), EntityType::Location)
            .is_some_and(|s| s >= SCORE)
    }
}

fn lines(src: &str) -> impl Iterator<Item = &str> {
    src.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
}

/// True when the line around `at` has no uppercase letter, as in lowercase chat messages.
fn uncased(text: &str, at: usize) -> bool {
    let start = text[..at].rfind('\n').map_or(0, |i| i + 1);
    let end = text[at..].find('\n').map_or(text.len(), |i| at + i);
    let line = &text[start..end];
    !line.chars().any(char::is_uppercase) && line.split_whitespace().count() >= 3
}

fn fold(s: &str) -> String {
    s.chars()
        .flat_map(char::to_lowercase)
        .map(|c| if c == '’' { '\'' } else { c })
        .collect()
}
