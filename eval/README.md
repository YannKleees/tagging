# Evaluation

## Dataset

`data/eval.jsonl` holds 288 synthetic sentences: 72 per language (EN, FR, DE, LB), in three subsets.

| subset | per language | content |
| --- | --- | --- |
| `standard` | 26 | Common full names (Faker en_GB, fr_FR, de_DE plus Luxembourgish names), well-known organizations and places, every structured type. |
| `hard` | 26 | Luxembourgish short names (Jhemp, Mett, Méck), names after articles and titles (`d'Lis`, `den Här Kieffer`, `dem Mett säi`), coordinated names, lowercase chat, small localities in their native spelling (`Lellgen` / `Lellingen`), local organizations and unusual number formats. |
| `entity_free` | 20 | No personal data, but many capitalized nouns, kinship and role words (`D'Mamm`, `Bopa`, `Noper`, `Die Mutter`) and decoys: `Berg`, `Gare`, `Mett`, a mosquito (`Méck`), prices, order and version numbers. |

Every template instance is rendered in all four languages with the same values, so each `group`
is a set of parallel sentences and language effects can be compared directly. Place names use
each language's own spelling.

Gold spans cover the name only, never the article or title: `den Här Kieffer` is labeled
`Kieffer`, and `D'Mett an de Méck` gets two person spans. An address is the street and house
number. The city is a separate location.

Each line has this shape. `start` and `end` are UTF-8 byte offsets, like the tool's output.

```json
{"id": "lb-hard-coordinated-0", "group": "hard-coordinated-0", "lang": "lb", "subset": "hard",
 "text": "D'Mady an d'Tessy sinn op Lellgen gefuer.",
 "spans": [{"type": "person", "start": 2, "end": 6, "text": "Mady"}, ...]}
```

Rebuild it (seeded, so the output is identical) with:

```
pip install -e ".[eval]"
python eval/build_dataset.py
```

Span counts: person 216, location 80, organization 48, email 32, phone 32, date of birth 24,
IBAN 16, credit card 16, national ID 16, address 8, URL 8, IP address 8.

## Running

```
python eval/evaluate.py                                  # rules and gazetteers, default threshold 0.5
python eval/evaluate.py --config my.json --name variant  # any config, written to results/variant/
```

Each run writes `report.md`, `metrics.csv` (per language, subset and type), `missed.csv` and
`false_positives.csv` to `results/<name>/`.

The metrics:

- **overlap**: a prediction of the same type overlaps the gold span (one-to-one).
- **exact**: same type and identical offsets.
- **any-type recall**: some prediction overlaps the gold span, whatever its type. This is the
  share that would actually be redacted.

Recall is listed first, because a missed identifier leaks data while an over-redaction only costs
readability.

## Results: rules and gazetteers only

From [`results/rules/report.md`](results/rules/report.md):

| lang | subset | recall (overlap) | recall (exact) | precision | F1 (overlap) |
| --- | --- | --- | --- | --- | --- |
| en | standard | 0.59 | 0.50 | 1.00 | 0.74 |
| en | hard | 0.63 | 0.63 | 1.00 | 0.78 |
| fr | standard | 0.59 | 0.50 | 1.00 | 0.74 |
| fr | hard | 0.67 | 0.67 | 1.00 | 0.80 |
| de | standard | 0.59 | 0.50 | 1.00 | 0.74 |
| de | hard | 0.67 | 0.67 | 1.00 | 0.80 |
| lb | standard | 0.59 | 0.50 | 1.00 | 0.74 |
| lb | hard | 0.67 | 0.67 | 1.00 | 0.80 |

False positives on the entity-free subset: EN 1, FR 2, DE 2, LB 5. They are all names marked as
ambiguous in the gazetteer (`Mett`, `Méck`, `Berg`, `Bierg`, `Gare`, `Stroossen`) and score 0.55.

Per type:

- Every checksum or format-based type has 1.00 recall: email, URL, IP, phone, IBAN, card and
  national ID.
- Date of birth: 0.92. The two misses are the English abbreviation `b.`.
- Location: 0.80 and organization: 0.75. The misses are names outside the gazetteer (Paris,
  Amazon, Garage Weis).
- Person: 0.30. Rules cannot find surnames or most full names.
- Address: 0.00. No rule covers addresses.

Runtime is about 5 ms per 1,000 words.

What this shows:

- Rules are nearly language-independent, so parallel sentences score the same in every language.
  The differences come from context words: the English `b.` for born, and Luxembourgish
  articles and place spellings.
- The gaps (person, address, names missing from lists) are what the contextual model in Phase 3
  has to close.
- Raising the threshold to 0.6 (`results/rules_threshold_060/`) removes all 10 false positives,
  but hard-subset recall drops 7 to 9 points. With recall first, the default stays at 0.5, and
  the ambiguous gazetteer hits are left for the model to confirm or reject.
- 88% of gold locations, 80% of organizations and 32% of person names appear in the gazetteer,
  partly because the generator and the gazetteer draw on the same knowledge. Rules-only recall on
  names is optimistic for that reason.

## Caveats

- The sentences are template-generated. Real documents are longer, noisier and less regular.
- The first evaluation run found four rule bugs, which were fixed with new unit tests:
  - Verb-final birth dates (`am 03.02.1961 … geboren`).
  - The Luxembourgish `Versioun` before a version number.
  - `(+352)` written with parentheses.
  - `0352` written instead of `+352`.

  This lifted date-of-birth recall from 0.75 to 0.92 and exact phone matching from 0.75 to 1.00,
  and removed one false positive. Because those fixes were informed by this set, it is no longer
  fully held out for the rules. Later phases should tune on a separate development split.
