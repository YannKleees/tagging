from tagging.postprocess import CONJUNCTION, LOCATIVE, strip_titles, trim

CHECKSUM_TYPES = {"iban", "credit_card", "national_id"}
# Gazetteer entries that are also common words score 0.55; with NER enabled they need its support.
AMBIGUOUS_BELOW = 0.6


def overlaps(a, b):
    return a["start"] < b["end"] and b["start"] < a["end"]


def contains(a, b):
    return a["start"] <= b["start"] and b["end"] <= a["end"] and (a["end"] - a["start"]) > (b["end"] - b["start"])


# A model span that strictly contains gazetteer spans of another type replaces them (an address
# around "Gare"), except when it is a coordination of several gazetteer names ("D'Mett an de Méck").
# A model span that is only a title in front of a listed person ("D'Doktesch Ries" as an
# organization) does not replace the person either.
def replaces(s, clash):
    coordinated = len(clash) > 1 and CONJUNCTION.search(s["text"])
    a, b = trim(s["text"], *strip_titles(s["text"], 0, len(s["text"])))
    titled = any(k["type"] == "person" and k["text"] == s["text"][a:b] for k in clash)
    return not (coordinated or titled) and all(tier(s) == 3 and tier(k) == 2 and contains(s, k) for k in clash)


# "Brasserie Schuman zu Luerenzweiler", "ADEM zu Welschent": a model span that ends in a listed
# place after a locative preposition is cut before the preposition, so the place keeps its own span.
def cut_trailing_place(text, s, gazetteer):
    raw = text.encode()
    for g in gazetteer:
        tail = g["type"] == "location" and g["score"] >= AMBIGUOUS_BELOW and g["end"] == s["end"]
        if not tail or s["type"] in ("location", "address") or g["start"] <= s["start"]:
            continue
        head = raw[s["start"] : g["start"]].decode()
        m = LOCATIVE.search(head)
        if m and m.start() > 0:
            end = s["start"] + len(head[: m.start()].encode())
            return {**s, "end": end, "text": raw[s["start"] : end].decode()}
    return s


def tier(s):
    if s["source"] == "rule":
        return 0 if s["type"] in CHECKSUM_TYPES else 1
    return 2 if "gazetteer" in s["source"] else 3


def union(text, group):
    start, end = min(s["start"] for s in group), max(s["end"] for s in group)
    sources = sorted({s["source"] for s in group})
    return {
        "type": group[0]["type"],
        "start": start,
        "end": end,
        "text": text.encode()[start:end].decode(),
        "score": max(s["score"] for s in group),
        "source": "+".join(sources),
    }


def merge_same_type(text, spans):
    out = []
    for s in sorted(spans, key=lambda s: (s["type"], s["start"])):
        group = out[-1]["_group"] if out and out[-1]["type"] == s["type"] and overlaps(out[-1], s) else None
        if group is None:
            out.append({**s, "_group": [s]})
        else:
            group.append(s)
            out[-1] = {**union(text, group), "_group": group}
    return [{k: v for k, v in s.items() if k != "_group"} for s in out]


# Precedence: checksum-validated rules, then other rules, then gazetteer, then NER. Rule spans
# always win. Overlapping gazetteer and NER spans of the same type are merged into their union, so
# "Jhemp" from the gazetteer and "Jhemp Schmit" from the model cover the full name. Across types
# the gazetteer wins, unless the NER span replaces it (see `replaces`).
def merge(text, rule_spans, ner_spans, confirm_ambiguous=True):
    rules = [s for s in rule_spans if s["source"] == "rule"]
    gazetteer = [s for s in rule_spans if s["source"] == "gazetteer"]
    ner = [s for s in ner_spans if not any(overlaps(s, r) for r in rules)]
    if confirm_ambiguous:
        gazetteer = [g for g in gazetteer if g["score"] >= AMBIGUOUS_BELOW or any(overlaps(g, n) for n in ner)]
    ner = [cut_trailing_place(text, s, gazetteer) for s in ner]
    names = merge_same_type(text, gazetteer + ner)
    kept = []
    ranked = sorted(names, key=lambda s: (tier(s), -(s["end"] - s["start"]), -s["score"]))
    for s in ranked:
        clash = [k for k in kept if overlaps(k, s)]
        if clash and replaces(s, clash):
            kept = [k for k in kept if k not in clash]
            clash = []
        if not clash:
            kept.append(s)
    return sorted(rules + kept, key=lambda s: (s["start"], s["end"], s["type"]))
