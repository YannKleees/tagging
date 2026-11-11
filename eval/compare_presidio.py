"""Runs Microsoft Presidio and this tool on the same data and writes results/presidio.md.

Presidio is set up the way a careful user would: spaCy's large models for English, German and
French, the multilingual `xx_ent_wiki_sm` for Luxembourgish (spaCy has no Luxembourgish NER model),
organizations switched on, phone regions extended to Luxembourg and its neighbours. Presidio is told
the language of every text; this tool is not.

Needs `pip install presidio-analyzer spacy huggingface_hub pyarrow`, spaCy's large English, French
and German pipelines for Presidio and the small ones for this tool's fast backend.
"""

import json
import random
import time
from pathlib import Path

import pandas as pd

import evaluate
import robustness as rb
import tagging

ROOT = Path(__file__).parent
LANGS = ["en", "fr", "de", "lb"]
MODELS = {"en": "en_core_web_lg", "fr": "fr_core_news_lg", "de": "de_core_news_lg", "lb": "xx_ent_wiki_sm"}
PHONE_REGIONS = ["LU", "FR", "DE", "BE", "NL", "GB", "US", "CH", "PT", "IT"]
# Presidio entity types in this tool's vocabulary. Presidio has one DATE_TIME type for all dates.
TYPES = {
    "PERSON": "person", "LOCATION": "location", "ORGANIZATION": "organization", "EMAIL_ADDRESS": "email",
    "PHONE_NUMBER": "phone", "IBAN_CODE": "iban", "CREDIT_CARD": "credit_card", "IP_ADDRESS": "ip_address",
    "URL": "url", "DATE_TIME": "date_of_birth",
}  # fmt: skip
NAMES = ["person", "organization", "location"]
STRUCTURED = ["email", "phone", "iban", "credit_card", "national_id", "ip_address", "url", "date_of_birth"]
# Presidio reports every date and time, so this tool is asked to do the same.
BASE = {"dates_and_times": True}
CONFIGS = {
    "rules": BASE,
    "rules + fast NER": BASE | {"ner": {"backend": "spacy"}},
    "rules + NER": BASE | {"ner": True},
    "rules + both": BASE | {"ner": {"backend": "both"}},
}
TOOLS = ["presidio", *CONFIGS]


def presidio():
    from presidio_analyzer import AnalyzerEngine, RecognizerRegistry
    from presidio_analyzer.nlp_engine import NlpEngineProvider
    from presidio_analyzer.predefined_recognizers import PhoneRecognizer

    labels = {"PER": "PERSON", "PERSON": "PERSON", "LOC": "LOCATION", "GPE": "LOCATION", "ORG": "ORGANIZATION"}
    labels |= {"DATE": "DATE_TIME", "TIME": "DATE_TIME"}
    engine = NlpEngineProvider(
        nlp_configuration={
            "nlp_engine_name": "spacy",
            "models": [{"lang_code": lang, "model_name": model} for lang, model in MODELS.items()],
            "ner_model_configuration": {"model_to_presidio_entity_mapping": labels, "labels_to_ignore": ["O"]},
        }
    ).create_engine()
    registry = RecognizerRegistry(supported_languages=LANGS)
    registry.load_predefined_recognizers(languages=LANGS, nlp_engine=engine)
    registry.remove_recognizer("PhoneRecognizer")
    for lang in LANGS:
        registry.add_recognizer(PhoneRecognizer(supported_regions=PHONE_REGIONS, supported_language=lang))
    return AnalyzerEngine(nlp_engine=engine, registry=registry, supported_languages=LANGS)


def presidio_spans(analyzer, text, lang):
    spans = []
    for r in analyzer.analyze(text=text, language=lang):
        kind = TYPES.get(r.entity_type, r.entity_type.lower())
        spans.append({**rb.byte_span(text, r.start, r.end, kind), "score": r.score, "source": "presidio"})
    return spans


# Presidio's three large pipelines and GLiNER do not fit into 8 GB together, so Presidio runs in a
# process of its own that ends before this tool's models are loaded.
def presidio_worker(texts, langs):
    analyzer = presidio()
    presidio_spans(analyzer, texts[0], langs[0])
    t0 = time.perf_counter()
    spans = [presidio_spans(analyzer, text, lang) for text, lang in zip(texts, langs)]
    return spans, time.perf_counter() - t0


