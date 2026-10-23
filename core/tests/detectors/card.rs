use super::{run, texts};
use tagging_core::{EntityType, detectors::CardDetector};

fn d() -> CardDetector {
    CardDetector::new().unwrap()
}

#[test]
fn finds_valid_cards() {
    for x in [
        "4111 1111 1111 1111",
        "4111-1111-1111-1111",
        "5555555555554444",
        "3782 822463 10005",
        "6011 1111 1111 1117",
        "4012888888881881",
    ] {
        assert_eq!(texts(&d(), &format!("Card: {x}.")), [x], "{x}");
    }
    assert_eq!(run(&d(), "4111111111111111")[0].kind, EntityType::CreditCard);
}

#[test]
fn stops_before_expiry() {
    assert_eq!(texts(&d(), "4111 1111 1111 1111 12/27"), ["4111 1111 1111 1111"]);
}

#[test]
fn rejects_invalid_luhn() {
    for t in ["4111 1111 1111 1112", "1234 5678 1234 5678", "5555555555554445"] {
        assert!(texts(&d(), t).is_empty(), "{t}");
    }
}

#[test]
fn rejects_luhn_valid_numbers_without_card_prefix() {
    for t in [
        "Order 1000000000000008",
        "Tracking 9000000000000001",
        "1234567890123452",
    ] {
        assert!(texts(&d(), t).is_empty(), "{t}");
    }
}

#[test]
fn rejects_national_ids_and_prices() {
    for t in ["1985031212307", "4,111.11", "4111 1111"] {
        assert!(texts(&d(), t).is_empty(), "{t}");
    }
}
