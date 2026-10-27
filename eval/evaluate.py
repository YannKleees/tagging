import argparse
import json
import time
from pathlib import Path

import pandas as pd

import tagging

ROOT = Path(__file__).parent
NAME_TYPES = {"person", "location", "organization"}
CONTEXT_TYPES = NAME_TYPES | {"address"}


def load(path):
    return [json.loads(x) for x in Path(path).read_text(encoding="utf-8").splitlines() if x.strip()]


def overlaps(a, b):
    return a["start"] < b["end"] and b["start"] < a["end"]


# Greedy one-to-one matching of same-type spans, returning {gold index: pred index}.
def match(gold, pred, mode):
    hits, used = {}, set()
    for i, g in enumerate(gold):
        for j, p in enumerate(pred):
            if j in used or p["type"] != g["type"]:
                continue
            ok = (p["start"], p["end"]) == (g["start"], g["end"]) if mode == "exact" else overlaps(p, g)
            if ok:
                hits[i] = j
                used.add(j)
                break
    return hits


def score(rows, preds):
    gold_rows, pred_rows = [], []
    for r, pred in zip(rows, preds):
        gold = r["spans"]
        exact, overlap = match(gold, pred, "exact"), match(gold, pred, "overlap")
        meta = {"id": r["id"], "lang": r["lang"], "subset": r["subset"], "sentence": r["text"]}
        for i, g in enumerate(gold):
            # Redacted at all, whatever type the tool assigned.
            covered = any(overlaps(g, p) for p in pred)
            gold_rows.append(
                {
                    **meta,
                    "type": g["type"],
                    "text": g["text"],
                    "exact": i in exact,
                    "overlap": i in overlap,
                    "covered": covered,
                }
            )
        exact_p, overlap_p = set(exact.values()), set(overlap.values())
        for j, p in enumerate(pred):
            pred_rows.append(
                {
                    **meta,
                    "type": p["type"],
                    "text": p["text"],
                    "score": p["score"],
                    "exact": j in exact_p,
                    "overlap": j in overlap_p,
                }
            )
    return pd.DataFrame(gold_rows), pd.DataFrame(pred_rows)


def metrics(gold, pred, keys):
    g = gold.groupby(keys).agg(
        n_gold=("overlap", "size"), tp_exact=("exact", "sum"), tp_overlap=("overlap", "sum"), covered=("covered", "sum")
    )
    p = pred.groupby(keys).agg(n_pred=("overlap", "size"))
    df = g.join(p, how="outer").fillna(0)
    for mode in ("overlap", "exact"):
        tp = df[f"tp_{mode}"]
        df[f"recall_{mode}"] = tp / df["n_gold"]
        df[f"precision_{mode}"] = tp / df["n_pred"]
        df[f"f1_{mode}"] = 2 * tp / (df["n_gold"] + df["n_pred"])
    df["any_type_recall"] = df["covered"] / df["n_gold"]
    return df.astype({c: int for c in ["n_gold", "n_pred", "tp_exact", "tp_overlap", "covered"]})


def fmt(x):
    if isinstance(x, float):
        return "–" if pd.isna(x) else f"{x:.2f}"
    return str(x)


def md(df, cols):
    df = df.reset_index()
    lines = ["| " + " | ".join(cols) + " |", "|" + "---|" * len(cols)]
    lines += ["| " + " | ".join(fmt(x) for x in row) + " |" for row in df[cols].itertuples(index=False)]
    return "\n".join(lines)


def gazetteer_coverage(gold):
    names = gold[gold["type"].isin(NAME_TYPES)].drop_duplicates(["type", "text"])
    hits = [
        any(s["type"] == t for s in tagging.detect(x, {"detectors": ["gazetteer"], "threshold": 0}))
        for t, x in zip(names["type"], names["text"])
    ]
    return names.assign(in_gazetteer=hits).groupby("type")["in_gazetteer"].agg(["size", "sum", "mean"])


# Reports are committed, so config paths are written relative to the repository root.
def shown(path):
    path, root = Path(path).resolve(), ROOT.parent.resolve()
    return path.relative_to(root).as_posix() if path.is_relative_to(root) else path.name


