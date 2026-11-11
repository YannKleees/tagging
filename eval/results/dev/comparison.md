# Variant comparison (dev split)

Variants: `rules` ({}), `ner_only` ({"detectors": [], "ner": {"threshold": 0.5, "postprocess": false, "confirm_ambiguous": false, "distractors": false}}), `rules_ner` ({"ner": {"threshold": 0.5, "postprocess": false, "confirm_ambiguous": false, "distractors": false}}), `rules_ner_post` ({"ner": {"threshold": 0.5}}), `rules_ner_post_tuned` ({"ner": true}), `rules_fast` ({"ner": {"backend": "spacy"}}), `rules_both` ({"ner": {"backend": "both"}}).

## Luxembourgish, names only (person, organization, location, address)

| lang | subset | variant | recall_overlap | recall_exact | precision_overlap | f1_overlap |
|---|---|---|---|---|---|---|
| lb | hard | ner_only | 0.67 | 0.26 | 0.91 | 0.78 |
| lb | hard | rules | 0.98 | 0.98 | 1.00 | 0.99 |
| lb | hard | rules_both | 1.00 | 0.98 | 1.00 | 1.00 |
| lb | hard | rules_fast | 0.98 | 0.98 | 1.00 | 0.99 |
| lb | hard | rules_ner | 0.96 | 0.57 | 0.96 | 0.96 |
| lb | hard | rules_ner_post | 1.00 | 1.00 | 1.00 | 1.00 |
| lb | hard | rules_ner_post_tuned | 1.00 | 0.98 | 1.00 | 1.00 |
| lb | standard | ner_only | 0.98 | 0.98 | 0.93 | 0.95 |
| lb | standard | rules | 0.98 | 0.98 | 1.00 | 0.99 |
| lb | standard | rules_both | 1.00 | 0.98 | 0.95 | 0.98 |
| lb | standard | rules_fast | 0.98 | 0.98 | 0.95 | 0.96 |
| lb | standard | rules_ner | 1.00 | 1.00 | 1.00 | 1.00 |
| lb | standard | rules_ner_post | 1.00 | 0.98 | 1.00 | 1.00 |
| lb | standard | rules_ner_post_tuned | 1.00 | 0.98 | 1.00 | 1.00 |

## False positives on entity-free sentences

| variant | en | fr | de | lb | sentences_per_lang |
|---|---|---|---|---|---|
| rules | 0 | 0 | 0 | 0 | 10 |
| ner_only | 12 | 11 | 7 | 7 | 10 |
| rules_ner | 12 | 11 | 7 | 7 | 10 |
| rules_ner_post | 0 | 0 | 0 | 0 | 10 |
| rules_ner_post_tuned | 0 | 0 | 0 | 0 | 10 |
| rules_fast | 0 | 0 | 0 | 0 | 10 |
| rules_both | 0 | 0 | 0 | 0 | 10 |

## Runtime (CPU, detection only)

| variant | ms per 1,000 words |
|---|---|
| rules | 10.00 |
| ner_only | 6043.00 |
| rules_ner | 6514.40 |
| rules_ner_post | 7720.80 |
| rules_ner_post_tuned | 7471.70 |
| rules_fast | 1030.90 |
| rules_both | 7537.40 |

## All entity types, by language and subset

