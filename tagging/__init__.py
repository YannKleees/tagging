import json
from functools import lru_cache
from pathlib import Path

from tagging._core import Tagger
from tagging._core import redact as _redact

__all__ = ["Tagger", "detect", "redact", "tagger"]


@lru_cache(maxsize=16)
def _cached(config_json=None, path=None):
    return Tagger.from_file(path) if path else Tagger(config_json)


def tagger(config=None):
    if isinstance(config, (str, Path)):
        return _cached(path=str(config))
    return _cached(None if config is None else json.dumps(config, sort_keys=True))


def detect(text, config=None):
    return json.loads(tagger(config).detect(text))


def redact(text, spans, mode="mask"):
    return _redact(text, json.dumps(spans), mode)
