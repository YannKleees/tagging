use std::collections::HashMap;

use crate::{EntityType, Source, Span, Token, detectors::Detector, resolve, tokenize};

const PLACES: &str = include_str!("../data/lu_places.txt");
const FIRST_NAMES: &str = include_str!("../data/lu_first_names.txt");
const ORGANIZATIONS: &str = include_str!("../data/lu_organizations.txt");

const SCORE: f64 = 0.85;
const AMBIGUOUS_SCORE: f64 = 0.55;
const LOWERCASE_SCORE: f64 = 0.45;

struct Entry {
    folded: Vec<String>,
    original: Vec<String>,
    kind: EntityType,
    score: f64,
    exact: bool,
}

/// Case-insensitive multi-token name lookup, indexed by the first folded token.
#[derive(Default)]
pub struct Gazetteer {
    index: HashMap<String, Vec<Entry>>,
}

impl Gazetteer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builtin() -> Self {
        let mut g = Self::new();
        g.add_list(PLACES, EntityType::Location);
        g.add_list(FIRST_NAMES, EntityType::Person);
        g.add_list(ORGANIZATIONS, EntityType::Organization);
        g
    }

    /// Parses the list format described in `data/README.md`.
    pub fn add_list(&mut self, src: &str, kind: EntityType) {
        for line in src
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
        {
            let (line, score) = line.strip_prefix('~').map_or((line, SCORE), |l| (l, AMBIGUOUS_SCORE));
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
                let score = if lowercase {
                    e.score.min(LOWERCASE_SCORE)
                } else {
                    e.score
                };
                let end = window[window.len() - 1].end;
                spans.push(Span::new(text, e.kind, window[0].start, end, score, Source::Gazetteer));
            }
        }
        resolve(spans)
    }
}

fn fold(s: &str) -> String {
    s.chars()
        .flat_map(char::to_lowercase)
        .map(|c| if c == '’' { '\'' } else { c })
        .collect()
}
