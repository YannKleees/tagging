# Variant comparison (eval split)

Variants: `rules` ({}), `ner_only` ({"detectors": [], "ner": {"threshold": 0.5, "postprocess": false, "confirm_ambiguous": false, "distractors": false}}), `rules_ner` ({"ner": {"threshold": 0.5, "postprocess": false, "confirm_ambiguous": false, "distractors": false}}), `rules_ner_post` ({"ner": {"threshold": 0.5}}), `rules_ner_post_tuned` ({"ner": true}), `rules_fast` ({"ner": {"backend": "spacy"}}), `rules_both` ({"ner": {"backend": "both"}}).

## Luxembourgish, names only (person, organization, location, address)

| lang | subset | variant | recall_overlap | recall_exact | precision_overlap | f1_overlap |
|---|---|---|---|---|---|---|
| lb | hard | ner_only | 0.74 | 0.30 | 0.79 | 0.76 |
| lb | hard | rules | 1.00 | 1.00 | 1.00 | 1.00 |
| lb | hard | rules_both | 1.00 | 0.98 | 1.00 | 1.00 |
| lb | hard | rules_fast | 0.96 | 0.93 | 1.00 | 0.98 |
| lb | hard | rules_ner | 0.93 | 0.50 | 0.90 | 0.91 |
| lb | hard | rules_ner_post | 1.00 | 1.00 | 1.00 | 1.00 |
| lb | hard | rules_ner_post_tuned | 1.00 | 1.00 | 1.00 | 1.00 |
| lb | standard | ner_only | 0.98 | 0.95 | 0.91 | 0.94 |
| lb | standard | rules | 0.98 | 0.98 | 1.00 | 0.99 |
| lb | standard | rules_both | 1.00 | 0.95 | 0.95 | 0.98 |
| lb | standard | rules_fast | 1.00 | 1.00 | 0.95 | 0.98 |
| lb | standard | rules_ner | 1.00 | 0.98 | 1.00 | 1.00 |
| lb | standard | rules_ner_post | 1.00 | 0.98 | 1.00 | 1.00 |
| lb | standard | rules_ner_post_tuned | 1.00 | 0.95 | 1.00 | 1.00 |

## False positives on entity-free sentences

| variant | en | fr | de | lb | sentences_per_lang |
|---|---|---|---|---|---|
| rules | 1 | 3 | 2 | 4 | 20 |
| ner_only | 27 | 23 | 19 | 23 | 20 |
| rules_ner | 27 | 25 | 21 | 27 | 20 |
| rules_ner_post | 3 | 4 | 1 | 5 | 20 |
| rules_ner_post_tuned | 3 | 4 | 1 | 5 | 20 |
| rules_fast | 3 | 3 | 2 | 5 | 20 |
| rules_both | 3 | 4 | 3 | 6 | 20 |

## Runtime (CPU, detection only)

| variant | ms per 1,000 words |
|---|---|
| rules | 8.30 |
| ner_only | 6317.30 |
| rules_ner | 5913.10 |
| rules_ner_post | 7467.10 |
| rules_ner_post_tuned | 7477.50 |
| rules_fast | 899.60 |
| rules_both | 7825.30 |

## All entity types, by language and subset