def run(dataset, config, name):
    rows = load(dataset)
    tagging.detect("warm up model loading", config)
    t0 = time.perf_counter()
    preds = [tagging.detect(r["text"], config) for r in rows]
    elapsed = time.perf_counter() - t0
    words = sum(len(r["text"].split()) for r in rows)
    gold, pred = score(rows, preds)
    entity_spans = gold[gold["subset"] != "entity_free"]

    summary = metrics(gold, pred, ["lang", "subset"])
    names = metrics(
        entity_spans[entity_spans["type"].isin(CONTEXT_TYPES)],
        pred[(pred["subset"] != "entity_free") & pred["type"].isin(CONTEXT_TYPES)],
        ["lang", "subset"],
    )
    by_type = metrics(entity_spans, pred[pred["subset"] != "entity_free"], ["type"])
    by_lang_type = metrics(entity_spans, pred[pred["subset"] != "entity_free"], ["lang", "type"])
    full = metrics(gold, pred, ["lang", "subset", "type"])
    false_pos = pred[pred["subset"] == "entity_free"]
    fp_by_lang = false_pos.groupby("lang").size().reindex(["en", "fr", "de", "lb"], fill_value=0)
    free_counts = pd.Series([r["lang"] for r in rows if r["subset"] == "entity_free"]).value_counts()
    missed = gold[~gold["overlap"]][["id", "type", "text", "covered", "sentence"]]
    coverage = gazetteer_coverage(entity_spans)

    out = ROOT / "results" / name
    out.mkdir(parents=True, exist_ok=True)
    full.to_csv(out / "metrics.csv")
    missed.to_csv(out / "missed.csv", index=False)
    false_pos[["id", "type", "text", "score", "sentence"]].to_csv(out / "false_positives.csv", index=False)

    main_cols = [
        "recall_overlap",
        "recall_exact",
        "any_type_recall",
        "precision_overlap",
        "f1_overlap",
        "f1_exact",
        "n_gold",
        "n_pred",
    ]
    report = [
        f"# Evaluation: {name}",
        "",
        (
            f"Config: `{json.dumps(config) if not isinstance(config, (str, Path)) else shown(config)}`. "
            f"{len(rows)} sentences, {words} words, {len(entity_spans)} gold spans."
        ),
        "",
        "Recall comes first: a missed identifier leaks personal data, an over-redaction only costs readability.",
        (
            "`overlap` counts a prediction of the right type that overlaps the gold span, `exact` needs identical "
            "offsets, and `any_type_recall` counts a gold span as redacted if any prediction overlaps it."
        ),
        "",
        f"Runtime: {1000 * elapsed / words * 1000:.2f} ms per 1,000 words (detection only).",
        "",
        "## By language and subset",
        "",
        md(summary[summary.index.get_level_values("subset") != "entity_free"], ["lang", "subset"] + main_cols),
        "",
        "## Names only (person, organization, location, address)",
        "",
        md(
            names,
            ["lang", "subset", "recall_overlap", "recall_exact", "precision_overlap", "f1_overlap", "n_gold", "n_pred"],
        ),
        "",
        "## False positives on the entity-free subset",
        "",
        "| lang | false positives | sentences |",
        "|---|---|---|",
        *[f"| {lang} | {n} | {free_counts[lang]} |" for lang, n in fp_by_lang.items()],
        "",
        md(false_pos.assign(sentence=false_pos["sentence"].str.slice(0, 70)), ["id", "type", "text", "score"])
        if len(false_pos)
        else "None.",
        "",
        "## By entity type",
        "",
        md(by_type, ["type"] + main_cols),
        "",
        "## By language and entity type (overlap)",
        "",
        md(by_lang_type, ["lang", "type", "recall_overlap", "precision_overlap", "f1_overlap", "n_gold", "n_pred"]),
        "",
        "## Gold names found in the gazetteer",
        "",
        (
            "Share of distinct gold person, location and organization strings that the built-in gazetteer contains. "
            "High values mean rules-only recall on names partly reflects list overlap with the generator."
        ),
        "",
        md(
            coverage.rename(columns={"size": "distinct", "sum": "in_gazetteer", "mean": "share"}),
            ["type", "distinct", "in_gazetteer", "share"],
        ),
        "",
        f"## Missed spans ({len(missed)})",
        "",
        md(missed.assign(sentence=missed["sentence"].str.slice(0, 80)), ["id", "type", "text", "covered", "sentence"])
        if len(missed)
        else "None.",
        "",
    ]
    (out / "report.md").write_text("\n".join(report), encoding="utf-8", newline="\n")
    print(md(summary[summary.index.get_level_values("subset") != "entity_free"], ["lang", "subset"] + main_cols))
    print(f"\nentity-free false positives: {fp_by_lang.astype(int).to_dict()}")
    print(f"report written to {out / 'report.md'}")
    return {
        "summary": summary,
        "names": names,
        "false_positives": fp_by_lang,
        "entity_free_sentences": free_counts,
        "ms_per_1k_words": 1e6 * elapsed / words,
    }


def main():
    ap = argparse.ArgumentParser(description="Evaluate tagging on the labeled synthetic dataset")
    ap.add_argument("--data", default=ROOT / "data" / "eval.jsonl")
    ap.add_argument("--config", help="JSON config file; defaults to the built-in rules and gazetteers")
    ap.add_argument("--name", default="rules")
    args = ap.parse_args()
    run(args.data, args.config, args.name)


if __name__ == "__main__":
    main()
