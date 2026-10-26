import re

import tagging
from tagging import ner
from tagging.merge import merge
from tagging.postprocess import postprocess


class FakeModel:
    def __init__(self, entities):
        self.entities = entities
        self.calls = 0

    def predict_entities(self, text, labels, threshold=0.5):
        self.calls += 1
        out = []
        for name, label in self.entities.items():
            for m in re.finditer(re.escape(name), text):
                out.append({"start": m.start(), "end": m.end(), "text": name, "label": label, "score": 0.9})
        return out


def span(text, sub, type, source, score=0.9):
    start = text.encode().index(sub.encode())
    return {
        "type": type,
        "start": start,
        "end": start + len(sub.encode()),
        "text": sub,
        "score": score,
        "source": source,
    }


def char_span(text, sub, type="person"):
    start = text.index(sub)
    return {"type": type, "start": start, "end": start + len(sub), "text": sub, "score": 0.9, "source": "ner"}


def test_windows_overlap_and_cover_all_words():
    text = " ".join(f"w{i}" for i in range(10))
    spans = list(ner.windows(text, 4, 1))
    assert spans[0][0] == 0 and spans[-1][1] == len(text)
    assert len(spans) == 3
    assert all(a < prev_end for (a, _), (_, prev_end) in zip(spans[1:], spans))


def test_predict_maps_chunk_offsets_and_dedupes():
    text = "filler " * 30 + "Jhemp Schmit wunnt zu Réimech. " + "filler " * 30
    model = FakeModel({"Jhemp Schmit": "person name", "Réimech": "city or place name"})
    spans = ner.predict(text, model, size=40, overlap=20)
    assert model.calls > 1
    assert [(s["type"], s["text"]) for s in spans] == [("person", "Jhemp Schmit"), ("location", "Réimech")]
    assert all(text[s["start"] : s["end"]] == s["text"] for s in spans)


def test_to_bytes_handles_multibyte_text():
    text = "Méck, Ettelbréck an d'Lis"
    spans = ner.to_bytes(text, [char_span(text, "Lis")])
    assert text.encode()[spans[0]["start"] : spans[0]["end"]].decode() == "Lis"


def test_postprocess_strips_articles_and_titles():
    for text, phrase, name in [
        ("Gëschter huet den Här Kieffer ugeruff.", "den Här Kieffer", "Kieffer"),
        ("D'Madame Weber ass do.", "D'Madame Weber", "Weber"),
        ("Dr. Welter kommt.", "Dr. Welter", "Welter"),
        ("Le docteur Hansen arrive.", "Le docteur Hansen", "Hansen"),
        ("De Mett ass do.", "De Mett", "Mett"),
    ]:
        assert [s["text"] for s in postprocess(text, [char_span(text, phrase)])] == [name], text


def test_postprocess_splits_coordinated_names():
    for text, phrase in [
        ("D'Mett an de Méck kommen.", "D'Mett an de Méck"),
        ("Mett und Méck kommen.", "Mett und Méck"),
        ("Mett et Méck viennent.", "Mett et Méck"),
    ]:
        assert [s["text"] for s in postprocess(text, [char_span(text, phrase)])] == ["Mett", "Méck"], text


def test_postprocess_drops_kinship_and_role_spans():
    for text, phrase, type in [
        ("Die Mutter kommt.", "Die Mutter", "person"),
        ("Meng Mamm an de Bopa kommen.", "Meng Mamm an de Bopa", "person"),
        ("La mère du voisin.", "La mère", "person"),
        ("My neighbour called.", "My neighbour", "person"),
        ("Den Direkter ass do.", "Den Direkter", "organization"),
        ("Madame est là.", "Madame", "person"),
    ]:
        assert postprocess(text, [char_span(text, phrase, type)]) == [], text


def test_merge_rules_win_over_ner():
    text = "Mail jhemp@pt.lu now"
    rules = [span(text, "jhemp@pt.lu", "email", "rule", 0.99)]
    assert merge(text, rules, [span(text, "jhemp@pt.lu", "person", "ner")]) == rules


def test_merge_unions_same_type_gazetteer_and_ner():
    text = "De Jhemp Schmit kommt"
    out = merge(text, [span(text, "Jhemp", "person", "gazetteer", 0.85)], [span(text, "Jhemp Schmit", "person", "ner")])
    assert [(s["text"], s["source"]) for s in out] == [("Jhemp Schmit", "gazetteer+ner")]


def test_merge_requires_ner_support_for_ambiguous_gazetteer_hits():
    text = "Mett mat Ënnen"
    gaz = [span(text, "Mett", "person", "gazetteer", 0.55)]
    assert merge(text, gaz, []) == []
    assert merge(text, gaz, [], confirm_ambiguous=False) == gaz
    assert len(merge(text, gaz, [span(text, "Mett", "person", "ner")])) == 1


def test_merge_prefers_gazetteer_across_types_unless_ner_contains_it():
    text = "12, rue de la Gare zu Remich"
    gare = span(text, "Gare", "location", "gazetteer", 0.85)
    address = span(text, "12, rue de la Gare", "address", "ner")
    assert [s["type"] for s in merge(text, [gare], [address])] == ["address"]
    remich = span(text, "Remich", "location", "gazetteer", 0.85)
    assert [s["type"] for s in merge(text, [remich], [span(text, "Remich", "organization", "ner")])] == ["location"]


def test_detect_with_ner_end_to_end(monkeypatch):
    model = FakeModel({"den Här Kieffer": "person name", "12, rue de la Gare": "street address"})
    monkeypatch.setattr(ner, "load", lambda name, device: model)
    text = "Ech hunn den Här Kieffer gesinn, hien wunnt 12, rue de la Gare zu Réimech."
    spans = tagging.detect(text, {"ner": True})
    assert [(s["type"], s["text"]) for s in spans] == [
        ("person", "Kieffer"),
        ("address", "12, rue de la Gare"),
        ("location", "Réimech"),
    ]
    raw = text.encode()
    assert all(raw[s["start"] : s["end"]].decode() == s["text"] for s in spans)


def test_merge_keeps_gazetteer_names_inside_coordinated_ner_span():
    text = "D'Mett an de Méck sinn do"
    gaz = [span(text, "Mett", "person", "gazetteer", 0.55), span(text, "Méck", "person", "gazetteer", 0.55)]
    out = merge(text, gaz, [span(text, "D'Mett an de Méck", "organization", "ner")])
    assert [(s["type"], s["text"]) for s in out] == [("person", "Mett"), ("person", "Méck")]
