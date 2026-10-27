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

With the model installed (`pip install -e ".[ner,eval]"`):

```
python eval/compare.py --split dev    # tuning runs, written to results/dev/
python eval/compare.py --split eval   # all variants on the held-out split, results/comparison.md
python eval/benchmark.py              # runtime per 1,000 words, results/runtime.md
TAGGING_NER_TESTS=1 pytest tests/test_ner_model.py
```

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

## Results: rules plus contextual NER

The model is GLiNER `urchade/gliner_multi_pii-v1` with descriptive labels. All tuning (filter
lists, threshold, the ambiguous-gazetteer rule) used the dev split `data/dev.jsonl`: 248
sentences from its own seed. The eval split was run once at the end. Full tables are in
[`results/comparison.md`](results/comparison.md).

| variant | what runs |
| --- | --- |
| `rules` | Rules and gazetteers. |
| `ner_only` | GLiNER alone, threshold 0.5, raw output. |
| `rules_ner` | Rules, gazetteers and GLiNER merged by precedence, threshold 0.5, no post-processing. |
| `rules_ner_post` | Adds post-processing: strip articles and titles, drop kinship, role and generic nouns, split coordinated names. Ambiguous gazetteer hits need the model to agree. Threshold 0.5. |
| `rules_ner_post_tuned` | The package default: the same with threshold 0.4. |

### Luxembourgish, names only

Person, organization, location and address spans. False positives are on the 20 Luxembourgish
entity-free sentences.

| variant | hard recall (overlap) | hard recall (exact) | hard precision | standard recall (overlap) | false positives |
| --- | --- | --- | --- | --- | --- |
| `rules` | 0.57 | 0.57 | 1.00 | 0.36 | 5 |
| `ner_only` | 0.74 | 0.30 | 0.79 | 0.98 | 23 |
| `rules_ner` | 0.87 | 0.43 | 0.89 | 0.98 | 27 |
| `rules_ner_post` | 0.91 | 0.89 | 1.00 | 0.98 | 10 |
| `rules_ner_post_tuned` | 0.91 | 0.89 | 1.00 | 0.98 | 13 |

Against the baseline from a quick test of the model alone (threshold 0.5, descriptive labels):

| | baseline | default config |
| --- | --- | --- |
| LB standard recall | 0.97 | 0.98 |
| LB hard recall | 0.82 | 0.91 |
| LB false positives | 9 on 8 sentences | 13 on 20 sentences |
| DE hard recall | 1.00 | 0.98 |
| DE false positives | 4 | 6 on 20 sentences |

The baseline came from a different, smaller set of sentences, so this comparison is only
indicative.

### All entity types, recall (overlap)

| lang | subset | `rules` | `rules_ner` | `rules_ner_post` | `rules_ner_post_tuned` | precision (tuned) |
| --- | --- | --- | --- | --- | --- | --- |
| en | standard | 0.59 | 0.97 | 0.97 | 0.97 | 1.00 |
| en | hard | 0.63 | 0.92 | 0.92 | 0.93 | 1.00 |
| fr | standard | 0.59 | 0.98 | 0.98 | 1.00 | 1.00 |
| fr | hard | 0.67 | 0.97 | 0.97 | 0.97 | 1.00 |
| de | standard | 0.59 | 0.98 | 0.98 | 0.98 | 0.98 |
| de | hard | 0.67 | 0.98 | 0.98 | 0.98 | 1.00 |
| lb | standard | 0.59 | 0.98 | 0.98 | 0.98 | 0.98 |
| lb | hard | 0.67 | 0.90 | 0.93 | 0.93 | 1.00 |

### False positives on entity-free sentences (20 per language)

| variant | en | fr | de | lb | total |
| --- | --- | --- | --- | --- | --- |
| `rules` | 1 | 2 | 2 | 5 | 10 |
| `ner_only` | 27 | 23 | 19 | 23 | 92 |
| `rules_ner` | 27 | 25 | 21 | 27 | 100 |
| `rules_ner_post` | 9 | 7 | 4 | 10 | 30 |
| `rules_ner_post_tuned` | 9 | 8 | 6 | 13 | 36 |

### Runtime

From [`results/runtime.md`](results/runtime.md), on a laptop CPU with no GPU:

- Rules only: 1.5 to 2.1 ms per 1,000 words.
- Rules plus NER: 14 to 18 s per 1,000 words for one long document, and 45 to 53 s when every
  sentence is a separate call, because each call pays the model's fixed overhead.

The model dominates the cost. A GPU should be much faster, but this has not been measured.

### What this shows

- The model closes the gaps the rules leave. Luxembourgish hard recall rises from 0.57 to 0.91
  for names. With the default config, recall across all types is 0.93 to 1.00 in every language
  and subset.
- Post-processing matters as much as the model. Stripping articles and titles lifts Luxembourgish
  hard exact recall from 0.43 to 0.89, so pseudonyms replace `Mett` and not `D'Mett`. The filters
  cut false positives by 70%, from 100 to 30.
- Luxembourgish is still the weakest language on hard sentences: 0.93 against 0.97 to 0.98 for
  French and German. The remaining misses are lowercase chat names (`ernest an gusty`) and names
  after the articles `den` and `der` (`den Tun`, `der Aline`).
- The dev split chose threshold 0.4: 2 more points of Luxembourgish hard recall for one extra
  false positive. On the eval split, 0.4 brings no Luxembourgish gain. English hard names rise
  from 0.93 to 0.96 and French standard from 0.98 to 1.00, at the cost of 6 more false positives.
  The default stays at 0.4, because recall comes first and changing it after seeing the eval split
  would tune on the test set.
- False positives were 0 on the dev split but 30 to 36 on the eval split, so the filter lists
  partly fit the dev sentences. Most of what remains are generic nouns the model labels as
  organizations or places: `Post Office`, `Winery`, `Kindergarten`, `The Council`, `Countryside`,
  `Le Train pour la Ville`. The rest are role phrases the filters miss (`Director`, `Paysan`,
  `Infirmière am Spidol`), plural street words (`Rues`, `Straßen`), and the decoys `Mett` (minced
  pork) and `Méck` (mosquito). These were left unpatched for the same reason, and they mean the tool
  over-redacts ordinary text.

## Caveats

- The sentences are template-generated. Real documents are longer, noisier and less regular.
- The first evaluation run found four rule bugs, which were fixed with new unit tests:
  - Verb-final birth dates (`am 03.02.1961 … geboren`).
  - The Luxembourgish `Versioun` before a version number.
  - `(+352)` written with parentheses.
  - `0352` written instead of `+352`.

  This lifted date-of-birth recall from 0.75 to 0.92 and exact phone matching from 0.75 to 1.00,
  and removed one false positive. Because those fixes were informed by this set, it is no longer
  fully held out for the rules. All Phase 3 tuning used the separate dev split.
