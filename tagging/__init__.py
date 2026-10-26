import json
from functools import lru_cache
from pathlib import Path

from tagging._core import Tagger
from tagging._core import redact as _redact

__all__ = ["Tagger", "detect", "redact", "tagger"]

NER_DEFAULTS = {
    "model": "urchade/gliner_multi_pii-v1",
    "device": None,
    "threshold": 0.5,
    "chunk_words": 200,
    "overlap_words": 50,
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
    cfg = read_config(config)
    ner_cfg = cfg.pop("ner", None)
    spans = json.loads(_cached(json.dumps(cfg, sort_keys=True)).detect(text))
    if not ner_cfg:
        return spans

    from tagging import ner
    from tagging.merge import merge
    from tagging.postprocess import postprocess

    opts = NER_DEFAULTS | (ner_cfg if isinstance(ner_cfg, dict) else {})
    model = ner.load(opts["model"], opts["device"])
    found = ner.predict(text, model, opts["threshold"], opts["chunk_words"], opts["overlap_words"])
    if opts["postprocess"]:
        found = postprocess(text, found)
    return merge(text, spans, ner.to_bytes(text, found), opts["confirm_ambiguous"])


def redact(text, spans, mode="mask"):
    return _redact(text, json.dumps(spans), mode)