def presidio_predictions(datasets):
    from concurrent.futures import ProcessPoolExecutor
    from multiprocessing import get_context

    rows = [r for d in datasets for r in d]
    with ProcessPoolExecutor(1, mp_context=get_context("spawn")) as pool:
        spans, seconds = pool.submit(presidio_worker, [r["text"] for r in rows], [r["lang"] for r in rows]).result()
    out, at = [], 0
    for d in datasets:
        out.append(spans[at : at + len(d)])
        at += len(d)
    return out, 1e6 * seconds / sum(len(r["text"].split()) for r in rows)


# With `timed`, every config runs a second time and that pass is timed, so that loading the models
# and warming them up is not part of the time.
def predictions(rows, by_presidio, timed=False):
    texts = [r["text"] for r in rows]
    out, seconds = {"presidio": by_presidio}, {}
    for name, config in CONFIGS.items():
        out[name] = tagging.detect_batch(texts, config)
        if timed:
            t0 = time.perf_counter()
            tagging.detect_batch(texts, config)
            seconds[name] = time.perf_counter() - t0
    words = sum(len(t.split()) for t in texts)
    return out, {k: 1e6 * v / words for k, v in seconds.items()}


# Share of gold spans that some predicted span overlaps, per gold type. A redaction tool has done
# its job when the span is covered, whatever it called it.
def covered(rows, preds, groups):
    table = {}
    for tool, pred in preds.items():
        hit = [(g["type"], any(evaluate.overlaps(g, p) for p in ps)) for r, ps in zip(rows, pred) for g in r["spans"]]
        df = pd.DataFrame(hit, columns=["type", "hit"])
        table[tool] = {name: df[df["type"].isin(types)]["hit"].mean() for name, types in groups.items()}
    return pd.DataFrame(table).rename_axis("gold type").reset_index()


def wikiann_rows(lang, n):
    import pyarrow.parquet as pq
    from huggingface_hub import hf_hub_download

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
            if tag in (1, 3, 5) or (tag in rb.WIKIANN_TAGS and current is None):
                current = [start, len(text), rb.WIKIANN_TAGS[tag]]
                spans.append(current)
            elif tag in rb.WIKIANN_TAGS:
                current[1] = len(text)
            else:
                current = None
        rows.append({"lang": lang, "text": text, "spans": [rb.byte_span(text, a, b, k) for a, b, k in spans]})
    return rows


def ai4privacy_rows(lang, n):
    from huggingface_hub import hf_hub_download

    path = hf_hub_download("ai4privacy/pii-masking-300k", rb.AI4PRIVACY[lang], repo_type="dataset")
    lines = Path(path).read_text(encoding="utf-8").splitlines()
    random.Random(1).shuffle(lines)
    docs = [json.loads(x) for x in lines[:n]]
    return [
        {
            "lang": lang,
            "text": d["source_text"],
            "spans": [rb.byte_span(d["source_text"], m["start"], m["end"], m["label"]) for m in d["privacy_mask"]],
        }
        for d in docs
    ]


# Hits on text without personal data. Dates and times are counted apart: both tools were asked
# for them, so they are no mistakes.
def hits(rows, preds):
    table = {}
    for tool, pred in preds.items():
        other = [r["lang"] for r, ps in zip(rows, pred) for p in ps if p["type"] not in ("date", "date_of_birth")]
        by_lang = pd.Series(other, dtype=object).value_counts().reindex(LANGS, fill_value=0)
        dates = sum(p["type"] in ("date", "date_of_birth") for ps in pred for p in ps)
        table[tool] = {**by_lang.to_dict(), "total": int(by_lang.sum()), "dates and times": dates}
    return pd.DataFrame(table).T.rename_axis("tool").reset_index()


# Share of predicted spans that touch a labelled item: a rough precision, rough because the labels
# of both external datasets are incomplete.
def on_gold(rows, preds):
    share = {}
    for tool, pred in preds.items():
        hit = [any(evaluate.overlaps(p, g) for g in r["spans"]) for r, ps in zip(rows, pred) for p in ps]
        share[tool] = {"predictions": len(hit), "on a labelled item": sum(hit) / max(len(hit), 1)}
    return pd.DataFrame(share).T.rename_axis("tool").reset_index()


def section(title, note, df):
    return [f"## {title}", "", note, "", evaluate.md(df, list(df.columns)), ""]


def by_language(rows, preds, groups):
    tables = []
    for lang in dict.fromkeys(r["lang"] for r in rows):
        idx = [i for i, r in enumerate(rows) if r["lang"] == lang]
        df = covered([rows[i] for i in idx], {k: [v[i] for i in idx] for k, v in preds.items()}, groups)
        tables.append(df.assign(lang=lang))
    return pd.concat(tables)[["lang", "gold type", *TOOLS]]


