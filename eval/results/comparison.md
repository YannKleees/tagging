# Variant comparison (eval split)

Variants: `rules` ({}), `ner_only` ({"detectors": [], "ner": {"threshold": 0.5, "postprocess": false, "confirm_ambiguous": false}}), `rules_ner` ({"ner": {"threshold": 0.5, "postprocess": false, "confirm_ambiguous": false}}), `rules_ner_post` ({"ner": {"threshold": 0.5}}), `rules_ner_post_tuned` ({"ner": true}).

## Luxembourgish, names only (person, organization, location, address)

| lang | subset | variant | recall_overlap | recall_exact | precision_overlap | f1_overlap |
|---|---|---|---|---|---|---|
| lb | hard | ner_only | 0.74 | 0.30 | 0.79 | 0.76 |
| lb | hard | rules | 0.57 | 0.57 | 1.00 | 0.72 |
| lb | hard | rules_ner | 0.87 | 0.43 | 0.89 | 0.88 |
| lb | hard | rules_ner_post | 0.91 | 0.89 | 1.00 | 0.95 |
| lb | hard | rules_ner_post_tuned | 0.91 | 0.89 | 1.00 | 0.95 |
| lb | standard | ner_only | 0.98 | 0.95 | 0.91 | 0.94 |
| lb | standard | rules | 0.36 | 0.21 | 1.00 | 0.53 |
| lb | standard | rules_ner | 0.98 | 0.95 | 0.98 | 0.98 |
| lb | standard | rules_ner_post | 0.98 | 0.95 | 0.98 | 0.98 |
| lb | standard | rules_ner_post_tuned | 0.98 | 0.95 | 0.98 | 0.98 |

## False positives on entity-free sentences

| variant | en | fr | de | lb | sentences_per_lang |
|---|---|---|---|---|---|
| rules | 1 | 2 | 2 | 5 | 20 |
| ner_only | 27 | 23 | 19 | 23 | 20 |
| rules_ner | 27 | 25 | 21 | 27 | 20 |
| rules_ner_post | 9 | 7 | 4 | 10 | 20 |
| rules_ner_post_tuned | 9 | 8 | 6 | 13 | 20 |

## Runtime (CPU, detection only)

| variant | ms per 1,000 words |
|---|---|
| rules | 63.90 |
| ner_only | 46809.00 |
| rules_ner | 43714.00 |
| rules_ner_post | 46825.50 |
| rules_ner_post_tuned | 39775.60 |

## All entity types, by language and subset

| lang | subset | variant | recall_overlap | recall_exact | precision_overlap | f1_overlap |
|---|---|---|---|---|---|---|
| en | hard | ner_only | 0.67 | 0.47 | 0.91 | 0.77 |
| en | hard | rules | 0.63 | 0.63 | 1.00 | 0.78 |
| en | hard | rules_ner | 0.92 | 0.72 | 0.96 | 0.94 |
| en | hard | rules_ner_post | 0.92 | 0.88 | 1.00 | 0.96 |
| en | hard | rules_ner_post_tuned | 0.93 | 0.90 | 1.00 | 0.97 |
| en | standard | ner_only | 0.61 | 0.56 | 0.93 | 0.73 |
| en | standard | rules | 0.59 | 0.50 | 1.00 | 0.74 |
| en | standard | rules_ner | 0.97 | 0.92 | 1.00 | 0.98 |
| en | standard | rules_ner_post | 0.97 | 0.92 | 1.00 | 0.98 |
| en | standard | rules_ner_post_tuned | 0.97 | 0.92 | 1.00 | 0.98 |
| fr | hard | ner_only | 0.70 | 0.48 | 1.00 | 0.82 |
| fr | hard | rules | 0.67 | 0.67 | 1.00 | 0.80 |
| fr | hard | rules_ner | 0.97 | 0.75 | 1.00 | 0.98 |
| fr | hard | rules_ner_post | 0.97 | 0.93 | 1.00 | 0.98 |
| fr | hard | rules_ner_post_tuned | 0.97 | 0.93 | 1.00 | 0.98 |
| fr | standard | ner_only | 0.62 | 0.59 | 0.91 | 0.74 |
| fr | standard | rules | 0.59 | 0.50 | 1.00 | 0.74 |
| fr | standard | rules_ner | 0.98 | 0.97 | 1.00 | 0.99 |
| fr | standard | rules_ner_post | 0.98 | 0.97 | 1.00 | 0.99 |
| fr | standard | rules_ner_post_tuned | 1.00 | 0.98 | 1.00 | 1.00 |
| de | hard | ner_only | 0.70 | 0.52 | 0.88 | 0.78 |
| de | hard | rules | 0.67 | 0.67 | 1.00 | 0.80 |
| de | hard | rules_ner | 0.98 | 0.80 | 0.97 | 0.98 |
| de | hard | rules_ner_post | 0.98 | 0.97 | 1.00 | 0.99 |
| de | hard | rules_ner_post_tuned | 0.98 | 0.97 | 1.00 | 0.99 |
| de | standard | ner_only | 0.62 | 0.59 | 0.91 | 0.74 |
| de | standard | rules | 0.59 | 0.50 | 1.00 | 0.74 |
| de | standard | rules_ner | 0.98 | 0.95 | 0.98 | 0.98 |
| de | standard | rules_ner_post | 0.98 | 0.95 | 0.98 | 0.98 |
| de | standard | rules_ner_post_tuned | 0.98 | 0.95 | 0.98 | 0.98 |
| lb | hard | ner_only | 0.57 | 0.23 | 0.79 | 0.66 |
| lb | hard | rules | 0.67 | 0.67 | 1.00 | 0.80 |
| lb | hard | rules_ner | 0.90 | 0.57 | 0.92 | 0.91 |
| lb | hard | rules_ner_post | 0.93 | 0.92 | 1.00 | 0.97 |
| lb | hard | rules_ner_post_tuned | 0.93 | 0.92 | 1.00 | 0.97 |
| lb | standard | ner_only | 0.62 | 0.61 | 0.91 | 0.74 |
| lb | standard | rules | 0.59 | 0.50 | 1.00 | 0.74 |
| lb | standard | rules_ner | 0.98 | 0.97 | 0.98 | 0.98 |
| lb | standard | rules_ner_post | 0.98 | 0.97 | 0.98 | 0.98 |
| lb | standard | rules_ner_post_tuned | 0.98 | 0.97 | 0.98 | 0.98 |

