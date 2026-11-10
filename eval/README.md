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

`data/everyday.jsonl` is a third, hand-written set: 120 sentences (30 per language) about plumbers,
landlords, museums and fire brigades, with no personal data at all. English and French nouns are
capitalized as in a heading, which is the hard case. It was written without looking at the
entity-free sentences of the eval split and measures over-redaction only.

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
python eval/over_redaction.py         # hits on the everyday sentences, results/over_redaction.md
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

## Results: rules and word lists only

From [`results/rules/report.md`](results/rules/report.md):

| lang | subset | recall (overlap) | recall (exact) | precision | F1 (overlap) |
| --- | --- | --- | --- | --- | --- |
| en | standard | 0.98 | 0.98 | 1.00 | 0.99 |
| en | hard | 1.00 | 1.00 | 1.00 | 1.00 |
| fr | standard | 0.98 | 0.98 | 1.00 | 0.99 |
| fr | hard | 1.00 | 1.00 | 1.00 | 1.00 |
| de | standard | 0.98 | 0.98 | 1.00 | 0.99 |
| de | hard | 1.00 | 1.00 | 1.00 | 1.00 |
| lb | standard | 0.98 | 0.98 | 1.00 | 0.99 |
| lb | hard | 1.00 | 1.00 | 1.00 | 1.00 |

Before the name grammar, the Wikidata lists and the address rules, recall was 0.59 to 0.67 in every
row and F1 0.74 to 0.80.

False positives on the entity-free subset: EN 1, FR 2, DE 2, LB 4 (before: 10 in total). Eight are
names marked as ambiguous that score 0.55: `Mett`, `Berg`, `Bierg`, `Gare`, `Stroossen`. `Méck`
after `eng` is gone, because an indefinite article now lowers an ambiguous name. One is new: `Noël`,
which is a first name and, with a capital, Christmas.

Per type:

- Every checksum or format-based type has 1.00 recall: email, URL, IP, phone, IBAN, card and
  national ID.
- Date of birth: 1.00 (before 0.92). The English abbreviation `b.` is now read as born.
- Address: 1.00 (before 0.00). Street and house number are a rule now.
- Location and organization: 1.00 (before 0.80 and 0.75). Paris, Amazon and Garage Weis are found:
  the first two by the Wikidata lists, the third by the rule for a business word in front of a
  listed name.
- Person: 0.98 (before 0.30). The four misses are one name, `Cilly Noack`, of which neither part is
  in a list.

Runtime is about 2.5 ms per 1,000 words, against 0.8 ms before on the same machine. Most of the
difference is the address patterns.

What this shows, and what it does not:

- The rules are no longer the weak half on this dataset. They are still nearly language-independent,
  so parallel sentences score the same in every language.
- **The dataset is saturated.** 100% of gold locations and organizations and 82% of person names
  are in a list (before: 88%, 80% and 32%), and a first name or a title in front of the others
  does the rest. None of the lists was built from the generator: names, places and organizations
  come from Wikidata. But the generator draws on frequent real names, and Wikidata knows frequent
  real names. A document with a rare surname on its own, a nickname or a two-person firm has
  nothing for the rules to hold on to.
- The eval split shares its sentence templates with the dev split; only the names and numbers
  differ. Rules written for the dev sentences (`105, boulevard Royal`, `Dr Ries and Mr Weber`)
  therefore fit the eval sentences by construction. The eval numbers confirm that nothing broke.
  They do not estimate recall on real text.

## Results: rules plus contextual NER

The model is GLiNER `urchade/gliner_multi_pii-v1` with descriptive labels. All tuning used the dev
split `data/dev.jsonl` (248 sentences from its own seed) and the everyday sentences. The eval split
was run once, after the code was frozen. Full tables are in
[`results/comparison.md`](results/comparison.md).

| variant | what runs |
| --- | --- |
| `rules` | Rules and word lists. |
| `ner_only` | GLiNER alone, threshold 0.5, the four entity labels, raw output. |
| `rules_ner` | Rules, word lists and GLiNER merged by precedence, threshold 0.5, no post-processing. |
| `rules_ner_post` | Adds the distractor labels and the post-processing: strip articles and titles, drop kinship, role and generic nouns, drop spans made of everyday words, split coordinated names. Ambiguous list hits need the model to agree. Threshold 0.5. |
| `rules_ner_post_tuned` | The package default: the same with threshold 0.4. |

### Luxembourgish, names only

Person, organization, location and address spans. False positives are on the 20 Luxembourgish
entity-free sentences.

| variant | hard recall (overlap) | hard recall (exact) | hard precision | standard recall (overlap) | false positives |
| --- | --- | --- | --- | --- | --- |
| `rules` | 1.00 | 1.00 | 1.00 | 0.98 | 4 |
| `ner_only` | 0.74 | 0.30 | 0.79 | 0.98 | 23 |
| `rules_ner` | 0.93 | 0.50 | 0.90 | 1.00 | 27 |
| `rules_ner_post` | 1.00 | 1.00 | 1.00 | 1.00 | 6 |
| `rules_ner_post_tuned` | 1.00 | 1.00 | 1.00 | 1.00 | 6 |

The default config stood at 0.91 hard recall, 0.89 exact and 13 false positives before.

### All entity types, recall (overlap)

