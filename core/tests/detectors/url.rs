use super::{run, texts};
use tagging_core::{EntityType, detectors::UrlDetector};

fn d() -> UrlDetector {
    UrlDetector::new().unwrap()
}

#[test]
fn finds_http_url_without_trailing_punctuation() {
    assert_eq!(
        texts(&d(), "See https://www.cfl.lu/fr/horaires?from=Remich&x=1."),
        ["https://www.cfl.lu/fr/horaires?from=Remich&x=1"]
    );
}

#[test]
fn finds_www_url() {
    assert_eq!(texts(&d(), "Visit www.post.lu, then call."), ["www.post.lu"]);
}

#[test]
fn drops_unbalanced_closing_paren() {
    assert_eq!(
        texts(&d(), "(see https://guichet.public.lu/en)"),
        ["https://guichet.public.lu/en"]
    );
}

#[test]
fn keeps_balanced_parens() {
    let u = "https://en.wikipedia.org/wiki/Remich_(Luxembourg)";
    assert_eq!(texts(&d(), &format!("Read {u}.")), [u]);
}

#[test]
fn tags_type() {
    assert_eq!(run(&d(), "http://x.lu")[0].kind, EntityType::Url);
}

#[test]
fn rejects_bare_domains_and_decimals() {
    for t in ["post.lu", "3.14", "file.txt", "e.g. this", "https://", "www."] {
        assert!(texts(&d(), t).is_empty(), "{t}");
    }
}