| lang | subset | variant | recall_overlap | recall_exact | precision_overlap | f1_overlap |
|---|---|---|---|---|---|---|
| en | hard | ner_only | 0.68 | 0.47 | 0.93 | 0.79 |
| en | hard | rules | 0.98 | 0.98 | 1.00 | 0.99 |
| en | hard | rules_both | 1.00 | 0.97 | 1.00 | 1.00 |
| en | hard | rules_fast | 0.97 | 0.93 | 1.00 | 0.98 |
| en | hard | rules_ner | 0.98 | 0.77 | 0.95 | 0.97 |
| en | hard | rules_ner_post | 1.00 | 1.00 | 1.00 | 1.00 |
| en | hard | rules_ner_post_tuned | 1.00 | 1.00 | 1.00 | 1.00 |
| en | standard | ner_only | 0.61 | 0.58 | 0.98 | 0.75 |
| en | standard | rules | 0.98 | 0.98 | 1.00 | 0.99 |
| en | standard | rules_both | 1.00 | 1.00 | 0.97 | 0.99 |
| en | standard | rules_fast | 0.98 | 0.98 | 0.97 | 0.98 |
| en | standard | rules_ner | 1.00 | 1.00 | 1.00 | 1.00 |
| en | standard | rules_ner_post | 1.00 | 1.00 | 1.00 | 1.00 |
| en | standard | rules_ner_post_tuned | 1.00 | 1.00 | 1.00 | 1.00 |
| fr | hard | ner_only | 0.72 | 0.50 | 0.96 | 0.82 |
| fr | hard | rules | 0.98 | 0.98 | 1.00 | 0.99 |
| fr | hard | rules_both | 1.00 | 1.00 | 1.00 | 1.00 |
| fr | hard | rules_fast | 1.00 | 1.00 | 1.00 | 1.00 |
| fr | hard | rules_ner | 0.98 | 0.77 | 0.98 | 0.98 |
| fr | hard | rules_ner_post | 0.98 | 0.98 | 1.00 | 0.99 |
| fr | hard | rules_ner_post_tuned | 0.98 | 0.98 | 1.00 | 0.99 |
| fr | standard | ner_only | 0.62 | 0.62 | 0.95 | 0.75 |
| fr | standard | rules | 0.98 | 0.98 | 1.00 | 0.99 |
| fr | standard | rules_both | 0.98 | 0.98 | 0.96 | 0.97 |
| fr | standard | rules_fast | 0.98 | 0.98 | 0.96 | 0.97 |
| fr | standard | rules_ner | 1.00 | 1.00 | 1.00 | 1.00 |
| fr | standard | rules_ner_post | 1.00 | 1.00 | 1.00 | 1.00 |
| fr | standard | rules_ner_post_tuned | 1.00 | 0.98 | 1.00 | 1.00 |
| de | hard | ner_only | 0.70 | 0.48 | 0.93 | 0.80 |
| de | hard | rules | 0.98 | 0.98 | 1.00 | 0.99 |
| de | hard | rules_both | 1.00 | 0.97 | 1.00 | 1.00 |
| de | hard | rules_fast | 0.97 | 0.93 | 1.00 | 0.98 |
| de | hard | rules_ner | 0.98 | 0.77 | 0.97 | 0.98 |
| de | hard | rules_ner_post | 1.00 | 1.00 | 1.00 | 1.00 |
| de | hard | rules_ner_post_tuned | 1.00 | 1.00 | 1.00 | 1.00 |
| de | standard | ner_only | 0.62 | 0.62 | 0.95 | 0.75 |
| de | standard | rules | 0.98 | 0.98 | 1.00 | 0.99 |
| de | standard | rules_both | 1.00 | 0.98 | 1.00 | 1.00 |
| de | standard | rules_fast | 1.00 | 1.00 | 1.00 | 1.00 |
| de | standard | rules_ner | 1.00 | 1.00 | 1.00 | 1.00 |
| de | standard | rules_ner_post | 1.00 | 0.98 | 1.00 | 1.00 |
| de | standard | rules_ner_post_tuned | 1.00 | 0.98 | 1.00 | 1.00 |
| lb | hard | ner_only | 0.52 | 0.20 | 0.91 | 0.66 |
| lb | hard | rules | 0.98 | 0.98 | 1.00 | 0.99 |
| lb | hard | rules_both | 1.00 | 0.98 | 1.00 | 1.00 |
| lb | hard | rules_fast | 0.98 | 0.98 | 1.00 | 0.99 |
| lb | hard | rules_ner | 0.97 | 0.67 | 0.97 | 0.97 |
| lb | hard | rules_ner_post | 1.00 | 1.00 | 1.00 | 1.00 |
| lb | hard | rules_ner_post_tuned | 1.00 | 0.98 | 1.00 | 1.00 |
| lb | standard | ner_only | 0.62 | 0.62 | 0.93 | 0.75 |
| lb | standard | rules | 0.98 | 0.98 | 1.00 | 0.99 |
| lb | standard | rules_both | 1.00 | 0.98 | 0.97 | 0.99 |
| lb | standard | rules_fast | 0.98 | 0.98 | 0.97 | 0.98 |
| lb | standard | rules_ner | 1.00 | 1.00 | 1.00 | 1.00 |
| lb | standard | rules_ner_post | 1.00 | 0.98 | 1.00 | 1.00 |
| lb | standard | rules_ner_post_tuned | 1.00 | 0.98 | 1.00 | 1.00 |

## Names only, by language and subset

