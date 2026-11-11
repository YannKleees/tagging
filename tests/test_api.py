import json

import pytest

import tagging

TEXT = "De Jhemp (jhemp@pt.lu) wunnt zu Réimech."


def test_detect_returns_spans_with_byte_offsets():
    spans = tagging.detect(TEXT)
    assert [(s["type"], s["text"], s["source"]) for s in spans] == [
        ("person", "Jhemp", "gazetteer"),
        ("email", "jhemp@pt.lu", "rule"),
        ("location", "Réimech", "gazetteer"),
    ]
    raw = TEXT.encode()
    assert all(raw[s["start"] : s["end"]].decode() == s["text"] for s in spans)
    assert set(spans[0]) == {"type", "start", "end", "text", "score", "source"}


def test_detect_accepts_config_dict():
    spans = tagging.detect(TEXT, {"detectors": ["email"]})
    assert [s["type"] for s in spans] == ["email"]


def test_detect_accepts_config_file(tmp_path):
    (tmp_path / "names.txt").write_text("Kieffer\n", encoding="utf-8")
    cfg = {
        "builtin_gazetteers": False,
        "detectors": ["gazetteer"],
        "gazetteers": [{"path": "names.txt", "type": "person"}],
    }
    path = tmp_path / "config.json"
    path.write_text(json.dumps(cfg), encoding="utf-8")
    assert [s["text"] for s in tagging.detect("den Här Kieffer zu Remich", path)] == ["Kieffer"]


def test_taggers_are_cached_per_config():
    assert tagging.tagger({"threshold": 0.3}) is tagging.tagger({"threshold": 0.3})
    assert tagging.tagger() is not tagging.tagger({"threshold": 0.3})


def test_bad_config_raises_value_error():
    with pytest.raises(ValueError, match="invalid config"):
        tagging.detect(TEXT, {"detectors": ["ssn"]})
    with pytest.raises(FileNotFoundError):
        tagging.detect(TEXT, "missing.json")


def test_redact_modes():
    spans = tagging.detect(TEXT)
    assert tagging.redact(TEXT, spans) == "De [REDACTED] ([REDACTED]) wunnt zu [REDACTED]."
    assert tagging.redact(TEXT, spans, "pseudo") == "De [PERSON_1] ([EMAIL_1]) wunnt zu [LOCATION_1]."


def test_redact_accepts_bare_spans():
    assert tagging.redact("ab cd", [{"type": "person", "start": 3, "end": 5}], "pseudo") == "ab [PERSON_1]"


def test_redact_rejects_bad_input():
    with pytest.raises(ValueError, match="overlap"):
        tagging.redact("abcdef", [{"type": "person", "start": 0, "end": 4}, {"type": "person", "start": 2, "end": 6}])
    with pytest.raises(ValueError):
        tagging.redact("abc", [{"type": "person", "start": 0, "end": 2}], "blur")


def test_detect_finds_full_names_addresses_and_companies():
    text = "Den Här Feltgen an d'Marie Weber vun Arendt GmbH wunnen 12, rue de la Gare, L-1611 Lëtzebuerg."
    assert [(s["type"], s["text"]) for s in tagging.detect(text)] == [
        ("person", "Feltgen"),
        ("person", "Marie Weber"),
        ("organization", "Arendt GmbH"),
        ("address", "12, rue de la Gare"),
        ("address", "L-1611"),
        ("location", "Lëtzebuerg"),
    ]


def test_everyday_text_has_no_hits():
    for text in [
        "The Plumber fixed the Sink in the Bathroom before lunch.",
        "Die Feuerwehr war in wenigen Minuten beim Lagerhaus.",
        "De Kellner huet d'Zopp an d'Brout op den Dësch bruecht.",
        "Le Juge a écouté le Témoin et l'Avocat.",
    ]:
        assert tagging.detect(text) == [], text


def test_detect_reads_form_fields_and_foreign_ids():
    text = 'Reisepass: C4FH7K2P1\nPasswort: V+B9o-ctO\n"Geburtsdatum": "juin/55"\nNIR 2 55 08 14 168 025 38'
    assert [(s["type"], s["text"]) for s in tagging.detect(text)] == [
        ("id_number", "C4FH7K2P1"),
        ("credential", "V+B9o-ctO"),
        ("date_of_birth", "juin/55"),
        ("id_number", "2 55 08 14 168 025 38"),
    ]
    redacted = tagging.redact(text, tagging.detect(text), "pseudo")
    assert "[ID_NUMBER_1]" in redacted and "[CREDENTIAL_1]" in redacted and "[ID_NUMBER_2]" in redacted


def test_dates_and_times_are_optional():
    text = "Termin am 14.11.2024 um 14:30 in Trier."
    assert [s["type"] for s in tagging.detect(text)] == ["location"]
    spans = tagging.detect(text, {"dates_and_times": True})
    assert [(s["type"], s["text"]) for s in spans] == [("date", "14.11.2024"), ("date", "14:30"), ("location", "Trier")]
