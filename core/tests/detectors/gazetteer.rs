use super::{run, texts};
use tagging_core::{EntityType, Source, gazetteer::Gazetteer};

fn builtin() -> Gazetteer {
    Gazetteer::builtin()
}

fn kinds(text: &str) -> Vec<(String, EntityType)> {
    run(&builtin(), text).into_iter().map(|s| (s.text, s.kind)).collect()
}

#[test]
fn finds_places_in_all_spellings() {
    for p in [
        "Réimech",
        "Remich",
        "Ettelbréck",
        "Ettelbruck",
        "Ettelbrück",
        "Iernz",
        "Ernzen",
        "Lëtzebuerg",
    ] {
        assert_eq!(
            kinds(&format!("Mir fueren op {p}.")),
            [(p.to_string(), EntityType::Location)],
            "{p}"
        );
    }
}

#[test]
fn matches_multi_token_names() {
    assert_eq!(texts(&builtin(), "in Esch an der Alzette"), ["Esch an der Alzette"]);
    assert_eq!(texts(&builtin(), "zu Esch-Uelzecht"), ["Esch-Uelzecht"]);
}

#[test]
fn is_case_insensitive() {
    assert_eq!(texts(&builtin(), "RÉIMECH"), ["RÉIMECH"]);
    let s = &run(&builtin(), "ech war zu remich")[0];
    assert_eq!(s.text, "remich");
    assert!(s.score < 0.5);
}

#[test]
fn respects_token_boundaries() {
    for t in ["Remicher Wäin", "Ettelbrécker", "Spuerkeessen", "XCFL"] {
        assert!(texts(&builtin(), t).is_empty(), "{t}");
    }
}

#[test]
fn finds_short_names_after_articles_and_elision() {
    assert_eq!(
        kinds("D'Mett an de Méck"),
        [("Mett".into(), EntityType::Person), ("Méck".into(), EntityType::Person)]
    );
    assert_eq!(texts(&builtin(), "z'Lëtzebuerg"), ["Lëtzebuerg"]);
    assert_eq!(texts(&builtin(), "Jhemp, Pol a Fränz"), ["Jhemp", "Pol", "Fränz"]);
}

#[test]
fn ambiguous_entries_score_lower() {
    let jhemp = run(&builtin(), "Jhemp")[0].score;
    let mett = run(&builtin(), "Mett")[0].score;
    assert!(mett < jhemp);
    assert!(mett >= 0.5);
}

#[test]
fn finds_organizations() {
    for o in [
        "Spuerkeess",
        "BCEE",
        "CFL",
        "POST",
        "CHL",
        "Banque et Caisse d'Epargne de l'Etat",
        "Université du Luxembourg",
    ] {
        assert_eq!(
            kinds(&format!("bei der {o} schaffen")),
            [(o.to_string(), EntityType::Organization)],
            "{o}"
        );
    }
}

#[test]
fn acronyms_match_exact_case_only() {
    for t in ["the post office", "Post", "cfl", "Chl"] {
        assert!(texts(&builtin(), t).is_empty(), "{t}");
    }
}

#[test]
fn tags_source() {
    assert_eq!(run(&builtin(), "Remich")[0].source, Source::Gazetteer);
}

#[test]
fn loads_custom_lists() {
    let mut g = Gazetteer::new();
    g.add_list("# comment\nKieffer|Kiefer\n~Schmit\n", EntityType::Person);
    g.add_words(&["Acme Sàrl"], EntityType::Organization);
    assert_eq!(texts(&g, "den Här Kieffer an d'Acme Sàrl"), ["Kieffer", "Acme Sàrl"]);
    assert!(run(&g, "Schmit")[0].score < run(&g, "Kiefer")[0].score);
}