## Names only, by language and subset

| lang | subset | variant | recall_overlap | recall_exact | precision_overlap | f1_overlap |
|---|---|---|---|---|---|---|
| en | hard | ner_only | 0.87 | 0.61 | 0.91 | 0.89 |
| en | hard | rules | 0.57 | 0.57 | 1.00 | 0.72 |
| en | hard | rules_ner | 0.93 | 0.67 | 0.96 | 0.95 |
| en | hard | rules_ner_post | 0.93 | 0.89 | 1.00 | 0.97 |
| en | hard | rules_ner_post_tuned | 0.96 | 0.91 | 1.00 | 0.98 |
| en | standard | ner_only | 0.95 | 0.88 | 0.93 | 0.94 |
| en | standard | rules | 0.36 | 0.21 | 1.00 | 0.53 |
| en | standard | rules_ner | 0.95 | 0.88 | 1.00 | 0.98 |
| en | standard | rules_ner_post | 0.95 | 0.88 | 1.00 | 0.98 |
| en | standard | rules_ner_post_tuned | 0.95 | 0.88 | 1.00 | 0.98 |
| fr | hard | ner_only | 0.91 | 0.63 | 1.00 | 0.95 |
| fr | hard | rules | 0.57 | 0.57 | 1.00 | 0.72 |
| fr | hard | rules_ner | 0.96 | 0.67 | 1.00 | 0.98 |
| fr | hard | rules_ner_post | 0.96 | 0.91 | 1.00 | 0.98 |
| fr | hard | rules_ner_post_tuned | 0.96 | 0.91 | 1.00 | 0.98 |
| fr | standard | ner_only | 0.98 | 0.93 | 0.91 | 0.94 |
| fr | standard | rules | 0.36 | 0.21 | 1.00 | 0.53 |
| fr | standard | rules_ner | 0.98 | 0.95 | 1.00 | 0.99 |
| fr | standard | rules_ner_post | 0.98 | 0.95 | 1.00 | 0.99 |
| fr | standard | rules_ner_post_tuned | 1.00 | 0.98 | 1.00 | 1.00 |
| de | hard | ner_only | 0.91 | 0.67 | 0.88 | 0.89 |
| de | hard | rules | 0.57 | 0.57 | 1.00 | 0.72 |
| de | hard | rules_ner | 0.98 | 0.74 | 0.96 | 0.97 |
| de | hard | rules_ner_post | 0.98 | 0.96 | 1.00 | 0.99 |
| de | hard | rules_ner_post_tuned | 0.98 | 0.96 | 1.00 | 0.99 |
| de | standard | ner_only | 0.98 | 0.93 | 0.91 | 0.94 |
| de | standard | rules | 0.36 | 0.21 | 1.00 | 0.53 |
| de | standard | rules_ner | 0.98 | 0.93 | 0.98 | 0.98 |
| de | standard | rules_ner_post | 0.98 | 0.93 | 0.98 | 0.98 |
| de | standard | rules_ner_post_tuned | 0.98 | 0.93 | 0.98 | 0.98 |
| lb | hard | ner_only | 0.74 | 0.30 | 0.79 | 0.76 |
| lb | hard | rules | 0.57 | 0.57 | 1.00 | 0.72 |
| lb | hard | rules_ner | 0.87 | 0.43 | 0.89 | 0.88 |
| lb | hard | rules_ner_post | 0.91 | 0.89 | 1.00 | 0.95 |
| lb | hard | rules_ner_post_tuned | 0.91 | 0.89 | 1.00 | 0.95 |
| lb | standard | ner_only | 0.98 | 0.95 | 0.91 | 0.94 |
| lb | standard | rules | 0.36 | 0.21 | 1.00 | 0.53 |
| lb | standard | rules_ner | 0.98 | 0.95 | 0.98 | 0.98 |
| lb | standard | rules_ner_post | 0.98 | 0.95 | 0.98 | 0.98 |
| lb | standard | rules_ner_post_tuned | 0.98 | 0.95 | 0.98 | 0.98 |