| lang | subset | variant | recall_overlap | recall_exact | precision_overlap | f1_overlap |
|---|---|---|---|---|---|---|
| en | hard | ner_only | 0.67 | 0.47 | 0.91 | 0.77 |
| en | hard | rules | 1.00 | 1.00 | 1.00 | 1.00 |
| en | hard | rules_both | 1.00 | 0.98 | 1.00 | 1.00 |
| en | hard | rules_fast | 0.98 | 0.97 | 1.00 | 0.99 |
| en | hard | rules_ner | 1.00 | 0.80 | 0.97 | 0.98 |
| en | hard | rules_ner_post | 1.00 | 1.00 | 1.00 | 1.00 |
| en | hard | rules_ner_post_tuned | 1.00 | 1.00 | 1.00 | 1.00 |
| en | standard | ner_only | 0.61 | 0.56 | 0.93 | 0.73 |
| en | standard | rules | 0.98 | 0.98 | 1.00 | 0.99 |
| en | standard | rules_both | 1.00 | 1.00 | 0.97 | 0.99 |
| en | standard | rules_fast | 0.98 | 0.98 | 0.96 | 0.97 |
| en | standard | rules_ner | 1.00 | 0.98 | 1.00 | 1.00 |
| en | standard | rules_ner_post | 1.00 | 1.00 | 1.00 | 1.00 |
| en | standard | rules_ner_post_tuned | 1.00 | 1.00 | 1.00 | 1.00 |
| fr | hard | ner_only | 0.70 | 0.48 | 1.00 | 0.82 |
| fr | hard | rules | 1.00 | 1.00 | 1.00 | 1.00 |
| fr | hard | rules_both | 0.98 | 0.97 | 1.00 | 0.99 |
| fr | hard | rules_fast | 0.97 | 0.97 | 1.00 | 0.98 |
| fr | hard | rules_ner | 0.98 | 0.77 | 1.00 | 0.99 |
| fr | hard | rules_ner_post | 0.98 | 0.97 | 1.00 | 0.99 |
| fr | hard | rules_ner_post_tuned | 0.98 | 0.97 | 1.00 | 0.99 |
| fr | standard | ner_only | 0.62 | 0.59 | 0.91 | 0.74 |
| fr | standard | rules | 0.98 | 0.98 | 1.00 | 0.99 |
| fr | standard | rules_both | 1.00 | 1.00 | 0.97 | 0.99 |
| fr | standard | rules_fast | 1.00 | 1.00 | 0.97 | 0.99 |
| fr | standard | rules_ner | 1.00 | 0.98 | 1.00 | 1.00 |
| fr | standard | rules_ner_post | 1.00 | 1.00 | 1.00 | 1.00 |
| fr | standard | rules_ner_post_tuned | 1.00 | 1.00 | 1.00 | 1.00 |
| de | hard | ner_only | 0.70 | 0.52 | 0.88 | 0.78 |
| de | hard | rules | 1.00 | 1.00 | 1.00 | 1.00 |
| de | hard | rules_both | 0.97 | 0.93 | 0.98 | 0.97 |
| de | hard | rules_fast | 0.97 | 0.93 | 0.98 | 0.97 |
| de | hard | rules_ner | 1.00 | 0.82 | 0.97 | 0.98 |
| de | hard | rules_ner_post | 0.98 | 0.98 | 1.00 | 0.99 |
| de | hard | rules_ner_post_tuned | 0.98 | 0.98 | 1.00 | 0.99 |
| de | standard | ner_only | 0.62 | 0.59 | 0.91 | 0.74 |
| de | standard | rules | 0.98 | 0.98 | 1.00 | 0.99 |
| de | standard | rules_both | 1.00 | 0.97 | 1.00 | 1.00 |
| de | standard | rules_fast | 1.00 | 1.00 | 1.00 | 1.00 |
| de | standard | rules_ner | 1.00 | 0.97 | 1.00 | 1.00 |
| de | standard | rules_ner_post | 1.00 | 0.97 | 1.00 | 1.00 |
| de | standard | rules_ner_post_tuned | 1.00 | 0.97 | 1.00 | 1.00 |
| lb | hard | ner_only | 0.57 | 0.23 | 0.79 | 0.66 |
| lb | hard | rules | 1.00 | 1.00 | 1.00 | 1.00 |
| lb | hard | rules_both | 1.00 | 0.98 | 1.00 | 1.00 |
| lb | hard | rules_fast | 0.97 | 0.95 | 1.00 | 0.98 |
| lb | hard | rules_ner | 0.95 | 0.62 | 0.92 | 0.93 |
| lb | hard | rules_ner_post | 1.00 | 1.00 | 1.00 | 1.00 |
| lb | hard | rules_ner_post_tuned | 1.00 | 1.00 | 1.00 | 1.00 |
| lb | standard | ner_only | 0.62 | 0.61 | 0.91 | 0.74 |
| lb | standard | rules | 0.98 | 0.98 | 1.00 | 0.99 |
| lb | standard | rules_both | 1.00 | 0.97 | 0.97 | 0.99 |
| lb | standard | rules_fast | 1.00 | 1.00 | 0.97 | 0.99 |
| lb | standard | rules_ner | 1.00 | 0.98 | 1.00 | 1.00 |
| lb | standard | rules_ner_post | 1.00 | 0.98 | 1.00 | 1.00 |
| lb | standard | rules_ner_post_tuned | 1.00 | 0.97 | 1.00 | 1.00 |

## Names only, by language and subset

