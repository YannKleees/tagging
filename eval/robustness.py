"""Checks that the eval split cannot make: names the word lists were not built around, damaged
casing and diacritics, and two external datasets. Writes results/robustness.md.

The external datasets are downloaded from Hugging Face and not redistributed: WikiANN (Wikipedia
sentences with silver PER, ORG and LOC labels) and ai4privacy/pii-masking-300k (synthetic letters
and forms). Needs `pip install huggingface_hub pyarrow`.
"""

import argparse
import json
import random
import unicodedata
from pathlib import Path

import pandas as pd

import evaluate
import tagging

ROOT = Path(__file__).parent
CONFIGS = {"rules": None, "rules + fast NER": {"ner": {"backend": "spacy"}}, "rules + NER": {"ner": True}}
NAMES = ["person", "organization", "location"]
# Names as they are common in Luxembourg today, written without looking at the word lists:
# Balkan, Cape Verdean, West African, Turkish, Polish, Asian, East African, Ukrainian, Italian,
# Romanian and local ones.
FIRST = [
    "Dijana",
    "Mirsad",
    "Aleksandra",
    "Nuno",
    "Tiago",
    "Yara",
    "Amadou",
    "Fatoumata",
    "Mehmet",
    "Elif",
    "Agnieszka",
    "Krzysztof",
    "Wei",
    "Priya",
    "Rahul",
    "Selam",
    "Yonas",
    "Oleksandr",
    "Svitlana",
    "Giulia",
    "Matteo",
    "Emir",
    "Ajla",
    "Kwame",
    "Thi",
    "Huong",
    "Nour",
    "Youssef",
    "Samira",
    "Bogdan",
    "Ioana",
    "Laetitia",
    "Gwendoline",
    "Yannick",
    "Sven",
    "Finn",
    "Liv",
    "Noémie",
    "Maëlle",
    "Kevin",
]
LAST = [
    "Kovačević",
    "Hadžić",
    "Nikolić",
    "Tavares",
    "Semedo",
    "Diallo",
    "Traoré",
    "Yılmaz",
    "Demir",
    "Kowalczyk",
    "Wiśniewski",
    "Zhang",
    "Sharma",
    "Patel",
    "Tesfay",
    "Habte",
    "Shevchenko",
    "Bondarenko",
    "Esposito",
    "Colombo",
    "Mujkić",
    "Osmanović",
    "Mensah",
    "Nguyen",
    "Benali",
    "Haddad",
    "Popescu",
    "Ionescu",
    "Ewert",
    "Majerus",
    "Reckinger",
    "Stoltz",
    "Biwer",
    "Flammang",
    "Urbany",
    "Wolter",
    "Lentz",
    "Goergen",
    "Thinnes",
    "Kayser",
]
ORGS = [
    "Atelier Dräieck", "Solux Consulting", "Kleederbuttek Faarwen", "Nordstad Immo", "Fiduciaire du Kirchberg",
    "Bureau Comptable Reiff", "Pizzeria Bella Vita", "Lëtz Data", "Moselle Logistics", "Garage Autocenter Sud",
    "Clervaux Timber", "Epicerie Am Duerf", "Crèche Butzemillen", "Luxsenior Home Care", "Müllerthal Trail Tours",
    "Schreinerei Holzwuerm", "Cabinet Dentaire Belair", "Tecnolux Engineering", "Vinsmoselle Reserve", "Fit4Coding",
]  # fmt: skip
PLACES = [
    "Perl", "Wasserbillig", "Echternacherbrück", "Audun-le-Tiche", "Villerupt", "Athus", "Messancy", "Wincheringen",
    "Schengen", "Cattenom", "Sierck-les-Bains", "Bollendorf", "Aubange", "Hettange-Grande", "Porto", "Praia",
    "Podgorica", "Braga", "Kraków", "Izmir",
]  # fmt: skip
WIKIANN_TAGS = {1: "person", 2: "person", 3: "organization", 4: "organization", 5: "location", 6: "location"}
AI4PRIVACY = {
    "en": "data/validation/1english_openpii_8k.jsonl",
    "fr": "data/validation/french_openpii_8k.jsonl",
    "de": "data/validation/german_openpii_8k.jsonl",
}


def new_person(old, rng):
    full = len(old.split()) >= 2
    name = f"{rng.choice(FIRST)} {rng.choice(LAST)}" if full else rng.choice(FIRST if rng.random() < 0.5 else LAST)
    return name.lower() if old.islower() else name


