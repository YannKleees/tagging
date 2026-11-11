use super::{run, texts};
use tagging_core::{EntityType, detectors::IdNumberDetector};

fn d() -> IdNumberDetector {
    IdNumberDetector::new().unwrap()
}

#[test]
fn finds_national_numbers_of_the_neighbours_by_check_digit() {
    for (text, id) in [
        ("NIR 2 55 08 14 168 025 38, merci", "2 55 08 14 168 025 38"),
        ("numéro 255081416802538.", "255081416802538"),
        ("Rijksregisternummer 85.03.12-123.62", "85.03.12-123.62"),
        ("Versicherungsnummer: 65 170839 J 00 3", "65 170839 J 00 3"),
        ("AHV 756.1234.5678.97", "756.1234.5678.97"),
    ] {
        let s = run(&d(), text);
        assert_eq!(s.len(), 1, "{text}");
        assert_eq!((s[0].text.as_str(), s[0].kind), (id, EntityType::IdNumber), "{text}");
        assert!(s[0].score >= 0.85, "{text}");
    }
}

#[test]
fn rejects_numbers_with_a_wrong_check_digit() {
    for text in [
        "2 55 08 14 168 025 39",
        "85.03.12-123.63",
        "65 170839 J 00 4",
        "12 345 678 901",
    ] {
        assert!(texts(&d(), text).is_empty(), "{text}");
    }
}

#[test]
fn context_raises_weak_formats() {
    let tax = "86 095 742 719";
    assert!(run(&d(), tax)[0].score < 0.6);
    assert!(run(&d(), &format!("Steuer-ID: {tax}"))[0].score >= 0.9);
    let ssn = "536-90-4399";
    assert!(run(&d(), ssn)[0].score < run(&d(), &format!("SSN {ssn}"))[0].score);
    assert!(texts(&d(), "000-12-3456").is_empty());
}

#[test]
fn finds_codes_shaped_like_document_numbers() {
    assert_eq!(texts(&d(), "Kommentar von P55790658: Danke"), ["P55790658"]);
    assert_eq!(texts(&d(), "Reisepass C85GR6998227, gültig"), ["C85GR6998227"]);
    assert_eq!(texts(&d(), "licence D8.25.FAWG3W4.5."), ["D8.25.FAWG3W4.5"]);
    assert!(run(&d(), "Passport KP2212837")[0].score > run(&d(), "KP2212837")[0].score);
}

#[test]
fn leaves_product_codes_order_numbers_and_plain_numbers() {
    for text in [
        "Bestellung AB12345678 ist unterwegs",
        "Invoice INV20240001X",
        "ISO9001 and an A320 with WINDOWS2000",
        "version 2024.11.05 of build 123456789",
        "see https://example.lu/AB12345678 or AB12345678@example.lu",
        "192.168.178.123 and 2a02:8388:1234:5678::1",
        "on 2024-11-05 at 14:30:00",
    ] {
        assert!(texts(&d(), text).is_empty(), "{text}");
    }
}