| lang | subset | `rules` | `rules_ner` | `rules_ner_post` | `rules_ner_post_tuned` | precision (tuned) |
| --- | --- | --- | --- | --- | --- | --- |
| en | standard | 0.98 | 1.00 | 1.00 | 1.00 | 1.00 |
| en | hard | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| fr | standard | 0.98 | 1.00 | 1.00 | 1.00 | 1.00 |
| fr | hard | 1.00 | 0.98 | 0.98 | 0.98 | 1.00 |
| de | standard | 0.98 | 1.00 | 1.00 | 1.00 | 1.00 |
| de | hard | 1.00 | 1.00 | 0.98 | 0.98 | 1.00 |
| lb | standard | 0.98 | 1.00 | 1.00 | 1.00 | 1.00 |
| lb | hard | 1.00 | 0.95 | 1.00 | 1.00 | 1.00 |

Two spans are missed by the default config, both ambiguous names that the rules alone keep and the
model does not confirm: `Tun` in `Meine Mutter und Tun kommen` and the supermarket `Cactus` in
`Madame Schiltz de Cactus`. That is the price of asking for the model's agreement on ambiguous
list hits.

### False positives on entity-free sentences (20 per language)

| variant | en | fr | de | lb | total |
| --- | --- | --- | --- | --- | --- |
| `rules` | 1 | 2 | 2 | 4 | 9 |
| `ner_only` | 27 | 23 | 19 | 23 | 92 |
| `rules_ner` | 27 | 25 | 21 | 27 | 100 |
| `rules_ner_post` | 3 | 3 | 1 | 6 | 13 |
| `rules_ner_post_tuned` | 3 | 3 | 1 | 6 | 13 |

Before, the default config made 36. The 13 that remain, from
[`results/rules_ner_post_tuned/report.md`](results/rules_ner_post_tuned/report.md): the decoys
`Mett` and `Méck` where the model insists (`Eng Méck`, `Moies Mett`), `Noël`, `The Council`, plural
street words tagged as addresses (`Streets`, `Rues`, `Straßen`), and Luxembourgish phrases the
model reads as a name or a firm (`Gaart Fussball`, `de Pak an der Gare`).

### Over-redaction on everyday sentences

From [`results/over_redaction.md`](results/over_redaction.md), 30 sentences per language:

| variant | en | fr | de | lb | total |
| --- | --- | --- | --- | --- | --- |
| `rules` | 1 | 0 | 0 | 1 | 2 |
| `ner_only` | 41 | 40 | 34 | 36 | 151 |
| `rules_ner` | 42 | 40 | 34 | 37 | 153 |
| `rules_ner_post_tuned` | 3 | 2 | 0 | 9 | 14 |

The previous default config made 174 hits here: the hand-written filter lists covered the dev
sentences and little else. Two changes account for the drop, and both were measured on this set and
the dev split, not on the eval split:

- **Distractor labels.** The model is also asked for `profession or role`, `relative`,
  `building or facility`, `object` and `group of people`, and those spans are thrown away. A
  plumber then competes for the label `profession or role` and no longer has to be a person or an
  organization. Alone this cut the hits from 174 to 95 and raised dev recall from 0.968 to 0.978.
- **Everyday words.** `tagging/common_words.txt` lists 130,000 words that Wikipedia text in the
  four languages writes in lowercase or, for German and Luxembourgish nouns, after a determiner. A
  model span made only of such words is dropped unless a word list knows a name in it.

Luxembourgish keeps the most hits (`Klempner`, `Hiewan`, `Coiffer`, `Déierendoktesch`). The
Luxembourgish Wikipedia is small and encyclopedic, so everyday nouns are missing from the list, and
Luxembourgish puts the definite article before first names too, which rules out the cheapest signal
for a noun.

### Runtime

From [`results/runtime.md`](results/runtime.md), on two cores of a cloud CPU with no GPU:

- Rules only: 2.5 to 2.8 ms per 1,000 words.
- Rules plus NER: 6 to 11 s per 1,000 words for one long document, 15 to 17 s when every sentence is
  a separate `detect` call, and 7 to 8 s when the same sentences go through `detect_batch`.

These figures replace the laptop measurements of the earlier version and are not comparable with
them. The model dominates the cost. A GPU should be much faster, but this has not been measured.

### What this shows

- With the rules as strong as they are on this dataset, the model adds little recall here: one
  more name, `Cilly Noack`, in its four language versions. Its value lies in names that no list and no cue word reaches,
  and this dataset has too few of those to measure it. A new hard subset with unlisted surnames on
  their own and small firms would be the next step for the evaluation.
- Post-processing still decides exact boundaries. Without it, Luxembourgish hard exact recall is
  0.50 against 1.00 with it.
- The model is the source of nearly all over-redaction, 92 hits on 80 entity-free sentences on its
  own. Distractor labels and the everyday-word filter bring the pipeline to 13, but not to the 9 of
  the rules alone.
- Thresholds 0.4 and 0.5 give the same result on the eval split. The default stays at 0.4, chosen on
  the dev split.

## Caveats

- The sentences are template-generated. Real documents are longer, noisier and less regular.
- Dev and eval share templates and differ in names and numbers only, and the word lists cover most
  of those names. Both splits are close to their ceiling and can no longer rank improvements to
  name detection.
- `everyday.jsonl` was written by the same hand that tuned the filters against it. It is a
  development set for over-redaction, not a held-out one. The entity-free subset of the eval split
  is the held-out measure: 36 false positives before, 13 now.
- The first evaluation run of the rules found four rule bugs, which were fixed with new unit tests:
  - Verb-final birth dates (`am 03.02.1961 … geboren`).
  - The Luxembourgish `Versioun` before a version number.
  - `(+352)` written with parentheses.
  - `0352` written instead of `+352`.

  Because those fixes were informed by the eval split, it was never fully held out for the rules.