# Replaces the gold spans of the given types and recomputes all byte offsets. The same old name
# gets the same new name within a sentence.
def swap(row, makers, rng):
    raw, out, pos, spans, chosen = row["text"].encode(), b"", 0, [], {}
    for s in sorted(row["spans"], key=lambda s: s["start"]):
        old = raw[s["start"] : s["end"]].decode()
        new = old
        if s["type"] in makers:
            new = chosen.setdefault((s["type"], old.lower()), makers[s["type"]](old, rng))
        out += raw[pos : s["start"]]
        start = len(out)
        out += new.encode()
        pos = s["end"]
        spans.append({**s, "start": start, "end": len(out), "text": new})
    return {**row, "text": (out + raw[pos:]).decode(), "spans": spans}


# Maps every character to one character, so character offsets stay valid.
def reshape(row, fn):
    raw, text = row["text"].encode(), row["text"]
    chars = [(len(raw[: s["start"]].decode()), len(raw[: s["end"]].decode())) for s in row["spans"]]
    new = "".join(fn(c) for c in text)
    spans = [
        {**s, "start": len(new[:a].encode()), "end": len(new[:b].encode()), "text": new[a:b]}
        for s, (a, b) in zip(row["spans"], chars)
    ]
    return {**row, "text": new, "spans": spans}


def without_diacritic(c):
    d = unicodedata.normalize("NFD", c)
    return d[0] if len(d) > 1 and unicodedata.combining(d[1]) else c


def one_to_one(fn):
    return lambda c: fn(c) if len(fn(c)) == 1 else c


def byte_span(text, start, end, kind):
    return {"type": kind, "start": len(text[:start].encode()), "end": len(text[:end].encode()), "text": text[start:end]}


# One row per gold span: found with the right type, with exact offsets, or covered by any span.
def recall(rows, config):
    preds = tagging.detect_batch([r["text"] for r in rows], config)
    out = []
    for r, pred in zip(rows, preds):
        for g in r["spans"]:
            hits = [p for p in pred if evaluate.overlaps(g, p)]
            typed = any(p["type"] == g["type"] for p in hits)
            exact = any((p["start"], p["end"], p["type"]) == (g["start"], g["end"], g["type"]) for p in hits)
            out.append({"type": g["type"], "typed": typed, "exact": exact, "covered": bool(hits)})
    on_gold = [any(evaluate.overlaps(p, g) for g in r["spans"]) for r, pred in zip(rows, preds) for p in pred]
    return pd.DataFrame(out), on_gold


def perturbations():
    rows = [r for r in evaluate.load(ROOT / "data" / "eval.jsonl") if r["subset"] != "entity_free"]
    rng = random.Random(2026)
    everything = {
        "person": new_person,
        "organization": lambda _, g: g.choice(ORGS),
        "location": lambda _, g: g.choice(PLACES),
    }
    variants = {
        "original eval split": rows,
        "new person names": [swap(r, {"person": new_person}, rng) for r in rows],
        "new persons, firms and places": [swap(r, everything, rng) for r in rows],
        "no diacritics": [reshape(r, without_diacritic) for r in rows],
        "all lowercase": [reshape(r, one_to_one(str.lower)) for r in rows],
        "ALL CAPITALS": [reshape(r, one_to_one(str.upper)) for r in rows],
    }
    table = []
    for name, config in CONFIGS.items():
        for variant, changed in variants.items():
            df, _ = recall(changed, config)
            names = df[df["type"].isin(NAMES)]
            table.append(
                {
                    "config": name,
                    "variant": variant,
                    **{t: df[df["type"] == t]["typed"].mean() for t in [*NAMES, "address"]},
                    "structured types": df[~df["type"].isin([*NAMES, "address"])]["typed"].mean(),
                    "names covered, any type": names["covered"].mean(),
                    "names exact": names["exact"].mean(),
                }
            )
            print(table[-1])
    return pd.DataFrame(table)


