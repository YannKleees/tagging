import re

import tagging
from tagging import ner
from tagging.merge import merge
from tagging.postprocess import drop_everyday, postprocess


class FakeModel:
    def __init__(self, entities):
        self.entities = entities
        self.calls = 0

    def batch_predict_entities(self, texts, labels, threshold=0.5, batch_size=8):
        self.calls += len(texts)
        self.labels = labels
        return [
            [
                {"start": m.start(), "end": m.end(), "text": name, "label": label, "score": 0.9}
                for name, label in self.entities.items()
                for m in re.finditer(re.escape(name), text)
            ]
            for text in texts
        ]


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


def test_postprocess_drops_pronouns_generic_places_and_coordinated_roles():
    for text, phrase, type in [
        ("The Baker and the Butcher close early.", "The Baker and the Butcher", "organization"),
        ("Wir sind gelaufen.", "Wir", "person"),
        ("They opened the new School.", "new School", "organization"),
        ("Il est dans la Cuisine.", "la Cuisine", "organization"),
    ]:
        assert postprocess(text, [char_span(text, phrase, type)]) == [], text


def test_postprocess_keeps_named_businesses():
    for text, phrase in [("Hien schafft am Garage Weis.", "Garage Weis"), ("Café Majerus ass zou.", "Café Majerus")]:
        assert [s["text"] for s in postprocess(text, [char_span(text, phrase, "organization")])] == [phrase]


def test_predict_many_keeps_texts_apart_and_drops_distractor_labels():
    model = FakeModel({"Jhemp": "person name", "Plumber": "profession or role"})
    spans = ner.predict_many(["The Plumber called Jhemp.", "Jhemp again"], model)
    assert [[(s["type"], s["text"], s["start"]) for s in t] for t in spans] == [
        [("person", "Jhemp", 19)],
        [("person", "Jhemp", 0)],
    ]
    assert set(ner.DISTRACTORS) <= set(model.labels)
    ner.predict_many(["Jhemp"], model, distractors=False)
    assert model.labels == list(ner.LABELS)


def test_postprocess_keeps_the_owner_of_a_possessive():
    for text, phrase, name in [
        ("Der Gréit hire Brudder schafft.", "Der Gréit hire Brudder", "Gréit"),
        ("Dem Charel säi Brudder schafft.", "Dem Charel säi Brudder", "Charel"),
        ("Charel's brother works.", "Charel's brother", "Charel"),
    ]:
        assert [s["text"] for s in postprocess(text, [char_span(text, phrase)])] == [name], text


def test_drop_everyday_removes_spans_made_of_common_words():
    for text, phrase, type in [
        ("The Fire Brigade arrived at the Warehouse.", "Fire Brigade", "organization"),
        ("Le Plombier a réparé l'évier.", "Le Plombier", "organization"),
        ("Die Feuerwehr war schnell da.", "Die Feuerwehr", "organization"),
        ("Das Museum schließt früher.", "Das Museum", "organization"),
        ("D'Schoul ass zou.", "Schoul", "organization"),
    ]:
        assert drop_everyday(text, [span(text, phrase, type, "ner")], []) == [], text


def test_drop_everyday_keeps_names():
    for text, phrase, type in [
        ("Den Här Kieffer ass do.", "Kieffer", "person"),
        ("Si schafft bei Garage Weis.", "Garage Weis", "organization"),
        # A word that is also a frequent name is only dropped after a determiner.
        ("Rose called yesterday.", "Rose", "person"),
        ("Wir trafen Richter gestern.", "Richter", "person"),
    ]:
        s = span(text, phrase, type, "ner")
        assert drop_everyday(text, [s], []) == [s], text
    text = "Der Richter hörte zu."
    assert drop_everyday(text, [span(text, "Richter", "person", "ner")], []) == []


