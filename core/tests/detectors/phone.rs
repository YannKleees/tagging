use super::{run, texts};
use tagging_core::{EntityType, detectors::PhoneDetector};

fn d() -> PhoneDetector {
    PhoneDetector::new().unwrap()
}

fn assert_found(text: &str, phone: &str) {
    assert_eq!(texts(&d(), text), [phone], "{text}");
}

#[test]
fn finds_international_formats() {
    for p in [
        "+352 621 123 456",
        "+352 26 12 34 56",
        "00352 621123456",
        "+33 6 12 34 56 78",
        "+33 (0)6 12 34 56 78",
        "+49 30 12345678",
        "+32 470 12 34 56",
        "+31 6 12345678",
        "+44 20 7946 0958",
        "+352621123456",
    ] {
        assert_found(&format!("Reach me at {p} today."), p);
    }
}

#[test]
fn finds_national_formats() {
    for p in [
        "06 12 34 56 78",
        "06.12.34.56.78",
        "030 12345678",
        "(030) 1234567",
        "0171 1234567",
        "0470 12 34 56",
        "02 123 45 67",
        "06-12345678",
        "020 123 4567",
    ] {
        assert_found(&format!("Call {p} please"), p);
    }
}

#[test]
fn finds_luxembourg_mobile_without_prefix() {
    assert_found("Mäin Handy: 621 123 456", "621 123 456");
    assert_found("schreif op 691123456", "691123456");
}

#[test]
fn finds_luxembourg_landline_only_with_keyword() {
    assert_found("Tel. 26 12 34 56", "26 12 34 56");
    assert_found("Tél.: 45 12 34", "45 12 34");
    assert_found("Telefon 4796-1", "4796-1");
    assert!(texts(&d(), "Zimmer 26 12 34 56").is_empty());
}

#[test]
fn keyword_raises_score() {
    let plain = run(&d(), "0612345678")[0].score;
    let kw = run(&d(), "GSM 0612345678")[0].score;
    assert!(kw > plain);
    assert_eq!(run(&d(), "GSM 0612345678")[0].kind, EntityType::Phone);
}

#[test]
fn finds_two_numbers() {
    assert_eq!(
        texts(&d(), "Tel 06 12 34 56 78, Fax 06 98 76 54 32"),
        ["06 12 34 56 78", "06 98 76 54 32"]
    );
}

#[test]
fn finds_number_after_date() {
    assert_eq!(texts(&d(), "12/01/2023 0612345678"), ["0612345678"]);
}

#[test]
fn rejects_prices() {
    for t in [
        "Total: 1 234 567 €",
        "Price 621 123 456 EUR",
        "€ 0612345678",
        "1.250,00",
    ] {
        assert!(texts(&d(), t).is_empty(), "{t}");
    }
}

#[test]
fn rejects_order_and_invoice_numbers() {
    for t in [
        "Order no. 0612345678",
        "Invoice 2023-000123",
        "Rechnungsnummer 0301234567",
        "Facture n° 0612345678",
        "Ref: 0470123456",
    ] {
        assert!(texts(&d(), t).is_empty(), "{t}");
    }
}

#[test]
fn rejects_dates_versions_and_ids() {
    for t in [
        "01.03.1985",
        "12/03/2023",
        "Version 1.2.3",
        "1985031212307",
        "0.612345678",
        "2023-03-12",
    ] {
        assert!(texts(&d(), t).is_empty(), "{t}");
    }
}
