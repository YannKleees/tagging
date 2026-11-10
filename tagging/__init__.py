import json
from functools import lru_cache
from pathlib import Path

from tagging._core import Tagger
from tagging._core import redact as _redact

__all__ = ["Tagger", "detect", "detect_batch", "redact", "tagger"]

NER_DEFAULTS = {
    "model": "urchade/gliner_multi_pii-v1",
    "device": None,
    # 0.4 was chosen on the dev split: +2 points Luxembourgish hard recall for one extra false positive.
    "threshold": 0.4,
    "chunk_words": 200,
    "overlap_words": 50,
    "batch_size": 8,
    # Extra labels that absorb generic nouns ("the Plumber", "the Museum"); see ner.DISTRACTORS.
    "distractors": True,
    "postprocess": True,
    "confirm_ambiguous": True,
}


@lru_cache(maxsize=16)
def _cached(config_json):
    return Tagger(config_json)


def read_config(config=None):
    if config is None:
        return {}
    if isinstance(config, (str, Path)):
        path = Path(config)
        cfg = json.loads(path.read_text(encoding="utf-8"))
        for g in cfg.get("gazetteers", []):
            g["path"] = str(path.parent / g["path"])
        return cfg
    return dict(config)


def tagger(config=None):
    cfg = read_config(config)
    cfg.pop("ner", None)
    return _cached(json.dumps(cfg, sort_keys=True))


# The "ner" key switches on the contextual model; `true` uses NER_DEFAULTS, a dict overrides them.
def detect(text, config=None):
    return detect_batch([text], config)[0]


# Same as `detect` for many texts. With the model on, all texts share its batches, which is two to
# three times faster than one call per text.
def detect_batch(texts, config=None):
    cfg = read_config(config)
    ner_cfg = cfg.pop("ner", None)
    rules = _cached(json.dumps(cfg, sort_keys=True))
    spans = [json.loads(rules.detect(t)) for t in texts]
    if not ner_cfg:
        return spans

    from tagging import ner
    from tagging.merge import merge
    from tagging.postprocess import drop_everyday, postprocess

    opts = NER_DEFAULTS | (ner_cfg if isinstance(ner_cfg, dict) else {})
    model = ner.load(opts["model"], opts["device"])
    found = ner.predict_many(
        texts,
        model,
        opts["threshold"],
        opts["chunk_words"],
        opts["overlap_words"],
        opts["distractors"],
        opts["batch_size"],
    )
    # Every list hit, also those below the threshold, counts as evidence that a model span is a name.
    lists = _cached(json.dumps(cfg | {"threshold": 0}, sort_keys=True))
    out = []
    for text, rule_spans, ner_spans in zip(texts, spans, found):
        if opts["postprocess"]:
            ner_spans = postprocess(text, ner_spans)
        ner_spans = ner.to_bytes(text, ner_spans)
        if opts["postprocess"]:
            ner_spans = drop_everyday(text, ner_spans, json.loads(lists.detect(text)))
        out.append(merge(text, rule_spans, ner_spans, opts["confirm_ambiguous"]))
    return out


def redact(text, spans, mode="mask"):
    return _redact(text, json.dumps(spans), mode)
