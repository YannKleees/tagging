use super::{run, texts};
use tagging_core::{EntityType, detectors::DateDetector};

fn d() -> DateDetector {
    DateDetector::new(false).unwrap()
}

fn assert_dob(text: &str, date: &str) {
    let s = run(&d(), text);
    assert_eq!(s.len(), 1, "{text}");
    assert_eq!(
        (s[0].text.as_str(), s[0].kind),
        (date, EntityType::DateOfBirth),
        "{text}"
    );
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
    assert_dob("Fernande, b. 08/02/1982 in Finsterthal", "08/02/1982");
    assert_dob("Müller, Hans (* 01.05.1990)", "01.05.1990");
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
    for t in [
        "born 31.02.1985",
        "born 12.13.1985",
        "born 2023-02-30",
        "born 32 mars 1985",
    ] {
        assert!(texts(&d(), t).is_empty(), "{t}");
    }
}

#[test]
fn rejects_non_dates() {
    for t in ["born 1.2.3", "Price 12.50", "v1.12.2024.3", "Order 12.03.198567"] {
        assert!(texts(&d(), t).is_empty(), "{t}");
    }
}

#[test]
fn finds_birth_keyword_after_verb_final_date() {
    assert_dob("Norbert Muller wurde am 03.02.1961 in Luxemburg geboren.", "03.02.1961");
    assert_dob("Hien ass den 03.02.1961 zu Lëtzebuerg gebuer.", "03.02.1961");
}

#[test]
fn a_capital_b_before_a_date_is_not_born() {
    for t in ["Plan B. 08/02/2021 is the deadline", "tab. 08/02/2021"] {
        assert_eq!(run(&d(), t)[0].kind, EntityType::Date, "{t}");
    }
}

#[test]
fn reads_timestamps_ordinals_and_partial_dates() {
    assert_dob("Born on: 1960-06-24T00:00:00 |", "1960-06-24T00:00:00");
    assert_dob("née le 30e juin 2002", "30e juin 2002");
    assert_dob("geboren am Januar 19., 2000", "Januar 19., 2000");
    assert_dob("born in June 1942", "June 1942");
    assert_dob("Born on: December/38", "December/38");
}

#[test]
fn other_dates_and_times_need_the_option() {
    let text = "Termin am 14.11.2024 um 14:30, nicht 8:20 PM oder 18 Uhr.";
    assert!(run(&d(), text).iter().all(|s| s.score < 0.5));
    let all = DateDetector::new(true).unwrap();
    assert_eq!(texts(&all, text), ["14.11.2024", "14:30", "8:20 PM", "18 Uhr"]);
    assert!(
        run(&all, text)
            .iter()
            .all(|s| s.score >= 0.5 && s.kind == EntityType::Date)
    );
    // Not times: a ratio, a score, part of an IPv6 address.
    assert!(texts(&all, "ratio 3:1, 2a02:12:30::1").is_empty());
}