def main():
    eval_rows = evaluate.load(ROOT / "data" / "eval.jsonl")
    entity = [r for r in eval_rows if r["subset"] != "entity_free"]
    free = [r for r in eval_rows if r["subset"] == "entity_free"]
    rng = random.Random(2026)
    makers = {
        "person": rb.new_person,
        "organization": lambda _, g: g.choice(rb.ORGS),
        "location": lambda _, g: g.choice(rb.PLACES),
    }
    swapped = [rb.swap(r, makers, rng) for r in entity]
    everyday = evaluate.load(ROOT / "data" / "everyday.jsonl")
    kinds = [*NAMES, "address", *STRUCTURED]
    groups = {"all": kinds} | {t: [t] for t in kinds}
    names = {"all names": NAMES} | {t: [t] for t in NAMES}
    report = [
        "# Presidio and this tool on the same data",
        "",
        "Share of gold spans that a predicted span overlaps, whatever its type: what a redaction would have",
        "covered. Presidio 2.2 runs with spaCy's large models for English, French and German and `xx_ent_wiki_sm`",
        "for Luxembourgish, organizations on, phone regions extended to Luxembourg and its neighbours, and is told",
        "the language of every text. This tool guesses the language and runs with `dates_and_times`, because",
        "Presidio reports every date and time. `rules + NER` uses GLiNER, `rules + fast NER` the small spaCy",
        "models, `rules + both` takes what either model finds.",
        "",
        "None of these sets was used to develop the rules they test: see `eval/README.md` for what was.",
        "",
    ]
    wiki = [r for lang in LANGS for r in wikiann_rows(lang, 300)]
    privacy = [r for lang in rb.AI4PRIVACY for r in ai4privacy_rows(lang, 120)]
    theirs, presidio_speed = presidio_predictions([entity, swapped, free, everyday, wiki, privacy])

    preds, speed = predictions(entity, theirs[0], timed=True)
    report += section("Eval split", "Most names here are in this tool's word lists.", covered(entity, preds, groups))
    preds, _ = predictions(swapped, theirs[1])
    note = "Persons, firms and towns replaced by ones written without looking at any word list."
    report += section("Eval split with other names", note, covered(swapped, preds, groups))
    lb = [i for i, r in enumerate(swapped) if r["lang"] == "lb"]
    lb_preds = {k: [v[i] for i in lb] for k, v in preds.items()}
    title = "Eval split with other names, Luxembourgish only"
    report += section(title, "", covered([swapped[i] for i in lb], lb_preds, groups))
    note = "Every hit that is no date or time is an over-redaction."
    preds, _ = predictions(free, theirs[2])
    report += section("Hits on the 80 entity-free sentences of the eval split", note, hits(free, preds))
    preds, _ = predictions(everyday, theirs[3])
    report += section("Hits on 120 everyday sentences", note, hits(everyday, preds))
    preds, _ = predictions(wiki, theirs[4])
    note = "Test split, silver labels for persons, organizations and locations."
    report += section("WikiANN, 300 Wikipedia sentences per language", note, by_language(wiki, preds, names))
    report += section("WikiANN, how much of what was marked is an entity", "", on_gold(wiki, preds))
    preds, _ = predictions(privacy, theirs[5])
    labels = sorted({s["type"] for r in privacy for s in r["spans"]})
    note = "English, French and German. Sex and title have no detector in this tool on purpose."
    title = "ai4privacy pii-masking-300k, 360 synthetic documents"
    report += section(title, note, by_language(privacy, preds, {"all": labels}))
    report += section("ai4privacy by label", "", covered(privacy, preds, {t: [t] for t in labels}))
    report += section("ai4privacy, how much of what was marked is labelled", "", on_gold(privacy, preds))
    speed = {"presidio": presidio_speed} | speed
    speed_df = pd.DataFrame([{"tool": k, "ms per 1,000 words": v} for k, v in speed.items()])
    note = "Two CPU cores, models loaded. Presidio text by text over all sets, this tool on the eval split in one call."
    report += section("Runtime", note, speed_df)
    out = ROOT / "results" / "presidio.md"
    out.write_text("\n".join(report), encoding="utf-8", newline="\n")
    print("\n".join(report))


if __name__ == "__main__":
    main()
