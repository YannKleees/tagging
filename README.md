# tagging

Finds and redacts personal data (names, places, organisations, emails, phone numbers, IBANs, card numbers, national IDs, dates of birth, addresses, URLs, IP addresses) in English, French, German and Luxembourgish text. A Rust core with rules and Luxembourg gazetteers runs in milliseconds; an optional multilingual NER model (GLiNER) adds the names that rules cannot know.

## Result

On a held-out set of 288 synthetic sentences (72 per language, including hard cases such as `d'Lis`, `den Här Kieffer` and lowercase chat):

| | Luxembourgish hard names, recall | Precision | All types, F1 (per language and subset) |
|---|---|---|---|
| Rules and gazetteers | 0.57 | 1.00 | 0.74–0.80 |
| NER model alone | 0.74 | 0.79 | 0.66–0.82 |
| **Rules + NER + post-processing** | **0.91** | **1.00** | **0.96–0.99** |

Rules alone take about 2 ms per 1,000 words; with the NER model about 15 s per 1,000 words on a laptop CPU. The combined setup also flags 4–10 of 20 entity-free sentences per language (capitalised nouns such as `Post Office`, `Director` or `Méck`, a mosquito), against 1–5 for rules alone: the filters were tuned on the dev split and do not fully carry over. Details in [eval/README.md](eval/README.md) and [eval/results/comparison.md](eval/results/comparison.md).

## Example

Input: `Moien, ech sinn de Jhemp Kieffer aus Ettelbréck. Schreif mer op jhemp.kieffer@pt.lu oder rouf um +352 621 555 123 un. Jhemp freet sech.`

Rules only (Rust CLI, `--mode pseudo`): the surname is not in any list.

```
Moien, ech sinn de [PERSON_1] Kieffer aus [LOCATION_1]. Schreif mer op [EMAIL_1] oder rouf um [PHONE_1] un. [PERSON_1] freet sech.
```

Rules + NER (Python, `{"ner": True}`):

```
Moien, ech sinn de [PERSON_1] aus [LOCATION_1]. Schreif mer op [EMAIL_1] oder rouf um [PHONE_1] un. [PERSON_1] freet sech.
```

Pseudonymisation keeps one placeholder per person, so `Jhemp Kieffer` and the later `Jhemp` are both `[PERSON_1]`.

## Stack

Rust (core detectors, gazetteers, redaction, CLI) · PyO3 and maturin (Python bindings) · Python, GLiNER, transformers (NER) · Faker, pandas (evaluation data and reports) · Wikidata SPARQL (Luxembourg place names)

## Run it

Requirements: Rust 1.85+ (edition 2024), Python 3.11+.

```
python -m venv .venv && .venv\Scripts\activate      # macOS/Linux: source .venv/bin/activate
pip install maturin
maturin develop --release                             # builds the Rust core into the venv
pip install -e ".[ner]"                               # optional: the NER model (about 1 GB download on first use)
```

```python
import tagging

text = "Schreif dem Jhemp op jhemp@pt.lu."
spans = tagging.detect(text)                  # rules only
spans = tagging.detect(text, {"ner": True})   # rules + NER + post-processing
print(tagging.redact(text, spans, "pseudo"))  # or "mask"
```

The CLI works without Python: `cargo run --release -- detect file.txt` or `-- redact file.txt --mode mask`.

Tests: `cargo test` and `pytest`. Evaluation: `pip install -e ".[eval]"`, then `python eval/evaluate.py` (see [eval/README.md](eval/README.md)).

## Data

The evaluation sentences are generated (Faker and hand-written templates) and contain no real personal data. Place names come from Wikidata (CC0); first names and organisations are hand-curated lists in `core/data/`.
