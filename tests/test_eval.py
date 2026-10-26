import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parents[1] / "eval"))

import evaluate


def span(type, start, end, text="x"):
    return {"type": type, "start": start, "end": end, "text": text, "score": 0.9}


def test_match_exact_and_overlap():
    gold = [span("person", 0, 12), span("email", 20, 30)]
    pred = [span("person", 0, 5), span("email", 20, 30), span("phone", 40, 45)]
    assert evaluate.match(gold, pred, "exact") == {1: 1}
    assert evaluate.match(gold, pred, "overlap") == {0: 0, 1: 1}


def test_match_is_one_to_one_and_typed():
    gold = [span("person", 0, 5), span("person", 6, 10)]
    pred = [span("person", 0, 10)]
    assert evaluate.match(gold, pred, "overlap") == {0: 0}
    assert evaluate.match(gold, [span("location", 0, 5)], "overlap") == {}


def test_metrics_and_false_positives():
    rows = [
        {
            "id": "a",
            "lang": "lb",
            "subset": "hard",
            "text": "t",
            "spans": [span("person", 0, 4), span("location", 10, 15)],
        },
        {"id": "b", "lang": "lb", "subset": "entity_free", "text": "t", "spans": []},
    ]
    preds = [[span("person", 0, 4), span("person", 10, 15)], [span("location", 0, 3)]]
    gold, pred = evaluate.score(rows, preds)
    m = evaluate.metrics(gold, pred, ["subset"])
    hard = m.loc["hard"]
    assert (hard["recall_overlap"], hard["precision_overlap"], hard["any_type_recall"]) == (0.5, 0.5, 1.0)
    assert m.loc["entity_free", "n_pred"] == 1
    assert gold.loc[~gold["overlap"], "type"].tolist() == ["location"]
