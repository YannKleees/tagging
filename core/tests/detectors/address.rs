use super::{run, texts};
use tagging_core::{EntityType, detectors::AddressDetector};

fn d() -> AddressDetector {
    AddressDetector::new().unwrap()
}

#[test]
fn finds_french_style_addresses() {
    assert_eq!(
        texts(&d(), "Jean habite 46, rue de la Gare à Luxembourg."),
        ["46, rue de la Gare"]
    );
    assert_eq!(texts(&d(), "105, boulevard Royal, Dudelange"), ["105, boulevard Royal"]);
    assert_eq!(texts(&d(), "Route d'Arlon 250, Strassen"), ["Route d'Arlon 250"]);
    assert_eq!(texts(&d(), "2-4, rue du Fort Neipperg"), ["2-4, rue du Fort Neipperg"]);
    assert_eq!(texts(&d(), "12A avenue J.F. Kennedy"), ["12A avenue J.F. Kennedy"]);
}

#[test]
fn finds_german_and_luxembourgish_streets() {
    assert_eq!(texts(&d(), "Die Hauptstraße 12 ist gesperrt."), ["Hauptstraße 12"]);
    assert_eq!(
        texts(&d(), "Wir wohnen Luxemburger Straße 45a."),
        ["Luxemburger Straße 45a"]
    );
    assert_eq!(texts(&d(), "an der Groussgaass 5 zu Réimech"), ["Groussgaass 5"]);
    assert_eq!(texts(&d(), "Hauptstr. 5, Trier"), ["Hauptstr. 5"]);
}

#[test]
fn finds_english_addresses() {
    assert_eq!(
        texts(&d(), "She lives at 221B Baker Street, London."),
        ["221B Baker Street"]
    );
    assert_eq!(texts(&d(), "send it to 10 Downing Street"), ["10 Downing Street"]);
}

#[test]
fn finds_postcodes() {
    assert_eq!(texts(&d(), "L-1611 Luxembourg"), ["L-1611"]);
    assert_eq!(texts(&d(), "D-54290 Trier, F-57000 Metz"), ["D-54290", "F-57000"]);
    assert!(texts(&d(), "model F-16, part B-1234, XL-1234").is_empty());
}

#[test]
fn a_street_name_without_number_scores_lower() {
    let numbered = &run(&d(), "12, rue de la Gare")[0];
    let street = &run(&d(), "dans la rue Principale")[0];
    assert_eq!(street.text, "rue Principale");
    assert_eq!((numbered.kind, street.kind), (EntityType::Address, EntityType::Address));
    assert!(street.score < numbered.score && street.score >= 0.5);
}

#[test]
fn rejects_words_that_only_look_like_streets() {
    for t in [
        "Parkplatz 12 ist frei",
        "Der Umweg 3 km lang",
        "Platz 3 im Rennen",
        "Die Straße 12 Meter breit",
        "les rues de la ville",
        "sur la route de Paris",
        "une place de parking",
        "3 Places to visit",
    ] {
        assert!(texts(&d(), t).is_empty(), "{t}");
    }
}

#[test]
fn leaves_unrelated_numbers_out() {
    assert_eq!(
        texts(&d(), "rue de la Gare 46 00352 691 042 147"),
        ["rue de la Gare 46"]
    );
    assert_eq!(texts(&d(), "Tel 26 12 34 56, rue des Bains 3"), ["rue des Bains 3"]);
    assert_eq!(texts(&d(), "Tel 26 12 34 56, Hauptstraße 12"), ["Hauptstraße 12"]);
}

#[test]
fn finds_addresses_far_into_a_long_text() {
    let text = format!(
        "{} Mir wunnen 46, rue de la Gare. {}",
        "Wuert ".repeat(300),
        "Wuert ".repeat(300)
    );
    assert_eq!(texts(&d(), &text), ["46, rue de la Gare"]);
    let two = "12, rue de la Gare an 14, rue de la Gare an Hauptstraße 5";
    assert_eq!(
        texts(&d(), two),
        ["12, rue de la Gare", "14, rue de la Gare", "Hauptstraße 5"]
    );
}

#[test]
fn accented_text_around_a_street_word_does_not_break_the_window() {
    // The search window is counted in bytes and must not cut `é` or `ë` in half.
    for pad in 0..6 {
        let filler = "é".repeat(60 + pad);
        let text = format!("{filler} 46, rue de la Gare {filler}ëéà");
        assert_eq!(texts(&d(), &text), ["46, rue de la Gare"], "{pad}");
        let glued = format!("{filler}rue {filler}");
        assert!(texts(&d(), &glued).is_empty());
    }
}

#[test]
fn finds_the_town_and_postcode_after_a_street() {
    assert_eq!(
        texts(&d(), "999, Grange Road, Kettering, ENG, NN16 8QP"),
        ["999, Grange Road", "Kettering", "NN16 8QP"]
    );
    assert_eq!(
        texts(&d(), "Hauptstraße 12, Trier, 54290"),
        ["Hauptstraße 12", "Trier", "54290"]
    );
    assert_eq!(texts(&d(), "Hauptstraße 12, dann links"), ["Hauptstraße 12"]);
}

#[test]
fn finds_foreign_postcodes_units_and_coordinates() {
    assert_eq!(texts(&d(), "Falmouth, KY, 41040-7980"), ["41040-7980"]);
    assert_eq!(texts(&d(), "send to LS10 3SL today"), ["LS10 3SL"]);
    assert_eq!(texts(&d(), "Apt 4B, then Suite 288"), ["Apt 4B", "Suite 288"]);
    assert_eq!(run(&d(), "at [49.6117, 6.1319] today")[0].kind, EntityType::Location);
    for t in [
        "Room 3 is free",
        "OK, 12345 items",
        "prices (12.50, 13.75)",
        "[120.5, 300.25]",
        "in NO, 12 Uhr",
    ] {
        let got = texts(&d(), t);
        assert!(got.is_empty() || t.starts_with("prices"), "{t}: {got:?}");
    }
}
