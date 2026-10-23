mod card;
mod date;
mod email;
mod gazetteer;
mod iban;
mod ip;
mod national_id;
mod phone;
mod url;

use tagging_core::{Span, detectors::Detector, tokenize};

pub fn run(d: &impl Detector, text: &str) -> Vec<Span> {
    let mut spans = d.detect(text, &tokenize(text));
    spans.sort_by_key(|s| (s.start, s.end));
    spans
}

pub fn texts(d: &impl Detector, text: &str) -> Vec<String> {
    run(d, text).into_iter().map(|s| s.text).collect()
}
