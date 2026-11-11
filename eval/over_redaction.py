import argparse
import json
from pathlib import Path

import pandas as pd

import evaluate
import tagging

ROOT = Path(__file__).parent
LANGS = ["en", "fr", "de", "lb"]


# Every hit on data/everyday.jsonl is an over-redaction: the sentences name no person, place or
# organization, only jobs, relatives, buildings and things, capitalized as in a heading.
def main():
    ap = argparse.ArgumentParser(description="Count hits on everyday sentences without personal data")
    ap.add_argument(
        "--configs",
        nargs="+",
        default=["rules", "ner_only", "rules_ner", "rules_fast", "rules_both", "rules_ner_post_tuned"],
    )
    args = ap.parse_args()
    rows = evaluate.load(ROOT / "data" / "everyday.jsonl")
    counts, hits = {}, []
    for name in args.configs:
        config = tagging.read_config(ROOT / "configs" / f"{name}.json")
        preds = tagging.detect_batch([r["text"] for r in rows], config)
        found = [
            {"config": name, "lang": r["lang"], **s, "sentence": r["text"]} for r, p in zip(rows, preds) for s in p
        ]
        counts[name] = pd.Series([h["lang"] for h in found]).value_counts().reindex(LANGS, fill_value=0)
        hits += found if name == args.configs[-1] else []
    table = pd.DataFrame(counts).T.assign(total=lambda d: d.sum(axis=1)).rename_axis("config")
    last = pd.DataFrame(hits, columns=["lang", "type", "text", "score", "source", "sentence"])
    report = [
        "# Over-redaction on everyday sentences",
        "",
        f"{len(rows)} sentences, {len(rows) // len(LANGS)} per language, without any personal data. Every hit is wrong.",
        "",
        evaluate.md(table, ["config", *LANGS, "total"]),
        "",
        f"## Remaining hits of `{args.configs[-1]}`",
        "",
        evaluate.md(last, ["lang", "type", "text", "score", "source", "sentence"]) if len(last) else "None.",
        "",
    ]
    out = ROOT / "results" / "over_redaction.md"
    out.write_text("\n".join(report), encoding="utf-8", newline="\n")
    print(table.to_string())
    print(json.dumps({"written": str(out)}))


if __name__ == "__main__":
    main()
