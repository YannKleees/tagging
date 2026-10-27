import os

import pytest

import tagging

pytest.importorskip("gliner")
pytestmark = pytest.mark.skipif(
    not os.environ.get("TAGGING_NER_TESTS"), reason="set TAGGING_NER_TESTS=1 to run the GLiNER model"
)


def test_model_finds_luxembourgish_names_places_and_addresses():
    text = "D'Mett an de Méck sinn op Réimech gefuer, den Här Kieffer wunnt 12, rue de la Gare."
    got = {(s["type"], s["text"]) for s in tagging.detect(text, {"ner": True})}
    assert {
        ("person", "Mett"),
        ("person", "Méck"),
        ("location", "Réimech"),
        ("person", "Kieffer"),
        ("address", "12, rue de la Gare"),
    } <= got


def test_model_ignores_kinship_and_generic_nouns():
    for text in [
        "Die Mutter und der Nachbar gehen ins Kino, die Garage ist zu.",
        "D'Mamm an de Bopa ginn haut an de Kino.",
    ]:
        assert tagging.detect(text, {"ner": True}) == [], text
