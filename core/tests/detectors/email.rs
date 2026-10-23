use super::{run, texts};
use tagging_core::{EntityType, Source, detectors::EmailDetector};

fn d() -> EmailDetector {
    EmailDetector::new().unwrap()
}

#[test]
fn finds_plain_address() {
    assert_eq!(texts(&d(), "Mail: jhemp.schmit@pt.lu"), ["jhemp.schmit@pt.lu"]);
}

#[test]
fn excludes_sentence_final_period() {
    assert_eq!(texts(&d(), "Write to mett@example.com."), ["mett@example.com"]);
}

#[test]
fn keeps_plus_tags_and_subdomains() {
    assert_eq!(texts(&d(), "<a.b+news@mail.uni.lu>"), ["a.b+news@mail.uni.lu"]);
}

#[test]
fn accepts_accented_local_part() {
    assert_eq!(texts(&d(), "méck@beispill.lu"), ["méck@beispill.lu"]);
}

#[test]
fn reports_byte_offsets_into_original() {
    let t = "Méck: méck@beispill.lu";
    let s = &run(&d(), t)[0];
    assert_eq!((s.start, s.end), (7, t.len()));
    assert_eq!(&t[s.start..s.end], "méck@beispill.lu");
    assert_eq!((s.kind, s.source), (EntityType::Email, Source::Rule));
}

#[test]
fn finds_several() {
    assert_eq!(texts(&d(), "a@x.lu, b@y.fr; c@z.de"), ["a@x.lu", "b@y.fr", "c@z.de"]);
}

#[test]
fn rejects_non_addresses() {
    for t in ["user@localhost", "@handle", "a@b.c", "price 5@3.50", "version@1.2", "foo@bar", "x@.lu"] {
        assert!(texts(&d(), t).is_empty(), "{t}");
    }
}
