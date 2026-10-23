use tagging_core::{Config, EntityType, Mode, Source, Span, Tagger, redact, resolve, tokenize};

fn toks(text: &str) -> Vec<&str> {
    tokenize(text).into_iter().map(|t| t.text).collect()
}

fn span(kind: EntityType, start: usize, end: usize, score: f64, text: &str) -> Span {
    Span {
        kind,
        start,
        end,
        text: text[start..end].to_string(),
        score,
        source: Source::Rule,
    }
}

#[test]
fn tokenizer_keeps_emails_urls_decimals_and_domains() {
    assert_eq!(toks("Mail jhemp@pt.lu."), ["Mail", "jhemp@pt.lu", "."]);
    assert_eq!(
        toks("See https://www.cfl.lu/fr?x=1, ok"),
        ["See", "https://www.cfl.lu/fr?x=1", ",", "ok"]
    );
    assert_eq!(
        toks("Pi is 3.14 and 1.250,00 on post.lu!"),
        ["Pi", "is", "3.14", "and", "1.250,00", "on", "post.lu", "!"]
    );
}

#[test]
fn tokenizer_splits_elision_and_edge_punctuation() {
    assert_eq!(toks("D'Mett an de Méck."), ["D'", "Mett", "an", "de", "Méck", "."]);
    assert_eq!(toks("d’Lis (Remich)"), ["d’", "Lis", "(", "Remich", ")"]);
    assert_eq!(toks("Jhemp's car"), ["Jhemp", "'s", "car"]);
    assert_eq!(toks("«Jang»"), ["«", "Jang", "»"]);
    assert_eq!(toks("O'Brien"), ["O'Brien"]);
}

#[test]
fn tokenizer_offsets_are_bytes_into_original() {
    let t = "Ettelbréck,  Lëtzebuerg\n\td'Méck?";
    let tokens = tokenize(t);
    for tok in &tokens {
        assert_eq!(&t[tok.start..tok.end], tok.text);
    }
    let joined: String = tokens.iter().map(|t| t.text).collect();
    assert_eq!(joined, t.split_whitespace().collect::<String>());
    assert!(tokenize("  \n ").is_empty());
}

#[test]
fn resolve_keeps_longest_then_highest_score() {
    let t = "IBAN LU28 0019 4006 4475 0000";
    let out = resolve(vec![
        span(EntityType::Phone, 15, 29, 0.9, t),
        span(EntityType::Iban, 5, 29, 0.99, t),
        span(EntityType::Date, 0, 4, 0.4, t),
        span(EntityType::Person, 0, 4, 0.6, t),
    ]);
    let kinds: Vec<_> = out.iter().map(|s| s.kind).collect();
    assert_eq!(kinds, [EntityType::Person, EntityType::Iban]);
}

#[test]
fn resolve_is_deterministic_and_sorted() {
    let t = "a b c d e f";
    let spans = vec![
        span(EntityType::Person, 8, 9, 0.5, t),
        span(EntityType::Location, 0, 1, 0.5, t),
        span(EntityType::Location, 2, 3, 0.5, t),
        span(EntityType::Person, 2, 3, 0.5, t),
    ];
    let mut reversed = spans.clone();
    reversed.reverse();
    let a = resolve(spans);
    assert_eq!(a, resolve(reversed));
    assert_eq!(a.iter().map(|s| s.start).collect::<Vec<_>>(), [0, 2, 8]);
}

#[test]
fn resolve_keeps_adjacent_spans() {
    let t = "abcdef";
    assert_eq!(
        resolve(vec![
            span(EntityType::Person, 0, 3, 0.5, t),
            span(EntityType::Person, 3, 6, 0.5, t)
        ])
        .len(),
        2
    );
}