| lang | subset | variant | recall_overlap | recall_exact | precision_overlap | f1_overlap |
|---|---|---|---|---|---|---|
| en | hard | ner_only | 0.87 | 0.61 | 0.91 | 0.89 |
| en | hard | rules | 1.00 | 1.00 | 1.00 | 1.00 |
| en | hard | rules_both | 1.00 | 0.98 | 1.00 | 1.00 |
| en | hard | rules_fast | 0.98 | 0.96 | 1.00 | 0.99 |
| en | hard | rules_ner | 1.00 | 0.74 | 0.96 | 0.98 |
| en | hard | rules_ner_post | 1.00 | 1.00 | 1.00 | 1.00 |
| en | hard | rules_ner_post_tuned | 1.00 | 1.00 | 1.00 | 1.00 |
| en | standard | ner_only | 0.95 | 0.88 | 0.93 | 0.94 |
| en | standard | rules | 0.98 | 0.98 | 1.00 | 0.99 |
| en | standard | rules_both | 1.00 | 1.00 | 0.95 | 0.98 |
| en | standard | rules_fast | 0.98 | 0.98 | 0.93 | 0.95 |
| en | standard | rules_ner | 1.00 | 0.98 | 1.00 | 1.00 |
| en | standard | rules_ner_post | 1.00 | 1.00 | 1.00 | 1.00 |
| en | standard | rules_ner_post_tuned | 1.00 | 1.00 | 1.00 | 1.00 |
| fr | hard | ner_only | 0.91 | 0.63 | 1.00 | 0.95 |
| fr | hard | rules | 1.00 | 1.00 | 1.00 | 1.00 |
| fr | hard | rules_both | 0.98 | 0.96 | 1.00 | 0.99 |
| fr | hard | rules_fast | 0.96 | 0.96 | 1.00 | 0.98 |
| fr | hard | rules_ner | 0.98 | 0.70 | 1.00 | 0.99 |
| fr | hard | rules_ner_post | 0.98 | 0.96 | 1.00 | 0.99 |
| fr | hard | rules_ner_post_tuned | 0.98 | 0.96 | 1.00 | 0.99 |
| fr | standard | ner_only | 0.98 | 0.93 | 0.91 | 0.94 |
| fr | standard | rules | 0.98 | 0.98 | 1.00 | 0.99 |
| fr | standard | rules_both | 1.00 | 1.00 | 0.95 | 0.98 |
| fr | standard | rules_fast | 1.00 | 1.00 | 0.95 | 0.98 |
| fr | standard | rules_ner | 1.00 | 0.98 | 1.00 | 1.00 |
| fr | standard | rules_ner_post | 1.00 | 1.00 | 1.00 | 1.00 |
| fr | standard | rules_ner_post_tuned | 1.00 | 1.00 | 1.00 | 1.00 |
| de | hard | ner_only | 0.91 | 0.67 | 0.88 | 0.89 |
| de | hard | rules | 1.00 | 1.00 | 1.00 | 1.00 |
| de | hard | rules_both | 0.96 | 0.91 | 0.98 | 0.97 |
| de | hard | rules_fast | 0.96 | 0.91 | 0.98 | 0.97 |
| de | hard | rules_ner | 1.00 | 0.76 | 0.96 | 0.98 |
| de | hard | rules_ner_post | 0.98 | 0.98 | 1.00 | 0.99 |
| de | hard | rules_ner_post_tuned | 0.98 | 0.98 | 1.00 | 0.99 |
| de | standard | ner_only | 0.98 | 0.93 | 0.91 | 0.94 |
| de | standard | rules | 0.98 | 0.98 | 1.00 | 0.99 |
| de | standard | rules_both | 1.00 | 0.95 | 1.00 | 1.00 |
| de | standard | rules_fast | 1.00 | 1.00 | 1.00 | 1.00 |
| de | standard | rules_ner | 1.00 | 0.95 | 1.00 | 1.00 |
| de | standard | rules_ner_post | 1.00 | 0.95 | 1.00 | 1.00 |
| de | standard | rules_ner_post_tuned | 1.00 | 0.95 | 1.00 | 1.00 |
| lb | hard | ner_only | 0.74 | 0.30 | 0.79 | 0.76 |
| lb | hard | rules | 1.00 | 1.00 | 1.00 | 1.00 |
| lb | hard | rules_both | 1.00 | 0.98 | 1.00 | 1.00 |
| lb | hard | rules_fast | 0.96 | 0.93 | 1.00 | 0.98 |
| lb | hard | rules_ner | 0.93 | 0.50 | 0.90 | 0.91 |
| lb | hard | rules_ner_post | 1.00 | 1.00 | 1.00 | 1.00 |
| lb | hard | rules_ner_post_tuned | 1.00 | 1.00 | 1.00 | 1.00 |
| lb | standard | ner_only | 0.98 | 0.95 | 0.91 | 0.94 |
| lb | standard | rules | 0.98 | 0.98 | 1.00 | 0.99 |
| lb | standard | rules_both | 1.00 | 0.95 | 0.95 | 0.98 |
| lb | standard | rules_fast | 1.00 | 1.00 | 0.95 | 0.98 |
| lb | standard | rules_ner | 1.00 | 0.98 | 1.00 | 1.00 |
| lb | standard | rules_ner_post | 1.00 | 0.98 | 1.00 | 1.00 |
| lb | standard | rules_ner_post_tuned | 1.00 | 0.95 | 1.00 | 1.00 |
