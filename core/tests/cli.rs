use std::path::PathBuf;
use std::process::{Command, Output};

fn sample(name: &str, text: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("cli_test");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(name);
    std::fs::write(&path, text).unwrap();
    path
}

fn tagging(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_tagging")).args(args).output().unwrap()
}

fn stdout(out: &Output) -> String {
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8(out.stdout.clone()).unwrap()
}

#[test]
fn detect_prints_json_spans() {
    let f = sample("json.txt", "Mail jhemp@pt.lu zu Réimech");
    let out = stdout(&tagging(&["detect", f.to_str().unwrap(), "--format", "json"]));
    let spans: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(spans[0]["type"], "email");
    assert_eq!(spans[0]["start"], 5);
    assert_eq!(spans[0]["end"], 16);
    assert_eq!(spans[0]["source"], "rule");
    assert_eq!(spans[1]["text"], "Réimech");
    assert_eq!(spans[1]["source"], "gazetteer");
}

#[test]
fn detect_prints_csv() {
    let f = sample("csv.txt", "Mail jhemp@pt.lu zu Réimech");
    let out = stdout(&tagging(&["detect", f.to_str().unwrap(), "--format", "csv"]));
    assert_eq!(
        out.lines().collect::<Vec<_>>(),
        ["tag,start,end", "email,5,16", "location,20,28"]
    );
}

#[test]
fn redact_pseudonymizes_consistently() {
    let f = sample("pseudo.txt", "Jhemp schreift jhemp@pt.lu. Jhemp wunnt zu Remich.");
    let out = stdout(&tagging(&["redact", f.to_str().unwrap(), "--mode", "pseudo"]));
    assert_eq!(out, "[PERSON_1] schreift [EMAIL_1]. [PERSON_1] wunnt zu [LOCATION_1].");
}

#[test]
fn redact_masks_by_default() {
    let f = sample("mask.txt", "Call +352 621 123 456");
    assert_eq!(stdout(&tagging(&["redact", f.to_str().unwrap()])), "Call [REDACTED]");
}

#[test]
fn uses_config_file() {
    let f = sample("cfg_input.txt", "Mail jhemp@pt.lu zu Réimech");
    let cfg = sample("cfg.json", r#"{"detectors": ["gazetteer"]}"#);
    let out = stdout(&tagging(&[
        "detect",
        f.to_str().unwrap(),
        "--format",
        "csv",
        "--config",
        cfg.to_str().unwrap(),
    ]));
    assert_eq!(out.lines().collect::<Vec<_>>(), ["tag,start,end", "location,20,28"]);
}

#[test]
fn fails_cleanly_on_missing_file() {
    let out = tagging(&["detect", "no/such/file.txt"]);
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("no/such/file.txt"));
}
