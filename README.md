# tagging

Finds and redacts personal data (names, places, organisations, emails, phone numbers, IBANs, card numbers, national IDs, passport and licence numbers, passwords, dates of birth, addresses, URLs, IP addresses) in English, French, German and Luxembourgish text. A Rust core with rules, word lists, a small name grammar and a form-field reader runs in milliseconds. Two optional NER models add the names that no list knows: a fast one (spaCy) and an accurate one (GLiNER).

**Try it in your browser:** [yannkleees.github.io/tagging](https://yannkleees.github.io/tagging/). The demo runs the Rust core compiled to WebAssembly; the NER models need the Python package.

## Result

On a held-out set of 288 synthetic sentences (72 per language, including hard cases such as `d'Lis`, `den Här Kieffer` and lowercase chat):

| | Luxembourgish hard names, recall | Precision | All types, F1 (per language and subset) | Hits on 80 entity-free sentences |
|---|---|---|---|---|
| Rules and word lists | 1.00 | 1.00 | 0.99–1.00 | 10 |
| NER model alone | 0.74 | 0.79 | 0.66–0.82 | 92 |
| Rules + fast NER | 0.96 | 1.00 | 0.97–1.00 | 13 |
| **Rules + NER + post-processing** | **1.00** | **1.00** | **0.99–1.00** | **13** |

These numbers are the ceiling of the test set, not of the task. The word lists know all places and organisations and 82% of the person names in it, and the sentences come from the same templates as the development split.

## How good it really is

[eval/results/robustness.md](eval/results/robustness.md) has the checks the eval split cannot make. Recall, with names written without looking at the word lists and with two external datasets:

| | Rules only | Rules + fast NER | Rules + NER |
|---|---|---|---|
| Person, original eval split | 0.98 | 0.97 | 1.00 |
| Person, unseen names | 0.54 | 0.80 | 0.92 |
| Organisation, unseen small firms | 0.06 | 0.35 | 0.81 |
| Location, unseen towns | 0.60 | 0.85 | 0.96 |
| Email, phone, IBAN, card, national ID, address (unseen names) | 1.00 | 1.00 | 1.00 |
| Persons in all-lowercase text | 0.72 | 0.82 | 0.98 |
| WikiANN, Wikipedia sentences in four languages, all entities | 0.48–0.57 | 0.75–0.80 | 0.70–0.81 |
| ai4privacy, synthetic letters and forms, all labelled items | 0.61–0.64 | 0.66–0.68 | 0.70–0.72 |

What that means:

- **Reliable:** checksum types (IBAN, card, the national numbers of Luxembourg, France, Belgium and Germany), emails, IP addresses, phone numbers and addresses in Luxembourg's formats, and labelled fields of a form (`Reisepass: …`, `"password": "…"`).
- **Good with a model:** person names, about 0.92 on names it has not seen with GLiNER and 0.80 with the fast model.
- **Weak:** organisations without a model, anything the rules alone must find without a list entry or a field label, and Luxembourgish names made of dictionary words.
- **Not measured:** real letters, emails and forms. Every number here comes from synthetic or Wikipedia text.

This is a portfolio project and a starting point, not a product to rely on for anonymisation. At 0.92 recall per name, a document with ten names leaks at least one more often than not.

## Against Microsoft Presidio

[Presidio](https://github.com/microsoft/presidio) is the open-source tool most people reach for. Both ran on the same data ([eval/results/presidio.md](eval/results/presidio.md)): Presidio with spaCy's large models, organisations switched on, phone regions extended to Luxembourg and its neighbours, and told the language of every text; this tool with `dates_and_times` on, because Presidio reports every date, and guessing the language itself. The numbers are the share of labelled items that a marked span covers, whatever type it was given. Dates and times do not count as wrong hits for either tool.

| | Presidio | Rules | + fast NER | + NER | + both models |
|---|---|---|---|---|---|
| Luxembourg-style sentences, unseen names, all types | 0.87 | 0.68 | 0.93 | 0.95 | **0.97** |
| The same, Luxembourgish only | 0.82 | 0.68 | 0.92 | **0.97** | **0.97** |
| ai4privacy letters and forms (en, fr, de) | 0.49–0.59 | 0.72–0.74 | 0.75–0.78 | 0.80–0.82 | **0.80–0.84** |
| WikiANN, English | 0.79 | 0.53 | 0.79 | 0.81 | **0.86** |
| WikiANN, French | **0.82** | 0.55 | 0.80 | 0.80 | **0.82** |
| WikiANN, German | **0.87** | 0.48 | 0.78 | 0.79 | 0.85 |
| WikiANN, Luxembourgish | **0.83** | 0.57 | 0.75 | 0.70 | 0.77 |
| Wrong hits on 80 entity-free sentences | 71 | **10** | 13 | 13 | 16 |
| Wrong hits on 120 everyday sentences | 115 | **2** | 9 | 7 | 12 |
| Marked spans that touch a labelled item, WikiANN | 0.82 | **0.97** | 0.95 | 0.94 | 0.92 |
| The same, ai4privacy | 0.77 | **0.93** | 0.85 | 0.89 | 0.83 |
| Seconds per 1,000 words, two CPU cores | 0.43 | **0.004** | 0.09 | 7.8 | 7.7 |

How to read this:

- **Ahead:** documents in Luxembourg's formats and languages, forms and identifiers, and above all over-redaction. With a model on, Presidio marks 5 to 16 times as many spans in text without personal data.
- **Level:** names in running English and French text, once a model is on, and ahead in English with both.
- **Behind:** names in German and Luxembourgish Wikipedia text. The filter that keeps `die Feuerwehr` and `d'Hiewan` from being redacted also drops organisations whose names are dictionary words. With GLiNER this tool is also 18 times slower than Presidio. The fast mode is five times faster when texts go through in one batch and about as fast one text at a time.
- **Not a fair fight everywhere.** The form-field and ID rules were developed on another sample of the ai4privacy dataset, so that row is home turf, and Presidio ran as configured above, without recognisers written for these data. On the other side, WikiANN is home turf for spaCy's models, which Presidio uses. Neither tool has been measured on real documents.

## Modes

| | Config | Finds | Seconds per 1,000 words |
|---|---|---|---|
| Rules | `None` | Everything with a format, a checksum, a list entry or a field label | 0.004 |
| Fast | `{"ner": {"backend": "spacy"}}` | Also most unseen person names | 0.06–0.1 |
| Accurate | `{"ner": True}` | Also unseen firms and names in lowercase text | 7–13 |
| Both | `{"ner": {"backend": "both"}}` | What either model finds | 7–13 |

`{"dates_and_times": True}` adds dates that are no birth dates and times of day. It is off by default: a date alone rarely identifies anyone.

## Example

Input: `Gëschter huet d'Tessy Nzuzi vum Atelier Dräieck ugeruff. D'Nzuzi wunnt 12, rue de la Gare zu Réimech.`

Rules only (Rust CLI, `--mode pseudo`): `Tessy` is a listed first name, so the capitalised word after it is taken as the surname. The firm and the surname on its own are in no list.

```
Gëschter huet d'[PERSON_1] vum Atelier Dräieck ugeruff. D'Nzuzi wunnt [ADDRESS_1] zu [LOCATION_1].
```

With a model (Python, `{"ner": True}` or the fast backend): the surname on its own is found. The firm is not: `Atelier` and `Dräieck` are both in the dictionary, and a name made of everyday Luxembourgish words is what the filter against over-redaction gives up.

```
Gëschter huet d'[PERSON_1] vum Atelier Dräieck ugeruff. D'[PERSON_1] wunnt [ADDRESS_1] zu [LOCATION_1].
```

Pseudonymisation keeps one placeholder per person, so `Tessy Nzuzi` and the later `Nzuzi` are both `[PERSON_1]`.

## How the rules find names

- **Word lists.** First names, places and organisations from Luxembourg (hand-curated and Wikidata), plus first names and family names of nine nationalities, countries, well-known cities, towns around Luxembourg and well-known organisations from Wikidata.
- **Name grammar.** A first name takes the surname after it (`Jhemp Kieffer`, `Marie-Josée de Kieffer`), a title marks the name after it (`den Här Feltgen`, `Dr. Weber`), forms put the surname first (`KIEFFER Jean`). A business word in front of a listed name (`Garage Weis`) or a company form behind capitalised words (`Muller & Fils S.à r.l.`) makes an organisation.
- **Everyday words.** `Rose`, `Orange`, `Berg` and `Essen` are names and words. A list of 190,000 everyday words, counted from Wikipedia text in the four languages and taken from the Lëtzebuerger Online Dictionnaire, decides which names need more evidence: a second name part, a title, or a cue word (`bei Orange`, `in Essen`). An indefinite article removes the doubt the other way (`eng Méck` is a mosquito).
- **Addresses.** Street and house number in French, German, Luxembourgish and English order, the town and postcode after them, and postcodes such as `L-1611`, `54290 Trier` or `LS10 3SL`.
- **Form fields.** A known field name, a separator and a value: `Reisepass: C4FH7K2P1`, `"Postcode": "EX31"`, `<Username>alex</Username>`, `**Vorname:** Rilon`. Field names are known in the four languages, for IDs, passwords, user names, names, places, address parts, phone numbers and birth dates.
- **ID numbers.** The French NIR, the Belgian national number and the German pension and tax numbers by their check digits, as cross-border workers carry them, plus codes shaped like passport and licence numbers (`KP2212837`). Order, invoice and serial numbers are left alone.

With a model on, the same list of everyday words drops model spans such as `die Feuerwehr`, `d'Hiewan` or `the Post Office`, and extra labels (`profession or role`, `building or facility`...) give GLiNER something better to call them than a person or an organisation.

## Stack

Rust (core detectors, word lists, name grammar, form fields, redaction, CLI) · PyO3 and maturin (Python bindings) · Python, GLiNER, transformers, spaCy (NER) · Faker, pandas (evaluation data and reports) · Wikidata SPARQL (names, places, organisations) · Wikipedia text via Hugging Face datasets and the Lëtzebuerger Online Dictionnaire (everyday words) · Presidio (comparison only)

## Run it

Requirements: Rust 1.85+ (edition 2024), Python 3.11+.

```
python -m venv .venv && .venv\Scripts\activate      # macOS/Linux: source .venv/bin/activate
pip install maturin
maturin develop --release                             # builds the Rust core into the venv
pip install -e ".[ner]"                               # optional: the accurate NER model (about 1 GB download on first use)
pip install -e ".[fast]"                              # optional: the fast NER model, then its four small pipelines:
python -m spacy download xx_ent_wiki_sm en_core_web_sm fr_core_news_sm de_core_news_sm
```

```python
import tagging

text = "Schreif dem Jhemp op jhemp@pt.lu."
spans = tagging.detect(text)                  # rules only
spans = tagging.detect(text, {"ner": True})   # rules + NER + post-processing
spans = tagging.detect(text, {"ner": {"backend": "spacy"}})  # the fast model instead
print(tagging.redact(text, spans, "pseudo"))  # or "mask"

many = tagging.detect_batch(["first text", "second text"], {"ner": True})  # one model pass, about twice as fast
```

The CLI works without Python: `cargo run --release -- detect file.txt` or `-- redact file.txt --mode mask`.

Tests: `cargo test` and `pytest`. Evaluation: `pip install -e ".[eval]"`, then `python eval/evaluate.py`, `python eval/robustness.py` and `python eval/compare_presidio.py` (see [eval/README.md](eval/README.md)).

## Data

The evaluation sentences are generated (Faker and hand-written templates) and contain no real personal data. Names, places and organisations come from Wikidata (CC0) and from hand-curated lists in `core/data/`; the list of everyday words in `tagging/common_words.txt` is counted from Wikipedia text (CC BY-SA) and extended with the Lëtzebuerger Online Dictionnaire (CC0). The external benchmarks are downloaded by the evaluation scripts and not redistributed. [core/data/README.md](core/data/README.md) describes how to rebuild each list.
