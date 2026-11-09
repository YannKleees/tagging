//! Browser bindings for the demo page: rules and gazetteers only, no NER model.

use std::sync::OnceLock;

use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::{Config, Mode, Tagger};

fn tagger() -> &'static Tagger {
    static TAGGER: OnceLock<Tagger> = OnceLock::new();
    TAGGER.get_or_init(|| Tagger::new(&Config::default()).expect("the default config is valid"))
}

/// A piece of the input: plain text, or a detected entity with its type.
#[derive(Serialize)]
struct Segment<'a> {
    text: &'a str,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    kind: Option<&'static str>,
}

/// Splits the text into plain and detected segments, as JSON. Segments avoid byte offsets,
/// which do not match JavaScript's UTF-16 string indices.
#[wasm_bindgen]
pub fn annotate(text: &str) -> Result<String, JsError> {
    let spans = tagger().detect(text);
    let mut segments = Vec::new();
    let mut at = 0;
    for s in &spans {
        if s.start < at {
            continue;
        }
        if s.start > at {
            segments.push(Segment {
                text: &text[at..s.start],
                kind: None,
            });
        }
        segments.push(Segment {
            text: &text[s.start..s.end],
            kind: Some(s.kind.as_str()),
        });
        at = s.end;
    }
    if at < text.len() {
        segments.push(Segment {
            text: &text[at..],
            kind: None,
        });
    }
    Ok(serde_json::to_string(&segments)?)
}

/// Redacts the text with `mode` "mask" (`[REDACTED]`) or "pseudo" (`[PERSON_1]`, ...).
#[wasm_bindgen]
pub fn redact(text: &str, mode: &str) -> Result<String, JsError> {
    let mode: Mode = serde_json::from_value(mode.into())?;
    Ok(crate::redact(text, &tagger().detect(text), mode)?)
}
