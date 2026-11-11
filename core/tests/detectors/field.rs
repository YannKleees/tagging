use super::run;
use tagging_core::{EntityType, detectors::FieldDetector};

fn found(text: &str) -> Vec<(String, EntityType)> {
    let d = FieldDetector::new(false).unwrap();
    run(&d, text).into_iter().map(|s| (s.text, s.kind)).collect()
}

fn one(text: &str, value: &str, kind: EntityType) {
    assert_eq!(found(text), [(value.to_string(), kind)], "{text}");
}

#[test]
fn reads_plain_markdown_json_and_xml_fields() {
    one("Passport: K6974618", "K6974618", EntityType::IdNumber);
    one("- **Reisepass:** P97T4103I  ", "P97T4103I", EntityType::IdNumber);
    one(
        "<li><strong>Passeport : </strong>I8114435</li>",
        "I8114435",
        EntityType::IdNumber,
    );
    one(r#""Driver_License": "4220436553","#, "4220436553", EntityType::IdNumber);
    one(
        "<Username>alex.seran45</Username>",
        "alex.seran45",
        EntityType::Credential,
    );
    one("  id_card: QW63852ZQ", "QW63852ZQ", EntityType::IdNumber);
    one("Carte d'identité n°: Y6879245", "Y6879245", EntityType::IdNumber);
}

#[test]
fn reads_names_places_and_address_parts() {
    one("Vorname: Rilon", "Rilon", EntityType::Person);
    one(
        r#""student_name": "Teklemichael Malin Akrong","#,
        "Teklemichael Malin Akrong",
        EntityType::Person,
    );
    one("Nom de famille: Gözelel", "Gözelel", EntityType::Person);
    one("Ville: La Bloutière", "La Bloutière", EntityType::Location);
    one("Country: United Kingdom", "United Kingdom", EntityType::Location);
    one("Code postal: 38770", "38770", EntityType::Address);
    one("Rue: Grossweid", "Grossweid", EntityType::Address);
    one("Date of Birth: juin/55", "juin/55", EntityType::DateOfBirth);
    one("Telefon: 07 9108 5282", "07 9108 5282", EntityType::Phone);
}

#[test]
fn a_password_keeps_its_punctuation() {
    one(r#""Password": "F3o]4","#, "F3o]4", EntityType::Credential);
    one("Passwort: V+B9o-ctO", "V+B9o-ctO", EntityType::Credential);
    one("<password>23EI=f</password>", "23EI=f", EntityType::Credential);
    // German `Pass` is a passport.
    one("Pass: C4FH7K2P1", "C4FH7K2P1", EntityType::IdNumber);
}

#[test]
fn a_value_ends_with_its_field() {
    one("Name: Kieffer, geboren in Remich", "Kieffer", EntityType::Person);
    one("Nom: Dupont | Tél: -", "Dupont", EntityType::Person);
    one("City: Trier - Country: -", "Trier", EntityType::Location);
}

#[test]
fn ignores_words_that_only_contain_a_field_name() {
    for text in [
        "Deutschland: 3 Punkte",
        "Report: Quarterly figures",
        "hostname: srv01.example.lu",
        "Company name: Acme",
        "file_name: Bericht",
        "The name of the game",
        "Passport office hours",
        "Status: active",
        "Name: 12 Stück",
        "Date: 12.03.2024",
        "Update: Meeting moved",
    ] {
        assert!(found(text).is_empty(), "{text}");
    }
}

#[test]
fn dates_and_times_are_read_on_request() {
    let d = FieldDetector::new(true).unwrap();
    let s = run(&d, "Date: août/59\nHeure: 9h");
    assert_eq!(s.iter().map(|s| s.text.as_str()).collect::<Vec<_>>(), ["août/59", "9h"]);
}
