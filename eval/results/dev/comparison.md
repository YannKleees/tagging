# Variant comparison (dev split)

Variants: `rules_ner_post` ({"ner": {"postprocess": true, "confirm_ambiguous": true}}), `post_t040`.

## Luxembourgish, names only (person, organization, location, address)

| lang | subset | variant | recall_overlap | recall_exact | precision_overlap | f1_overlap |
|---|---|---|---|---|---|---|
| lb | hard | post_t040 | 0.89 | 0.83 | 0.95 | 0.92 |
| lb | hard | rules_ner_post | 0.87 | 0.83 | 0.98 | 0.92 |
| lb | standard | post_t040 | 1.00 | 0.98 | 1.00 | 1.00 |
| lb | standard | rules_ner_post | 0.98 | 0.98 | 1.00 | 0.99 |

## False positives on entity-free sentences

| variant | en | fr | de | lb | sentences_per_lang |
|---|---|---|---|---|---|
| rules_ner_post | 0 | 0 | 0 | 0 | 10 |
| post_t040 | 0 | 1 | 0 | 0 | 10 |

## Runtime (CPU, detection only)

| variant | ms per 1,000 words |
|---|---|
| rules_ner_post | 312935.50 |
| post_t040 | 107843.10 |

## All entity types, by language and subset

| lang | subset | variant | recall_overlap | recall_exact | precision_overlap | f1_overlap |
|---|---|---|---|---|---|---|
| en | hard | post_t040 | 0.92 | 0.88 | 1.00 | 0.96 |
| en | hard | rules_ner_post | 0.92 | 0.88 | 1.00 | 0.96 |
| en | standard | post_t040 | 0.97 | 0.94 | 1.00 | 0.98 |
| en | standard | rules_ner_post | 0.97 | 0.94 | 1.00 | 0.98 |
| fr | hard | post_t040 | 0.98 | 0.95 | 1.00 | 0.99 |
| fr | hard | rules_ner_post | 0.98 | 0.95 | 1.00 | 0.99 |
| fr | standard | post_t040 | 0.98 | 0.98 | 1.00 | 0.99 |
| fr | standard | rules_ner_post | 0.98 | 0.98 | 1.00 | 0.99 |
| de | hard | post_t040 | 0.98 | 0.95 | 1.00 | 0.99 |
| de | hard | rules_ner_post | 0.97 | 0.93 | 1.00 | 0.98 |
| de | standard | post_t040 | 0.98 | 0.98 | 1.00 | 0.99 |
| de | standard | rules_ner_post | 0.98 | 0.98 | 1.00 | 0.99 |
| lb | hard | post_t040 | 0.92 | 0.87 | 0.96 | 0.94 |
| lb | hard | rules_ner_post | 0.90 | 0.87 | 0.98 | 0.94 |
| lb | standard | post_t040 | 1.00 | 0.98 | 1.00 | 1.00 |
| lb | standard | rules_ner_post | 0.98 | 0.98 | 1.00 | 0.99 |

## Names only, by language and subset

| lang | subset | variant | recall_overlap | recall_exact | precision_overlap | f1_overlap |
|---|---|---|---|---|---|---|
| en | hard | post_t040 | 0.93 | 0.89 | 1.00 | 0.97 |
| en | hard | rules_ner_post | 0.93 | 0.89 | 1.00 | 0.97 |
| en | standard | post_t040 | 0.95 | 0.90 | 1.00 | 0.98 |
| en | standard | rules_ner_post | 0.95 | 0.90 | 1.00 | 0.98 |
| fr | hard | post_t040 | 0.98 | 0.93 | 1.00 | 0.99 |
| fr | hard | rules_ner_post | 0.98 | 0.93 | 1.00 | 0.99 |
| fr | standard | post_t040 | 0.98 | 0.98 | 1.00 | 0.99 |
| fr | standard | rules_ner_post | 0.98 | 0.98 | 1.00 | 0.99 |
| de | hard | post_t040 | 0.98 | 0.93 | 1.00 | 0.99 |
| de | hard | rules_ner_post | 0.96 | 0.91 | 1.00 | 0.98 |
| de | standard | post_t040 | 0.98 | 0.98 | 1.00 | 0.99 |
| de | standard | rules_ner_post | 0.98 | 0.98 | 1.00 | 0.99 |
| lb | hard | post_t040 | 0.89 | 0.83 | 0.95 | 0.92 |
| lb | hard | rules_ner_post | 0.87 | 0.83 | 0.98 | 0.92 |
| lb | standard | post_t040 | 1.00 | 0.98 | 1.00 | 1.00 |
| lb | standard | rules_ner_post | 0.98 | 0.98 | 1.00 | 0.99 |
