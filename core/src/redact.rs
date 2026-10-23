use std::collections::HashMap;
use std::fmt::Write;

use serde::{Deserialize, Serialize};

use crate::{EntityType, Error, Result, Span};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    Mask,
    Pseudo,
}

/// Replaces each span with `[REDACTED]` or, in pseudo mode, a numbered placeholder such as
/// `[PERSON_1]`. Placeholders are keyed on type and value with case and punctuation removed, so
/// `621 123 456` and `621123456` share one placeholder.
pub fn redact(text: &str, spans: &[Span], mode: Mode) -> Result<String> {
    let mut sorted: Vec<&Span> = spans.iter().collect();
    sorted.sort_by_key(|s| (s.start, s.end));
    let mut out = String::with_capacity(text.len());
    let mut ids: HashMap<(EntityType, String), usize> = HashMap::new();
    let mut counts: HashMap<EntityType, usize> = HashMap::new();
    let mut prev: Option<&Span> = None;
    for s in sorted {
        let in_bounds = s.start < s.end && s.end <= text.len();
        if !in_bounds || !text.is_char_boundary(s.start) || !text.is_char_boundary(s.end) {
            return Err(Error::InvalidSpan {
                start: s.start,
                end: s.end,
            });
        }
        if let Some(p) = prev.filter(|p| p.end > s.start) {
            return Err(Error::OverlappingSpans((p.start, p.end), (s.start, s.end)));
        }
        out.push_str(&text[prev.map_or(0, |p| p.end)..s.start]);
        match mode {
            Mode::Mask => out.push_str("[REDACTED]"),
            Mode::Pseudo => {
                let key = (s.kind, normalize(&text[s.start..s.end]));
                let n = *ids.entry(key).or_insert_with(|| {
                    let c = counts.entry(s.kind).or_insert(0);
                    *c += 1;
                    *c
                });
                let _ = write!(out, "[{}_{n}]", s.kind.label());
            }
        }
        prev = Some(s);
    }
    out.push_str(&text[prev.map_or(0, |p| p.end)..]);
    Ok(out)
}

fn normalize(value: &str) -> String {
    value
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}