def test_drop_everyday_keeps_spans_a_word_list_vouches_for():
    text = "Si schafft bei Deutsche Bank."
    ner_span = span(text, "Deutsche Bank", "organization", "ner")
    assert drop_everyday(text, [ner_span], []) == []
    assert drop_everyday(text, [ner_span], [span(text, "Deutsche Bank", "organization", "gazetteer", 0.85)]) == [
        ner_span
    ]
    # A name that is itself an everyday word only vouches for a span it covers.
    text = "An der Kantin an der Rei."
    weak = span(text, "An", "person", "gazetteer", 0.3)
    assert drop_everyday(text, [span(text, "An der Kantin", "organization", "ner")], [weak]) == []


def test_merge_cuts_a_listed_place_off_the_end_of_a_model_span():
    text = "Hie schafft bei Brasserie Schuman zu Luerenzweiler."
    place = span(text, "Luerenzweiler", "location", "gazetteer", 0.85)
    out = merge(text, [place], [span(text, "Brasserie Schuman zu Luerenzweiler", "organization", "ner")])
    assert [(s["type"], s["text"]) for s in out] == [
        ("organization", "Brasserie Schuman"),
        ("location", "Luerenzweiler"),
    ]
    # `du` builds names, it does not locate them.
    text = "Université du Luxembourg"
    place = span(text, "Luxembourg", "location", "gazetteer", 0.85)
    out = merge(text, [place], [span(text, text, "organization", "ner")])
    assert [(s["type"], s["text"]) for s in out] == [("organization", text)]


def test_merge_keeps_a_listed_person_under_a_titled_model_span():
    text = "D'Doktesch Ries ass do"
    person = span(text, "Ries", "person", "gazetteer", 0.85)
    out = merge(text, [person], [span(text, "D'Doktesch Ries", "organization", "ner")])
    assert [(s["type"], s["text"]) for s in out] == [("person", "Ries")]


def test_detect_batch_matches_detect(monkeypatch):
    model = FakeModel({"den Här Kieffer": "person name", "Die Feuerwehr": "organization name"})
    monkeypatch.setattr(ner, "load", lambda name, device: model)
    texts = ["Ech hunn den Här Kieffer gesinn.", "Die Feuerwehr war schnell da.", "De Jhemp wunnt zu Réimech."]
    batch = tagging.detect_batch(texts, {"ner": True})
    assert batch == [tagging.detect(t, {"ner": True}) for t in texts]
    assert [[(s["type"], s["text"]) for s in spans] for spans in batch] == [
        [("person", "Kieffer")],
        [],
        [("person", "Jhemp"), ("location", "Réimech")],
    ]
    assert tagging.detect_batch(texts) == [tagging.detect(t) for t in texts]


def test_postprocess_cuts_a_span_that_runs_into_the_sentence():
    for text, phrase, kept in [
        ("De Schmit huet d'Schof gefouert.", "Schmit huet d'Schof", "Schmit"),
        ("Den Zöllner kontrolliert den Lastwagen.", "Zöllner kontrolliert den Lastwagen", "Zöllner"),
        # Lowercase particles belong to names.
        ("Si schafft bei der Banque de Luxembourg.", "Banque de Luxembourg", "Banque de Luxembourg"),
        ("Hie wunnt zu Esch an der Alzette.", "Esch an der Alzette", "Esch an der Alzette"),
    ]:
        got = postprocess(text, [char_span(text, phrase, "organization")])
        assert [s["text"] for s in got] == [kept], text


def test_postprocess_drops_spans_without_letters():
    text = "Version 2.3.0.1 ass do."
    assert postprocess(text, [char_span(text, "2.3.0.1")]) == []


def test_drop_everyday_ignores_numbers_and_knows_dictionary_words():
    for text, phrase in [
        ("Invoice 4471 is paid.", "Invoice 4471"),
        ("D'Hiewan huet de Puppelchen besicht.", "Hiewan"),
        ("De Coiffer huet méindes zou.", "Coiffer"),
    ]:
        assert drop_everyday(text, [span(text, phrase, "person", "ner")], []) == [], text


