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

With the models installed (`pip install -e ".[ner,fast,eval]"` and the four spaCy pipelines named in
the main README; the Presidio comparison also needs `presidio-analyzer` and spaCy's large English,
French and German pipelines):

```
python eval/compare.py --split dev    # tuning runs, written to results/dev/
python eval/compare.py --split eval   # all variants on the held-out split, results/comparison.md
python eval/over_redaction.py         # hits on the everyday sentences, results/over_redaction.md
python eval/benchmark.py              # runtime per 1,000 words, results/runtime.md
python eval/robustness.py             # unseen names, damaged text, external data, results/robustness.md
python eval/compare_presidio.py       # the same data through Microsoft Presidio, results/presidio.md
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

False positives on the entity-free subset: EN 1, FR 3, DE 2, LB 4. Eight are names marked as
ambiguous that score 0.55: `Mett`, `Berg`, `Bierg`, `Gare`, `Stroossen`. `Méck` after `eng` is
gone, because an indefinite article now lowers an ambiguous name. Two came with the larger lists:
`Noël`, which is a first name and, with a capital, Christmas, and `La Réunion`, which is an island
and, with two capitals as the decoy sentence writes it, a meeting.

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

Runtime is about 4 ms per 1,000 words, against 0.8 ms for the first version on the same machine.
The address patterns, the form-field reader and the ID shapes each cost about a millisecond.

The eval split holds no form fields, passports, passwords or foreign ID numbers, so the detectors
for those are not measured here at all. Their numbers are under Robustness and in the comparison
with Presidio.

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
| `rules_ner_post` | Adds the distractor labels and the post-processing: strip articles and titles, drop kinship, role and generic nouns, drop spans made of everyday words, cut spans that run on into the sentence, split coordinated names. Ambiguous list hits need the model to agree. Threshold 0.5. |
| `rules_ner_post_tuned` | The package default: the same with threshold 0.4. |
| `rules_fast` | The same pipeline with spaCy's small models in place of GLiNER: `{"ner": {"backend": "spacy"}}`. The language of each text is guessed from its function words; Luxembourgish and unclear texts go to spaCy's multilingual model. |
| `rules_both` | Both models, a span from either counts: `{"ner": {"backend": "both"}}`. |

### Luxembourgish, names only

Person, organization, location and address spans. False positives are on the 20 Luxembourgish
entity-free sentences.

| variant | hard recall (overlap) | hard recall (exact) | hard precision | standard recall (overlap) | false positives |
| --- | --- | --- | --- | --- | --- |
| `rules` | 1.00 | 1.00 | 1.00 | 0.98 | 4 |
| `ner_only` | 0.74 | 0.30 | 0.79 | 0.98 | 23 |
| `rules_ner` | 0.93 | 0.50 | 0.90 | 1.00 | 27 |
| `rules_ner_post` | 1.00 | 1.00 | 1.00 | 1.00 | 5 |
| `rules_ner_post_tuned` | 1.00 | 1.00 | 1.00 | 1.00 | 5 |
| `rules_fast` | 0.96 | 0.93 | 1.00 | 1.00 | 5 |
| `rules_both` | 1.00 | 0.98 | 1.00 | 1.00 | 6 |

The default config of the first version stood at 0.91 hard recall, 0.89 exact and 13 false
positives.

### All entity types, recall (overlap)

| lang | subset | `rules` | `rules_ner` | `rules_fast` | `rules_both` | `rules_ner_post_tuned` | precision (tuned) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| en | standard | 0.98 | 1.00 | 0.98 | 1.00 | 1.00 | 1.00 |
| en | hard | 1.00 | 1.00 | 0.98 | 1.00 | 1.00 | 1.00 |
| fr | standard | 0.98 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| fr | hard | 1.00 | 0.98 | 0.97 | 0.98 | 0.98 | 1.00 |
| de | standard | 0.98 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| de | hard | 1.00 | 1.00 | 0.97 | 0.97 | 0.98 | 1.00 |
| lb | standard | 0.98 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| lb | hard | 1.00 | 0.95 | 0.97 | 1.00 | 1.00 | 1.00 |

Two spans are missed by the default config, both ambiguous names that the rules alone keep and the
model does not confirm: `Tun` in `Meine Mutter und Tun kommen` and the supermarket `Cactus` in
`Madame Schiltz de Cactus`. That is the price of asking for the model's agreement on ambiguous
list hits. The fast model confirms fewer of them and misses `Tun` in all four languages.

### False positives on entity-free sentences (20 per language)

| variant | en | fr | de | lb | total |
| --- | --- | --- | --- | --- | --- |
| `rules` | 1 | 3 | 2 | 4 | 10 |
| `ner_only` | 27 | 23 | 19 | 23 | 92 |
| `rules_ner` | 27 | 25 | 21 | 27 | 100 |
| `rules_ner_post` | 3 | 4 | 1 | 5 | 13 |
| `rules_ner_post_tuned` | 3 | 4 | 1 | 5 | 13 |
| `rules_fast` | 3 | 3 | 2 | 5 | 13 |
| `rules_both` | 3 | 4 | 3 | 6 | 16 |

The default config of the first version made 36. The 13 that remain, from
[`results/rules_ner_post_tuned/report.md`](results/rules_ner_post_tuned/report.md): the decoys
`Mett` and `Méck` where the model insists (`Eng Méck`, `Moies Mett`), `Noël`, `La Réunion`,
`The Council`, plural street words tagged as addresses (`Streets`, `Rues`, `Straßen`), and
Luxembourgish phrases the model reads as a name or a firm (`Gaart Fussball`, `de Pak an der Gare`).
This count did not move with the later changes: the dictionary and the new filters took nothing
away here, and `La Réunion` took the place of a sentence the model no longer marks.

### Over-redaction on everyday sentences

From [`results/over_redaction.md`](results/over_redaction.md), 30 sentences per language:

| variant | en | fr | de | lb | total |
| --- | --- | --- | --- | --- | --- |
| `rules` | 1 | 0 | 0 | 1 | 2 |
| `ner_only` | 41 | 40 | 34 | 36 | 151 |
| `rules_ner` | 42 | 40 | 34 | 37 | 153 |
| `rules_fast` | 2 | 3 | 2 | 2 | 9 |
| `rules_both` | 4 | 4 | 2 | 2 | 12 |
| `rules_ner_post_tuned` | 3 | 3 | 0 | 1 | 7 |

The default config of the first version made 174 hits here: the hand-written filter lists covered
the dev sentences and little else. Four changes account for the drop. All were measured on this
set and the dev split, not on the eval split:

- **Distractor labels.** GLiNER is also asked for `profession or role`, `relative`,
  `building or facility`, `object` and `group of people`, and those spans are thrown away. A
  plumber then competes for the label `profession or role` and no longer has to be a person or an
  organization. Alone this cut the hits from 174 to 95 and raised dev recall from 0.968 to 0.978.
- **Everyday words.** `tagging/common_words.txt` lists the words that Wikipedia text in the four
  languages writes in lowercase or, for German and Luxembourgish nouns, after a determiner. A
  model span made only of such words is dropped unless a word list knows a name in it.
- **The Luxembourgish dictionary.** The Luxembourgish Wikipedia is small and encyclopedic and does
  not know `Hiewan`, `Coiffer` or `Kaffi`. Every word form of the Lëtzebuerger Online Dictionnaire
  that is no proper noun was added. Luxembourgish hits fell from 9 to 1.
- **Spans that run on.** `Musée mécht op Feierdeeg` is cut before the first lowercase everyday
  word that is no name particle, and what is left is judged like any other span.

The filter has a price, which WikiANN shows (see Robustness): an organization whose name is made
of dictionary words is dropped with the plumbers. Several capitalized words in a row survive in
English and French when no determiner precedes them (`National Water Well Association`), but not
in German and Luxembourgish, where every noun has a capital.

### Runtime

From [`results/runtime.md`](results/runtime.md), on two cores of a cloud CPU with no GPU:

- Rules only: 4 to 5 ms per 1,000 words.
- Rules plus the fast model: 55 to 85 ms per 1,000 words for one long document, 70 to 105 ms for
  sentences in one `detect_batch` call, 210 to 445 ms when every sentence is a separate call.
- Rules plus GLiNER: 7 to 13 s per 1,000 words for one long document, 16 to 18 s when every
  sentence is a separate `detect` call, and about 8 s when the same sentences go through
  `detect_batch`.

GLiNER dominates the cost of the accurate mode. A GPU should be much faster, but this has not
been measured.

### What this shows

- With the rules as strong as they are on this dataset, the model adds little recall here: one
  more name, `Cilly Noack`, in its four language versions. Its value lies in names that no list
  and no cue word reaches, and this dataset has too few of those to measure it. The Robustness
  section does.
- Post-processing still decides exact boundaries. Without it, Luxembourgish hard exact recall is
  0.50 against 1.00 with it.
- The models are the source of nearly all over-redaction, 92 hits on 80 entity-free sentences for
  GLiNER on its own. Distractor labels and the everyday-word filter bring the pipeline to 13, but
  not to the 10 of the rules alone.
- Thresholds 0.4 and 0.5 give the same result on the eval split. The default stays at 0.4, chosen on
  the dev split.
- The fast model costs one to three points of recall on the hard subsets and gives a hundredfold
  speed-up.

## Robustness

`robustness.py` answers what the eval split cannot: how much of the result is the word lists
knowing the test names. From [`results/robustness.md`](results/robustness.md):

| recall | `rules` | `rules_fast` | `rules_ner_post_tuned` |
| --- | --- | --- | --- |
| person, original eval split | 0.98 | 0.97 | 1.00 |
| person, 80 unseen first names and surnames | 0.54 | 0.80 | 0.92 |
| organization, 20 unseen small firms | 0.06 | 0.35 | 0.81 |
| location, 20 unseen towns | 0.60 | 0.85 | 0.96 |
| structured types and addresses, unseen names | 1.00 | 1.00 | 1.00 |
| person, text without diacritics | 0.86 | 0.89 | 0.98 |
| person, all lowercase | 0.72 | 0.82 | 0.98 |
| address, all lowercase | 0.00 | 0.00 | 1.00 |
| WikiANN, all entities, by language | 0.48 to 0.57 | 0.75 to 0.80 | 0.70 to 0.81 |
| WikiANN, organizations | 0.27 to 0.39 | 0.55 to 0.66 | 0.47 to 0.64 |
| ai4privacy, all labelled items, by language | 0.61 to 0.64 | 0.66 to 0.68 | 0.70 to 0.72 |

- The rules were overfitted to the eval names in the plain sense: with other names, person recall
  halves and organizations all but vanish. Of the unseen names, 48% of the first names, 68% of the
  surnames, 5% of the firms and 55% of the towns are in a list.
- The models are what generalizes. With GLiNER, unseen persons stay at 0.92 and firms at 0.81; the
  fast model keeps 0.80 of the persons and a third of the firms.
- The format-based detectors do not depend on names and keep 1.00.
- The address rule needs capitals. In lowercase text only GLiNER finds addresses.
- On Luxembourgish Wikipedia text the fast model is ahead of GLiNER (0.75 against 0.70). spaCy's
  multilingual model was trained on Wikipedia in many languages, Luxembourgish among them.
- ai4privacy rose from 0.16 to 0.20 for the rules and 0.39 to 0.43 with GLiNER in the first run of
  these checks to the numbers above. The gain comes from reading form fields (`Passport: …`), from
  ID and document number shapes, postcodes and phone fields. See below for which data they were
  developed on.
- The first run of these checks crashed: the address detector cut its search window inside a
  multi-byte character. That is fixed and has a regression test, and the rules have since made
  100,000 runs over texts in eleven languages, whole and cut at random places, without a crash.

None of this is real text. Letters, emails and forms from an actual organization, labelled by hand,
are the measurement that is still missing.

## Against Microsoft Presidio

`compare_presidio.py` runs [Presidio](https://github.com/microsoft/presidio) 2.2 and this tool on
the same data. Presidio is set up as a careful user would: spaCy's large models for English, French
and German, the multilingual `xx_ent_wiki_sm` for Luxembourgish, organizations switched on, phone
regions extended to Luxembourg and its neighbours, and the language of each text given. This tool
guesses the language and runs with `dates_and_times`, because Presidio reports every date and
time. A gold span counts as covered if any marked span overlaps it, whatever its type. From
[`results/presidio.md`](results/presidio.md):

| | Presidio | `rules` | `rules_fast` | `rules_ner_post_tuned` | `rules_both` |
| --- | --- | --- | --- | --- | --- |
| eval split, all types | 0.89 | 0.99 | 0.99 | 1.00 | 1.00 |
| eval split with unseen names, all types | 0.87 | 0.68 | 0.93 | 0.95 | 0.97 |
| the same, Luxembourgish only | 0.82 | 0.68 | 0.92 | 0.97 | 0.97 |
| the same, Luxembourg national ID | 0.25 | 1.00 | 1.00 | 1.00 | 1.00 |
| ai4privacy, by language | 0.49 to 0.59 | 0.72 to 0.74 | 0.75 to 0.78 | 0.80 to 0.82 | 0.80 to 0.84 |
| WikiANN English | 0.79 | 0.53 | 0.79 | 0.81 | 0.86 |
| WikiANN French | 0.82 | 0.55 | 0.80 | 0.80 | 0.82 |
| WikiANN German | 0.87 | 0.48 | 0.78 | 0.79 | 0.85 |
| WikiANN Luxembourgish | 0.83 | 0.57 | 0.75 | 0.70 | 0.77 |
| wrong hits, 80 entity-free eval sentences | 71 | 10 | 13 | 13 | 16 |
| wrong hits, 120 everyday sentences | 115 | 2 | 9 | 7 | 12 |
| marked spans on a labelled item, WikiANN | 0.82 | 0.97 | 0.95 | 0.94 | 0.92 |
| marked spans on a labelled item, ai4privacy | 0.77 | 0.93 | 0.85 | 0.89 | 0.83 |

Dates and times in the entity-free sentences are not counted as wrong hits for either tool.

Where this tool is ahead:

- **Luxembourg-style text.** With a model it covers 0.93 to 0.97 of the items against 0.87, and the
  rules alone find every matricule, phone number and birth date, where Presidio finds a quarter of
  the matricules.
- **Forms and identifiers.** On ai4privacy Presidio covers about half, this tool three quarters
  with rules alone. Presidio has no reader for labelled fields, and its recognizers for document
  numbers are mostly American.
- **Over-redaction.** With a model on, Presidio marks 5 to 16 times as many spans in text without
  personal data, and 18% of what it marks on Wikipedia sentences touches no labelled entity,
  against 3% to 8%.

Where Presidio is ahead:

- **Names in German and Luxembourgish running text.** 0.87 and 0.83 on WikiANN against 0.85 and
  0.77 with both models. The everyday-word filter is the reason: it drops what Presidio keeps,
  right and wrong alike.
- **Speed against the accurate mode.** Presidio takes 0.43 s per 1,000 words, one text at a time.
  GLiNER takes 7.8 s. The fast mode takes 0.09 s in one batch and 0.2 to 0.4 s one text at a time.
- **Breadth and maturity.** Presidio has recognizers for many countries' documents, an anonymizer
  with encryption and hashing, and years of use. This tool knows four languages and one region.

What the comparison does not show:

- ai4privacy is home turf for this tool's form rules (next section) and WikiANN for spaCy's
  models, which Presidio uses. The two sets pull in opposite directions.
- Presidio can be extended with custom recognizers, and nobody did that here.
- Neither tool was measured on real documents.

## Which data the later rules were developed on

The form-field reader, the ID shapes, the postcode and date extensions, the fast backend and the
refined filters were added after the first robustness run. To keep that run's test data usable:

- **ai4privacy.** Development used documents 2,001 to 2,150 of each shuffled validation file
  (seed 1). The reported sample is documents 1 to 120 of the same shuffle, which were never looked
  at. Development and reported coverage agree (0.63 and 0.61 to 0.64 for the rules), so the rules
  fit the dataset's format, not the sample. They do fit the format: a generator that writes
  `Passport Number: …` is easier than a real letter.
- **WikiANN.** Development used the validation split, the report uses the test split.
- **Unseen names.** The pools of names, firms and towns are those of the first run. No rule was
  written for them; the gain there comes from the fast model and from the larger place lists.
- **Over-redaction.** `everyday.jsonl` and the entity-free dev sentences were the development sets.
  The entity-free eval sentences were not used, with one exception: they showed that `La réunion`
  (a meeting, with a small r) matched the island, and the fix (every capitalized word of a list
  name needs its capital) is general. The capitalized decoy `La Réunion` still counts as a hit.

## Caveats

- The sentences are template-generated. Real documents are longer, noisier and less regular.
- Dev and eval share templates and differ in names and numbers only, and the word lists cover most
  of those names. Both splits are close to their ceiling and can no longer rank improvements to
  name detection.
- `everyday.jsonl` was written by the same hand that tuned the filters against it. It is a
  development set for over-redaction, not a held-out one. The entity-free subset of the eval split
  is the held-out measure: 36 false positives for the first version, 13 now.
- The form-field reader takes a label at its word. `Name: Quarterly figures` is rejected because
  the value does not look like a name, but `Name: Blue Horizon` (a boat) would be marked. No
  dataset here measures that.
- The first evaluation run of the rules found four rule bugs, which were fixed with new unit tests:
  - Verb-final birth dates (`am 03.02.1961 … geboren`).
  - The Luxembourgish `Versioun` before a version number.
  - `(+352)` written with parentheses.
  - `0352` written instead of `+352`.

  Because those fixes were informed by the eval split, it was never fully held out for the rules.
