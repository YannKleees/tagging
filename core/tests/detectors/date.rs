use super::{run, texts};
use tagging_core::{EntityType, detectors::DateDetector};

fn d() -> DateDetector {
    DateDetector::new().unwrap()
}

fn assert_dob(text: &str, date: &str) {
    let s = run(&d(), text);
    assert_eq!(s.len(), 1, "{text}");
    assert_eq!((s[0].text.as_str(), s[0].kind), (date, EntityType::DateOfBirth), "{text}");
    assert!(s[0].score >= 0.9);
}

#[test]
fn finds_birth_dates_with_context() {
    assert_dob("born on 12.03.1985 in Remich", "12.03.1985");
    assert_dob("geboren am 12. März 1985", "12. März 1985");
    assert_dob("née le 1er mars 1985", "1er mars 1985");
    assert_dob("gebuer den 12. Mäerz 1985", "12. Mäerz 1985");
    assert_dob("Date of birth: 1985-03-12", "1985-03-12");
    assert_dob("DOB: 12/03/1985", "12/03/1985");
    assert_dob("Geburtsdatum: 12.03.85", "12.03.85");
    assert_dob("He was born March 12, 1985.", "March 12, 1985");
    assert_dob("né(e) le 3-4-1990", "3-4-1990");
    assert_dob("geb. 7 Abrëll 1960", "7 Abrëll 1960");
}

#[test]
fn plain_dates_get_low_score() {
    let s = run(&d(), "Invoice date 12.03.2024");
    assert_eq!(s.len(), 1);
    assert_eq!(s[0].kind, EntityType::Date);
    assert!(s[0].score < 0.5);
}

#[test]
fn short_keywords_match_whole_words_only() {
    let s = run(&d(), "nécessaire le 12.03.2024");
    assert_eq!(s[0].kind, EntityType::Date);
}

#[test]
fn rejects_impossible_dates() {
    for t in ["born 31.02.1985", "born 12.13.1985", "born 2023-02-30", "born 32 mars 1985"] {
        assert!(texts(&d(), t).is_empty(), "{t}");
    }
}

#[test]
fn rejects_non_dates() {
    for t in ["born 1.2.3", "Price 12.50", "v1.12.2024.3", "Order 12.03.198567"] {
        assert!(texts(&d(), t).is_empty(), "{t}");
    }
}
