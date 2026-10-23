use super::{run, texts};
use tagging_core::{EntityType, detectors::NationalIdDetector};

fn d() -> NationalIdDetector {
    NationalIdDetector::new().unwrap()
}

#[test]
fn finds_valid_matricules() {
    for x in [
        "1985031212307",
        "1985 0312 123 07",
        "1992 02 29 045 85",
        "2001070498726",
        "1976123100157",
    ] {
        assert_eq!(texts(&d(), &format!("Matricule: {x}.")), [x], "{x}");
    }
    assert_eq!(run(&d(), "1985031212307")[0].kind, EntityType::NationalId);
}

#[test]
fn rejects_wrong_luhn_digit() {
    assert!(texts(&d(), "1985031212317").is_empty());
}

#[test]
fn rejects_wrong_verhoeff_digit() {
    assert!(texts(&d(), "1985031212308").is_empty());
}

#[test]
fn rejects_impossible_dates_with_valid_checksums() {
    assert!(texts(&d(), "1985133112353").is_empty());
    assert!(texts(&d(), "1993022904576").is_empty());
}

#[test]
fn rejects_longer_numbers() {
    for t in ["19850312123071", "91985031212307", "1985031212307-4"] {
        assert!(texts(&d(), t).is_empty(), "{t}");
    }
}
