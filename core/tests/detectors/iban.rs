use super::{run, texts};
use tagging_core::{EntityType, detectors::IbanDetector};

fn d() -> IbanDetector {
    IbanDetector::new().unwrap()
}

#[test]
fn finds_valid_ibans() {
    for x in [
        "LU28 0019 4006 4475 0000",
        "LU280019400644750000",
        "DE89 3704 0044 0532 0130 00",
        "FR14 2004 1010 0505 0001 3M02 606",
        "BE68 5390 0754 7034",
        "NL91 ABNA 0417 1643 00",
        "GB82 WEST 1234 5698 7654 32",
    ] {
        assert_eq!(texts(&d(), &format!("IBAN: {x}.")), [x], "{x}");
    }
}

#[test]
fn stops_at_country_length() {
    assert_eq!(
        texts(&d(), "IBAN LU28 0019 4006 4475 0000 BIC BCEELULL"),
        ["LU28 0019 4006 4475 0000"]
    );
}

#[test]
fn accepts_lowercase() {
    assert_eq!(texts(&d(), "lu28 0019 4006 4475 0000"), ["lu28 0019 4006 4475 0000"]);
    assert_eq!(run(&d(), "LU280019400644750000")[0].kind, EntityType::Iban);
}

#[test]
fn rejects_invalid_checksum() {
    assert!(texts(&d(), "LU28 0019 4006 4475 0001").is_empty());
    assert!(texts(&d(), "DE88 3704 0044 0532 0130 00").is_empty());
}

#[test]
fn rejects_unknown_country_and_wrong_length() {
    for t in ["XX28 0019 4006 4475 0000", "LU28 0019 4006 4475 000", "LU28 0019 4006 4475 00001"] {
        assert!(texts(&d(), t).is_empty(), "{t}");
    }
}
