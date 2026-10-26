import argparse
import platform
import time
from pathlib import Path

import pandas as pd

import evaluate
import tagging

ROOT = Path(__file__).parent


def timed(texts, config, repeats):
    best = float("inf")
    for _ in range(repeats):
        t0 = time.perf_counter()
        for t in texts:
            tagging.detect(t, config)
        best = min(best, time.perf_counter() - t0)
    return best


def main():
    ap = argparse.ArgumentParser(description="Measure detection runtime per 1,000 words")
    ap.add_argument("--configs", nargs="+", default=["rules", "rules_ner_post"])
    ap.add_argument("--repeats", type=int, default=3)
    args = ap.parse_args()
    rows = evaluate.load(ROOT / "data" / "eval.jsonl")
    out = []
    for name in args.configs:
        config = tagging.read_config(ROOT / "configs" / f"{name}.json")
        tagging.detect("warm up model loading", config)
        for lang in ["en", "fr", "de", "lb"]:
            texts = [r["text"] for r in rows if r["lang"] == lang]
            doc = " ".join(texts)
            words = len(doc.split())
            # One long document is chunked into a few model calls; single sentences pay the per-call overhead.
            doc_s = timed([doc], config, args.repeats)
            sent_s = timed(texts, config, 1)
            out.append(
                {
                    "config": name,
                    "lang": lang,
                    "words": words,
                    "document_ms_per_1k_words": 1e6 * doc_s / words,
                    "sentences_ms_per_1k_words": 1e6 * sent_s / words,
                }
            )
    df = pd.DataFrame(out).set_index(["config", "lang"]).round(1)
    cpu = platform.processor() or platform.machine()
    report = [
        "# Runtime",
        "",
        (
            f"CPU only ({cpu}), best of {args.repeats} runs for documents. Each language's evaluation sentences "
            "are joined into one document of the given word count; the sentence column runs them one by one."
        ),
        "",
        evaluate.md(df, ["config", "lang", "words", "document_ms_per_1k_words", "sentences_ms_per_1k_words"]),
        "",
    ]
    path = ROOT / "results" / "runtime.md"
    path.write_text("\n".join(report), encoding="utf-8", newline="\n")
    print(df.to_string())
    print(f"written to {path}")


if __name__ == "__main__":
    main()
