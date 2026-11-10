use super::{run, texts};
use tagging_core::{EntityType, gazetteer::Gazetteer};

fn g() -> Gazetteer {
    Gazetteer::builtin()
}

fn kinds(text: &str) -> Vec<(String, EntityType)> {
    run(&g(), text)
        .into_iter()
        .filter(|s| s.score >= 0.5)
        .map(|s| (s.text, s.kind))
        .collect()
}

fn found(text: &str) -> Vec<String> {
    kinds(text).into_iter().map(|(t, _)| t).collect()
}

#[test]
fn first_name_takes_a_listed_surname() {
    assert_eq!(
        found("ech sinn de Jhemp Kieffer aus Ettelbréck"),
        ["Jhemp Kieffer", "Ettelbréck"]
    );
    assert_eq!(
        found("Jean-Claude Juncker a Xavier Bettel"),
        ["Jean-Claude Juncker", "Xavier Bettel"]
    );
    assert_eq!(found("Marie de Kieffer huet ugeruff"), ["Marie de Kieffer"]);
    assert_eq!(found("Rachel O'Donnell lives here"), ["Rachel O'Donnell"]);
}

#[test]
fn first_name_takes_an_unlisted_surname_with_a_lower_score() {
    let listed = &run(&g(), "Jhemp Kieffer")[0];
    let guessed = &run(&g(), "Jhemp Zzyzxer")[0];
    assert_eq!(guessed.text, "Jhemp Zzyzxer");
    assert!(guessed.score < listed.score);
    // A weekday, a listed place or an acronym is not a surname.
    assert_eq!(found("Jhemp Méindeg"), ["Jhemp"]);
    assert_eq!(found("Jhemp Remich"), ["Jhemp", "Remich"]);
    assert_eq!(found("Charel CHL"), ["Charel", "CHL"]);
}

#[test]
fn title_marks_the_name_but_is_not_part_of_it() {
    assert_eq!(found("den Här Feltgen vun der CNS"), ["Feltgen", "CNS"]);
    assert_eq!(found("D'Doktesch Ries an den Här Weber"), ["Ries", "Weber"]);
    assert_eq!(found("Dr. Zzyzxer and Mrs Qwertz"), ["Zzyzxer", "Qwertz"]);
    assert_eq!(
        found("M. Dupont et Mme Marie-Josée Reuter"),
        ["Dupont", "Marie-Josée Reuter"]
    );
    assert_eq!(found("Herr Direktor Thill"), ["Thill"]);
    assert!(found("Sehr geehrter Herr, wir danken").is_empty());
}

#[test]
fn a_title_does_not_reach_across_a_sentence() {
    assert_eq!(found("Ask Mr President. Then we leave."), Vec::<String>::new());
}

#[test]
fn surname_in_capitals_comes_first_on_forms() {
    assert_eq!(found("KIEFFER Jean, Matricule"), ["KIEFFER Jean"]);
}

#[test]
fn everyday_words_need_a_second_name_part() {
    // `Rose` and `Will` are first names and everyday words.
    assert!(found("The Rose is red. Will you come?").is_empty());
    assert_eq!(found("Rose Schmit called"), ["Rose Schmit"]);
    assert_eq!(found("Mrs Rose called"), ["Rose"]);
    // Both parts everyday words proves nothing.
    assert!(found("May Day is a holiday").is_empty());
}

#[test]
fn a_name_that_is_also_a_place_stays_a_place() {
    assert_eq!(
        kinds("He flew to Paris."),
        [("Paris".to_string(), EntityType::Location)]
    );
    assert_eq!(found("Olivia Gardiner in Berlin"), ["Olivia Gardiner", "Berlin"]);
    assert_eq!(found("Jean de Luxembourg"), ["Jean", "Luxembourg"]);
}

#[test]
fn indefinite_article_makes_an_ambiguous_name_a_thing() {
    assert!(found("Ech hunn eng Méck gesinn an e Kilo Mett kaf.").is_empty());
    assert_eq!(found("De Mett an d'Lis kommen muer."), ["Mett", "Lis"]);
    assert!(found("Wir sehen einen Berg.").is_empty());
}

#[test]
fn cue_word_lifts_an_ambiguous_place_or_organization() {
    let cued = &run(&g(), "Ich arbeite bei Orange")[0];
    assert_eq!((cued.text.as_str(), cued.kind), ("Orange", EntityType::Organization));
    assert!(cued.score >= 0.6);
    assert!(found("Eine Orange und ein Apfel").is_empty());
    assert_eq!(found("from Paris to Nice"), ["Paris", "Nice"]);
    assert!(found("Nice to meet you").is_empty());
}

#[test]
fn lowercase_names_count_in_text_without_capitals() {
    assert_eq!(found("ech war gëschter mam jhemp zu remich"), ["jhemp", "remich"]);
    // Ambiguous names stay out, and so does a lowercase name in normally written text.
    assert!(found("mir iessen haut mett mat brout").is_empty());
    assert!(found("Ech war zu remich.").is_empty());
}

#[test]
fn business_named_after_a_person_or_place() {
    assert_eq!(
        kinds("Garage Weis a Brasserie Schuman sinn zou"),
        [
            ("Garage Weis".to_string(), EntityType::Organization),
            ("Brasserie Schuman".to_string(), EntityType::Organization)
        ]
    );
    assert_eq!(found("am Café de Paris"), ["Café de Paris"]);
    // `Zimmer` is a surname and a room.
    assert!(found("Hotel Zimmer frei").is_empty());
    assert!(found("in der Garage steht das Auto").is_empty());
}

#[test]
fn company_form_marks_an_organization() {
    assert_eq!(
        texts(&g(), "Muller & Fils S.à r.l. an Arendt GmbH"),
        ["Muller & Fils S.à r.l.", "Arendt GmbH"]
    );
    assert_eq!(found("bei Zzyzx S.A. zu Lëtzebuerg"), ["Zzyzx S.A.", "Lëtzebuerg"]);
    assert_eq!(found("d'Caritas asbl"), ["Caritas asbl"]);
    // Lowercase `sa` and `se` are French words, `EV` an electric vehicle.
    assert!(found("Marie et sa voiture, Zzyzx EV").iter().all(|t| t == "Marie"));
}
