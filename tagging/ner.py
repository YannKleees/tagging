import re
from functools import lru_cache

import numpy as np

DEFAULT_MODEL = "urchade/gliner_multi_pii-v1"
# Bare labels such as "person" or "location" make the model tag generic nouns ("Die Mutter",
# "Garage", "Kino"); descriptive labels avoid that.
LABELS = {
    "person name": "person",
    "organization name": "organization",
    "city or place name": "location",
    "street address": "address",
}
# Labels the model may pick instead of a name. Their spans are thrown away, so "the Plumber" or
# "the Museum" compete with "person name" and lose. On the dev split they raise recall as well.
DISTRACTORS = ["profession or role", "relative", "building or facility", "object", "group of people"]
WORD = re.compile(r"\S+")


@lru_cache(maxsize=2)
def load(name=DEFAULT_MODEL, device=None):
    import torch
    from gliner import GLiNER

    device = device or ("cuda" if torch.cuda.is_available() else "cpu")
    return GLiNER.from_pretrained(name).to(device).eval()


def windows(text, size, overlap):
    words = [m.span() for m in WORD.finditer(text)]
    step = max(size - overlap, 1)
    for i in range(0, max(len(words) - overlap, 1), step):
        chunk = words[i : i + size]
        if chunk:
            yield chunk[0][0], chunk[-1][1]


def dedupe(spans):
    kept = []
    for s in sorted(spans, key=lambda s: (-(s["end"] - s["start"]), -s["score"], s["start"])):
        if not any(s["start"] < k["end"] and k["start"] < s["end"] for k in kept):
            kept.append(s)
    return sorted(kept, key=lambda s: s["start"])


# Character offsets; chunks overlap so entities cut at a window edge are seen whole in the next one.
def predict(text, model, threshold=0.5, size=200, overlap=50, distractors=True, batch_size=8):
    return predict_many([text], model, threshold, size, overlap, distractors, batch_size)[0]


# All chunks of all texts go through the model in batches, which is two to three times faster on
# short texts than one call per text.
def predict_many(texts, model, threshold=0.5, size=200, overlap=50, distractors=True, batch_size=8):
    chunks = [(i, a, b) for i, text in enumerate(texts) for a, b in windows(text, size, overlap)]
    labels = list(LABELS) + (DISTRACTORS if distractors else [])
    # GLiNER 0.2.30 renamed `batch_predict_entities` to `inference`.
    run = getattr(model, "inference", None) or model.batch_predict_entities
    found = run([texts[i][a:b] for i, a, b in chunks], labels, threshold=threshold, batch_size=batch_size)
    spans = [[] for _ in texts]
    for (i, a, _), entities in zip(chunks, found):
        for e in entities:
            if e["label"] in LABELS:
                start, end = a + e["start"], a + e["end"]
                spans[i].append(
                    {
                        "type": LABELS[e["label"]],
                        "start": start,
                        "end": end,
                        "text": texts[i][start:end],
                        "score": round(float(e["score"]), 4),
                        "source": "ner",
                    }
                )
    return [dedupe(s) for s in spans]


def byte_offsets(text):
    cp = np.frombuffer(text.encode("utf-32-le"), dtype=np.uint32)
    width = 1 + (cp >= 0x80) + (cp >= 0x800) + (cp >= 0x10000)
    return np.concatenate([[0], np.cumsum(width)])


def to_bytes(text, spans):
    offsets = byte_offsets(text)
    return [{**s, "start": int(offsets[s["start"]]), "end": int(offsets[s["end"]])} for s in spans]