| lang | subset | variant | recall_overlap | recall_exact | precision_overlap | f1_overlap |
|---|---|---|---|---|---|---|
| en | hard | ner_only | 0.89 | 0.61 | 0.93 | 0.91 |
| en | hard | rules | 0.98 | 0.98 | 1.00 | 0.99 |
| en | hard | rules_both | 1.00 | 0.96 | 1.00 | 1.00 |
| en | hard | rules_fast | 0.96 | 0.91 | 1.00 | 0.98 |
| en | hard | rules_ner | 0.98 | 0.70 | 0.94 | 0.96 |
| en | hard | rules_ner_post | 1.00 | 1.00 | 1.00 | 1.00 |
| en | hard | rules_ner_post_tuned | 1.00 | 1.00 | 1.00 | 1.00 |
| en | standard | ner_only | 0.95 | 0.90 | 0.98 | 0.96 |
| en | standard | rules | 0.98 | 0.98 | 1.00 | 0.99 |
| en | standard | rules_both | 1.00 | 1.00 | 0.95 | 0.98 |
| en | standard | rules_fast | 0.98 | 0.98 | 0.95 | 0.96 |
| en | standard | rules_ner | 1.00 | 1.00 | 1.00 | 1.00 |
| en | standard | rules_ner_post | 1.00 | 1.00 | 1.00 | 1.00 |
| en | standard | rules_ner_post_tuned | 1.00 | 1.00 | 1.00 | 1.00 |
| fr | hard | ner_only | 0.93 | 0.65 | 0.96 | 0.95 |
| fr | hard | rules | 0.98 | 0.98 | 1.00 | 0.99 |
| fr | hard | rules_both | 1.00 | 1.00 | 1.00 | 1.00 |
| fr | hard | rules_fast | 1.00 | 1.00 | 1.00 | 1.00 |
| fr | hard | rules_ner | 0.98 | 0.70 | 0.98 | 0.98 |
| fr | hard | rules_ner_post | 0.98 | 0.98 | 1.00 | 0.99 |
| fr | hard | rules_ner_post_tuned | 0.98 | 0.98 | 1.00 | 0.99 |
| fr | standard | ner_only | 0.98 | 0.98 | 0.95 | 0.96 |
| fr | standard | rules | 0.98 | 0.98 | 1.00 | 0.99 |
| fr | standard | rules_both | 0.98 | 0.98 | 0.93 | 0.95 |
| fr | standard | rules_fast | 0.98 | 0.98 | 0.93 | 0.95 |
| fr | standard | rules_ner | 1.00 | 1.00 | 1.00 | 1.00 |
| fr | standard | rules_ner_post | 1.00 | 1.00 | 1.00 | 1.00 |
| fr | standard | rules_ner_post_tuned | 1.00 | 0.98 | 1.00 | 1.00 |
| de | hard | ner_only | 0.91 | 0.63 | 0.93 | 0.92 |
| de | hard | rules | 0.98 | 0.98 | 1.00 | 0.99 |
| de | hard | rules_both | 1.00 | 0.96 | 1.00 | 1.00 |
| de | hard | rules_fast | 0.96 | 0.91 | 1.00 | 0.98 |
| de | hard | rules_ner | 0.98 | 0.70 | 0.96 | 0.97 |
| de | hard | rules_ner_post | 1.00 | 1.00 | 1.00 | 1.00 |
| de | hard | rules_ner_post_tuned | 1.00 | 1.00 | 1.00 | 1.00 |
| de | standard | ner_only | 0.98 | 0.98 | 0.95 | 0.96 |
| de | standard | rules | 0.98 | 0.98 | 1.00 | 0.99 |
| de | standard | rules_both | 1.00 | 0.98 | 1.00 | 1.00 |
| de | standard | rules_fast | 1.00 | 1.00 | 1.00 | 1.00 |
| de | standard | rules_ner | 1.00 | 1.00 | 1.00 | 1.00 |
| de | standard | rules_ner_post | 1.00 | 0.98 | 1.00 | 1.00 |
| de | standard | rules_ner_post_tuned | 1.00 | 0.98 | 1.00 | 1.00 |
| lb | hard | ner_only | 0.67 | 0.26 | 0.91 | 0.78 |
| lb | hard | rules | 0.98 | 0.98 | 1.00 | 0.99 |
| lb | hard | rules_both | 1.00 | 0.98 | 1.00 | 1.00 |
| lb | hard | rules_fast | 0.98 | 0.98 | 1.00 | 0.99 |
| lb | hard | rules_ner | 0.96 | 0.57 | 0.96 | 0.96 |
| lb | hard | rules_ner_post | 1.00 | 1.00 | 1.00 | 1.00 |
| lb | hard | rules_ner_post_tuned | 1.00 | 0.98 | 1.00 | 1.00 |
| lb | standard | ner_only | 0.98 | 0.98 | 0.93 | 0.95 |
| lb | standard | rules | 0.98 | 0.98 | 1.00 | 0.99 |
| lb | standard | rules_both | 1.00 | 0.98 | 0.95 | 0.98 |
| lb | standard | rules_fast | 0.98 | 0.98 | 0.95 | 0.96 |
| lb | standard | rules_ner | 1.00 | 1.00 | 1.00 | 1.00 |
| lb | standard | rules_ner_post | 1.00 | 0.98 | 1.00 | 1.00 |
| lb | standard | rules_ner_post_tuned | 1.00 | 0.98 | 1.00 | 1.00 |