def test_drop_everyday_keeps_several_capitalized_words_without_a_determiner():
    text = "She joined National Water Well Association last year."
    name = span(text, "National Water Well Association", "organization", "ner")
    assert drop_everyday(text, [name], []) == [name]
    # After a determiner the same shape is a description, and German capitalizes every noun.
    for text, phrase in [
        ("The Fire Brigade arrived at the Tax Office.", "Fire Brigade"),
        ("Heute bleibt Kindergarten Schule geschlossen und wir warten.", "Kindergarten Schule"),
    ]:
        assert drop_everyday(text, [span(text, phrase, "organization", "ner")], []) == [], text


class FakeDoc:
    def __init__(self, text, entities):
        self.ents = [FakeEntity(text, name, label) for name, label in entities.items() if name in text]


class FakeEntity:
    def __init__(self, text, name, label):
        self.text, self.label_ = name, label
        self.start_char = text.index(name)
        self.end_char = self.start_char + len(name)


class FakeNlp:
    def __init__(self, name, entities, loaded):
        self.name, self.entities = name, entities
        loaded.append(name)

    def pipe(self, texts, batch_size=64):
        return [FakeDoc(t, self.entities) for t in texts]


def fake_spacy(monkeypatch, entities):
    from tagging import spacy_ner

    loaded = []
    monkeypatch.setattr(spacy_ner, "load", lambda name: FakeNlp(name, entities, loaded))
    return loaded


def test_spacy_backend_picks_a_model_per_language(monkeypatch):
    from tagging import spacy_ner

    loaded = fake_spacy(monkeypatch, {"Nzuzi": "PER", "Zzyzx": "ORG", "tomorrow": "DATE"})
    texts = ["Ech hunn d'Nzuzi gëschter gesinn.", "Wir haben Nzuzi von Zzyzx gesehen.", "See you tomorrow.", "Nzuzi"]
    spans = spacy_ner.predict_many(texts)
    assert [[(s["type"], s["text"]) for s in t] for t in spans] == [
        [("person", "Nzuzi")],
        [("person", "Nzuzi"), ("organization", "Zzyzx")],
        [],
        [("person", "Nzuzi")],
    ]
    assert set(loaded) == {spacy_ner.MULTILINGUAL, "de_core_news_sm", "en_core_web_sm"}
    assert all(t[s["start"] : s["end"]] == s["text"] for t, ss in zip(texts, spans) for s in ss)
    loaded.clear()
    spacy_ner.predict_many(texts, {"de": "de_core_news_lg"}, language="de")
    assert loaded == ["de_core_news_lg"]


def test_detect_with_the_fast_backend(monkeypatch):
    fake_spacy(monkeypatch, {"Nzuzi": "PER", "Die Feuerwehr": "ORG"})
    text = "Die Feuerwehr hat Nzuzi in Trier gesehen."
    spans = tagging.detect(text, {"ner": {"backend": "spacy"}})
    assert [(s["type"], s["text"], s["source"]) for s in spans] == [
        ("person", "Nzuzi", "ner"),
        ("location", "Trier", "gazetteer"),
    ]


def test_both_backends_are_combined(monkeypatch):
    fake_spacy(monkeypatch, {"Nzuzi": "PER"})
    monkeypatch.setattr(ner, "load", lambda name, device: FakeModel({"Zzyzx Consulting": "organization name"}))
    text = "Nzuzi schafft bei Zzyzx Consulting."
    spans = tagging.detect(text, {"ner": {"backend": "both"}})
    assert [(s["type"], s["text"]) for s in spans] == [("person", "Nzuzi"), ("organization", "Zzyzx Consulting")]


def test_unknown_backend_is_an_error():
    import pytest

    with pytest.raises(ValueError, match="backend"):
        tagging.detect("x", {"ner": {"backend": "bert"}})
