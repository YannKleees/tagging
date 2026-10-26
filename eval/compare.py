import argparse
import json
from pathlib import Path

import pandas as pd

import evaluate

ROOT = Path(__file__).parent
VARIANTS = {
    "rules": "rules.json",
    "ner_only": "ner_only.json",
    "rules_ner": "rules_ner.json",
    "rules_ner_post": "rules_ner_post.json",
}


def table(results, key, cols):
    frames = [r[key][cols].assign(variant=name) for name, r in results.items()]
    df = pd.concat(frames).reset_index().set_index(["lang", "subset", "variant"]).sort_index()
    return df.loc[[x for x in ["en", "fr", "de", "lb"] if x in df.index.get_level_values(0)]]


def main():
    ap = argparse.ArgumentParser(description="Compare detection variants on one dataset")
    ap.add_argument("--split", choices=["eval", "dev"], default="eval")
    ap.add_argument("--variants", nargs="+", default=list(VARIANTS), help="names in VARIANTS or config file paths")
    args = ap.parse_args()
    data = ROOT / "data" / f"{args.split}.jsonl"
    prefix = "" if args.split == "eval" else "dev/"
    results = {}
    for v in args.variants:
        path = ROOT / "configs" / VARIANTS[v] if v in VARIANTS else Path(v)
        name = v if v in VARIANTS else path.stem
        print(f"== {name}")
        results[name] = evaluate.run(data, path, prefix + name)

    cols = ["recall_overlap", "recall_exact", "precision_overlap", "f1_overlap"]
    overall = table(results, "summary", cols)
    names = table(results, "names", cols)
    overall = overall[overall.index.get_level_values("subset") != "entity_free"]
    fp = pd.DataFrame({name: r["false_positives"] for name, r in results.items()}).T
    sentences = next(iter(results.values()))["entity_free_sentences"]
    runtime = pd.Series({name: r["ms_per_1k_words"] for name, r in results.items()}, name="ms per 1,000 words")
    lb = names.loc[["lb"]] if "lb" in names.index.get_level_values(0) else names.iloc[:0]

    report = [
        f"# Variant comparison ({args.split} split)",
        "",
        "Variants: "
        + ", ".join(
            f"`{n}` ({json.dumps(evaluate.tagging.read_config(ROOT / 'configs' / VARIANTS[n]))})"
            if n in VARIANTS
            else f"`{n}`"
            for n in results
        )
        + ".",
        "",
        "## Luxembourgish, names only (person, organization, location, address)",
        "",
        evaluate.md(lb, ["lang", "subset", "variant"] + cols),
        "",
        "## False positives on entity-free sentences",
        "",
        evaluate.md(
            fp.assign(sentences_per_lang=int(sentences.iloc[0])).rename_axis("variant"),
            ["variant", "en", "fr", "de", "lb", "sentences_per_lang"],
        ),
        "",
        "## Runtime (CPU, detection only)",
        "",
        evaluate.md(runtime.round(1).to_frame().rename_axis("variant"), ["variant", "ms per 1,000 words"]),
        "",
        "## All entity types, by language and subset",
        "",
        evaluate.md(overall, ["lang", "subset", "variant"] + cols),
        "",
        "## Names only, by language and subset",
        "",
        evaluate.md(names, ["lang", "subset", "variant"] + cols),
        "",
    ]
    out = ROOT / "results" / prefix / "comparison.md"
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text("\n".join(report), encoding="utf-8", newline="\n")
    print(f"comparison written to {out}")


if __name__ == "__main__":
    main()