#[test]
fn tagger_detects_mixed_text_in_order() {
    let t = "De Jhemp (jhemp@pt.lu, +352 621 123 456) wunnt zu Réimech, IBAN LU28 0019 4006 4475 0000.";
    let spans = Tagger::new(&Config::default()).unwrap().detect(t);
    let got: Vec<_> = spans.iter().map(|s| (s.kind, s.text.as_str())).collect();
    assert_eq!(
        got,
        [
            (EntityType::Person, "Jhemp"),
            (EntityType::Email, "jhemp@pt.lu"),
            (EntityType::Phone, "+352 621 123 456"),
            (EntityType::Location, "Réimech"),
            (EntityType::Iban, "LU28 0019 4006 4475 0000"),
        ]
    );
}

#[test]
fn tagger_applies_threshold_and_detector_selection() {
    let t = "Invoice 12.03.2024 to mett@example.com";
    let default = Tagger::new(&Config::default()).unwrap();
    assert_eq!(default.detect(t).len(), 1);
    let low = Tagger::new(&Config {
        threshold: 0.3,
        ..Config::default()
    })
    .unwrap();
    assert_eq!(low.detect(t).len(), 2);
    let cfg = Config::from_json(r#"{"detectors": ["date"], "threshold": 0.3}"#).unwrap();
    let only_dates = Tagger::new(&cfg).unwrap().detect(t);
    assert_eq!(
        only_dates.iter().map(|s| s.kind).collect::<Vec<_>>(),
        [EntityType::Date]
    );
}

#[test]
fn config_supports_wordlists_and_gazetteer_files() {
    let dir = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("config_test");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("names.txt"), "Kieffer\n").unwrap();
    std::fs::write(
        dir.join("config.json"),
        r#"{"builtin_gazetteers": false, "gazetteers": [{"path": "names.txt", "type": "person"}],
            "wordlists": [{"type": "organization", "words": ["Acme Sàrl"]}]}"#,
    )
    .unwrap();
    let cfg = Config::from_path(&dir.join("config.json")).unwrap();
    let spans = Tagger::new(&cfg)
        .unwrap()
        .detect("Den Här Kieffer vun Acme Sàrl zu Remich");
    let got: Vec<_> = spans.iter().map(|s| s.text.as_str()).collect();
    assert_eq!(got, ["Kieffer", "Acme Sàrl"]);
}

#[test]
fn config_rejects_unknown_fields_and_detectors() {
    assert!(Config::from_json(r#"{"treshold": 0.5}"#).is_err());
    assert!(Config::from_json(r#"{"detectors": ["ssn"]}"#).is_err());
    assert!(Config::from_path(std::path::Path::new("does/not/exist.json")).is_err());
}

#[test]
fn redact_masks_spans() {
    let t = "Mail jhemp@pt.lu now";
    let spans = Tagger::new(&Config::default()).unwrap().detect(t);
    assert_eq!(redact(t, &spans, Mode::Mask).unwrap(), "Mail [REDACTED] now");
}

#[test]
fn pseudonymization_is_consistent_within_document() {
    let t = "Jhemp met Pol. JHEMP called 621 123 456, then 621123456 again.";
    let spans = Tagger::new(&Config::default()).unwrap().detect(t);
    assert_eq!(
        redact(t, &spans, Mode::Pseudo).unwrap(),
        "[PERSON_1] met [PERSON_2]. [PERSON_1] called [PHONE_1], then [PHONE_1] again."
    );
}

#[test]
fn redact_accepts_unsorted_spans() {
    let t = "ab cd";
    let spans = [
        span(EntityType::Email, 3, 5, 0.9, t),
        span(EntityType::Person, 0, 2, 0.9, t),
    ];
    assert_eq!(redact(t, &spans, Mode::Pseudo).unwrap(), "[PERSON_1] [EMAIL_1]");
}

#[test]
fn redact_rejects_bad_spans() {
    let t = "Méck abc";
    let mut s = span(EntityType::Person, 0, 5, 0.9, t);
    assert!(redact(t, &[s.clone(), span(EntityType::Person, 3, 8, 0.9, t)], Mode::Mask).is_err());
    s.end = 2;
    assert!(redact(t, &[s.clone()], Mode::Mask).is_err());
    s.end = 99;
    assert!(redact(t, &[s], Mode::Mask).is_err());
}
