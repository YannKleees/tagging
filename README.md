# tagging

Finds and redacts personal data (names, places, organisations, emails, phone numbers, IBANs, card numbers, national IDs, dates of birth, addresses, URLs, IP addresses) in English, French, German and Luxembourgish text. A Rust core with rules, word lists and a small name grammar runs in milliseconds; an optional multilingual NER model (GLiNER) adds the names that no list knows.

**Try it in your browser:** [yannkleees.github.io/tagging](https://yannkleees.github.io/tagging/). The demo runs the Rust core compiled to WebAssembly; the NER model needs the Python package.

## Result

On a held-out set of 288 synthetic sentences (72 per language, including hard cases such as `d'Lis`, `den Här Kieffer` and lowercase chat):

| | Luxembourgish hard names, recall | Precision | All types, F1 (per language and subset) | Hits on 80 entity-free sentences |
|---|---|---|---|---|
| Rules and word lists | 1.00 | 1.00 | 0.99–1.00 | 9 |
| NER model alone | 0.74 | 0.79 | 0.66–0.82 | 92 |
| **Rules + NER + post-processing** | **1.00** | **1.00** | **0.99–1.00** | **13** |

These numbers are close to the ceiling of the test set, not of the task. The word lists now know all places and organisations and 82% of the person names in it, and the sentences come from the same templates as the development split. On real documents the rules will miss surnames without a first name or title, rare first names and small firms; that is what the model is for. See the caveats in [eval/README.md](eval/README.md).

Over-redaction is the other half. On 120 everyday sentences without any personal data (`The Plumber fixed the Sink`, `Die Feuerwehr war schnell da`), the model alone marks 151 spans, the full pipeline 14, of which 9 are Luxembourgish ([eval/results/over_redaction.md](eval/results/over_redaction.md)).

Rules alone take about 2.5 ms per 1,000 words; with the NER model 6 to 11 s per 1,000 words on two CPU cores ([eval/results/runtime.md](eval/results/runtime.md)). Details in [eval/README.md](eval/README.md) and [eval/results/comparison.md](eval/results/comparison.md).

## Example

Input: `Gëschter huet d'Tessy Nzuzi vum Atelier Dräieck ugeruff. D'Nzuzi wunnt 12, rue de la Gare zu Réimech.`

Rules only (Rust CLI, `--mode pseudo`): `Tessy` is a listed first name, so the capitalised word after it is taken as the surname. The firm and the surname on its own are in no list.

```
Gëschter huet d'[PERSON_1] vum Atelier Dräieck ugeruff. D'Nzuzi wunnt [ADDRESS_1] zu [LOCATION_1].
```

Rules + NER (Python, `{"ner": True}`): the model finds both, and takes one word too many after the firm.

```
Gëschter huet d'[PERSON_1] vum [ORGANIZATION_1]. D'[PERSON_1] wunnt [ADDRESS_1] zu [LOCATION_1].
```

Pseudonymisation keeps one placeholder per person, so `Tessy Nzuzi` and the later `Nzuzi` are both `[PERSON_1]`.

## How the rules find names

- **Word lists.** First names, places and organisations from Luxembourg (hand-curated and Wikidata), plus first names and family names of nine nationalities, countries, well-known cities, towns around Luxembourg and well-known organisations from Wikidata.
- **Name grammar.** A first name takes the surname after it (`Jhemp Kieffer`, `Marie-Josée de Kieffer`), a title marks the name after it (`den Här Feltgen`, `Dr. Weber`), forms put the surname first (`KIEFFER Jean`). A business word in front of a listed name (`Garage Weis`) or a company form behind capitalised words (`Muller & Fils S.à r.l.`) makes an organisation.
- **Everyday words.** `Rose`, `Orange`, `Berg` and `Essen` are names and words. A list of 130,000 everyday words, counted from Wikipedia text in the four languages, decides which names need more evidence: a second name part, a title, or a cue word (`bei Orange`, `in Essen`). An indefinite article removes the doubt the other way (`eng Méck` is a mosquito).
- **Addresses.** Street and house number in French, German, Luxembourgish and English order, and postcodes such as `L-1611`.

With the model on, the same list of everyday words drops model spans such as `Post Office` or `die Feuerwehr`, and extra labels (`profession or role`, `building or facility`...) give the model something better to call them than a person or an organisation.

## Stack

Rust (core detectors, word lists, name grammar, redaction, CLI) · PyO3 and maturin (Python bindings) · Python, GLiNER, transformers (NER) · Faker, pandas (evaluation data and reports) · Wikidata SPARQL (names, places, organisations) · Wikipedia text via Hugging Face datasets (everyday words)

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

many = tagging.detect_batch(["first text", "second text"], {"ner": True})  # one model pass, about twice as fast
```

The CLI works without Python: `cargo run --release -- detect file.txt` or `-- redact file.txt --mode mask`.

Tests: `cargo test` and `pytest`. Evaluation: `pip install -e ".[eval]"`, then `python eval/evaluate.py` (see [eval/README.md](eval/README.md)).

## Data

The evaluation sentences are generated (Faker and hand-written templates) and contain no real personal data. Names, places and organisations come from Wikidata (CC0) and from hand-curated lists in `core/data/`; the list of everyday words in `tagging/common_words.txt` is counted from Wikipedia text (CC BY-SA). [core/data/README.md](core/data/README.md) describes how to rebuild each list.