def wikiann(n):
    import pyarrow.parquet as pq
    from huggingface_hub import hf_hub_download

    table = []
    for lang in ["lb", "de", "fr", "en"]:
        path = hf_hub_download("unimelb-nlp/wikiann", f"{lang}/test-00000-of-00001.parquet", repo_type="dataset")
        examples = pq.read_table(path).to_pylist()
        random.Random(1).shuffle(examples)
        rows = []
        for ex in examples[:n]:
            text, spans, current = "", [], None
            for token, tag in zip(ex["tokens"], ex["ner_tags"]):
                text += " " if text else ""
                start = len(text)
                text += token
                if tag in (1, 3, 5) or (tag in WIKIANN_TAGS and current is None):
                    current = [start, len(text), WIKIANN_TAGS[tag]]
                    spans.append(current)
                elif tag in WIKIANN_TAGS:
                    current[1] = len(text)
                else:
                    current = None
            rows.append({"text": text, "spans": [byte_span(text, a, b, kind) for a, b, kind in spans]})
        for name, config in CONFIGS.items():
            df, on_gold = recall(rows, config)
            table.append(
                {
                    "lang": lang,
                    "config": name,
                    **{t: df[df["type"] == t]["covered"].mean() for t in NAMES},
                    "all covered": df["covered"].mean(),
                    "right type": df["typed"].mean(),
                    "predictions on a gold entity": sum(on_gold) / max(len(on_gold), 1),
                    "entities": len(df),
                }
            )
            print(table[-1])
    return pd.DataFrame(table)


def ai4privacy(n):
    from huggingface_hub import hf_hub_download

    found = []
    for lang, file in AI4PRIVACY.items():
        path = hf_hub_download("ai4privacy/pii-masking-300k", file, repo_type="dataset")
        lines = Path(path).read_text(encoding="utf-8").splitlines()
        random.Random(1).shuffle(lines)
        docs = [json.loads(x) for x in lines[:n]]
        rows = [
            {
                "text": d["source_text"],
                "spans": [byte_span(d["source_text"], m["start"], m["end"], m["label"]) for m in d["privacy_mask"]],
            }
            for d in docs
        ]
        for name, config in CONFIGS.items():
            df, _ = recall(rows, config)
            found.append(df.assign(config=name, lang=lang))
    df = pd.concat(found)
    by_label = df.pivot_table(index="type", columns="config", values="covered", aggfunc="mean")
    by_label["items"] = df[df["config"] == "rules"].groupby("type").size()
    by_lang = df.pivot_table(index="lang", columns="config", values="covered", aggfunc="mean")
    return by_label.rename_axis("label"), by_lang


def main():
    ap = argparse.ArgumentParser(description="Robustness checks beyond the eval split")
    ap.add_argument("--wikiann", type=int, default=300, help="sentences per language")
    ap.add_argument("--ai4privacy", type=int, default=120, help="documents per language")
    args = ap.parse_args()
    perturbed = perturbations()
    wiki = wikiann(args.wikiann)
    by_label, by_lang = ai4privacy(args.ai4privacy)
    report = [
        "# Robustness",
        "",
        "Recall only. `rules + NER` is the package default with the GLiNER model, `rules + fast NER` the spaCy backend.",
        "",
        "## The eval split with other names and damaged text",
        "",
        "The names of the eval split are replaced by names written without looking at the word lists (40 first",
        "names, 40 surnames, 20 small firms, 20 towns), or the text loses its diacritics or its casing. Columns",
        "`person` to `structured types` count a span of the right type that overlaps the gold span.",
        "",
        evaluate.md(perturbed, list(perturbed.columns)),
        "",
        f"## WikiANN, {args.wikiann} Wikipedia sentences per language",
        "",
        "Silver labels for persons, organizations and locations, and a different idea of an entity than this tool",
        "has (countries, sports clubs, historical figures). Columns `person` to `all covered` count an entity that",
        "any predicted span overlaps.",
        "",
        evaluate.md(wiki, list(wiki.columns)),
        "",
        f"## ai4privacy pii-masking-300k, {args.ai4privacy} synthetic documents per language",
        "",
        "Share of labelled items that any predicted span overlaps, with the default config: dates that are no birth",
        "dates and times of day are off, and sex and title have no detector on purpose. A different sample of the",
        "same dataset served to develop the form-field and ID rules; these documents were not looked at.",
        "",
        evaluate.md(by_lang.reset_index(), ["lang", *CONFIGS]),
        "",
        evaluate.md(by_label.reset_index(), ["label", *CONFIGS, "items"]),
        "",
    ]
    out = ROOT / "results" / "robustness.md"
    out.write_text("\n".join(report), encoding="utf-8", newline="\n")
    print(f"written to {out}")


if __name__ == "__main__":
    main()
