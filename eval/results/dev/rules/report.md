# Evaluation: dev/rules

Config: `eval/configs/rules.json`. 248 sentences, 2297 words, 504 gold spans.

Recall comes first: a missed identifier leaks personal data, an over-redaction only costs readability.
`overlap` counts a prediction of the right type that overlaps the gold span, `exact` needs identical offsets, and `any_type_recall` counts a gold span as redacted if any prediction overlaps it.

Runtime: 10.02 ms per 1,000 words (detection only).

## By language and subset

| lang | subset | recall_overlap | recall_exact | any_type_recall | precision_overlap | f1_overlap | f1_exact | n_gold | n_pred |
|---|---|---|---|---|---|---|---|---|---|
| de | hard | 0.98 | 0.98 | 0.98 | 1.00 | 0.99 | 0.99 | 60 | 59 |
| de | standard | 0.98 | 0.98 | 0.98 | 1.00 | 0.99 | 0.99 | 66 | 65 |
| en | hard | 0.98 | 0.98 | 0.98 | 1.00 | 0.99 | 0.99 | 60 | 59 |
| en | standard | 0.98 | 0.98 | 0.98 | 1.00 | 0.99 | 0.99 | 66 | 65 |
| fr | hard | 0.98 | 0.98 | 0.98 | 1.00 | 0.99 | 0.99 | 60 | 59 |
| fr | standard | 0.98 | 0.98 | 0.98 | 1.00 | 0.99 | 0.99 | 66 | 65 |
| lb | hard | 0.98 | 0.98 | 0.98 | 1.00 | 0.99 | 0.99 | 60 | 59 |
| lb | standard | 0.98 | 0.98 | 0.98 | 1.00 | 0.99 | 0.99 | 66 | 65 |

## Names only (person, organization, location, address)

| lang | subset | recall_overlap | recall_exact | precision_overlap | f1_overlap | n_gold | n_pred |
|---|---|---|---|---|---|---|---|
| de | hard | 0.98 | 0.98 | 1.00 | 0.99 | 46 | 45 |
| de | standard | 0.98 | 0.98 | 1.00 | 0.99 | 42 | 41 |
| en | hard | 0.98 | 0.98 | 1.00 | 0.99 | 46 | 45 |
| en | standard | 0.98 | 0.98 | 1.00 | 0.99 | 42 | 41 |
| fr | hard | 0.98 | 0.98 | 1.00 | 0.99 | 46 | 45 |
| fr | standard | 0.98 | 0.98 | 1.00 | 0.99 | 42 | 41 |
| lb | hard | 0.98 | 0.98 | 1.00 | 0.99 | 46 | 45 |
| lb | standard | 0.98 | 0.98 | 1.00 | 0.99 | 42 | 41 |

## False positives on the entity-free subset

| lang | false positives | sentences |
|---|---|---|
| en | 0 | 10 |
| fr | 0 | 10 |
| de | 0 | 10 |
| lb | 0 | 10 |

None.

## By entity type

| type | recall_overlap | recall_exact | any_type_recall | precision_overlap | f1_overlap | f1_exact | n_gold | n_pred |
|---|---|---|---|---|---|---|---|---|
| address | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 8 | 8 |
| credit_card | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 16 | 16 |
| date_of_birth | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 24 | 24 |
| email | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 32 | 32 |
| iban | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 16 | 16 |
| ip_address | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 8 | 8 |
| location | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 80 | 80 |
| national_id | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 16 | 16 |
| organization | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 48 | 48 |
| person | 0.96 | 0.96 | 0.96 | 1.00 | 0.98 | 0.98 | 216 | 208 |
| phone | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 32 | 32 |
| url | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 8 | 8 |

## By language and entity type (overlap)

| lang | type | recall_overlap | precision_overlap | f1_overlap | n_gold | n_pred |
|---|---|---|---|---|---|---|
| de | address | 1.00 | 1.00 | 1.00 | 2 | 2 |
| de | credit_card | 1.00 | 1.00 | 1.00 | 4 | 4 |
| de | date_of_birth | 1.00 | 1.00 | 1.00 | 6 | 6 |
| de | email | 1.00 | 1.00 | 1.00 | 8 | 8 |
| de | iban | 1.00 | 1.00 | 1.00 | 4 | 4 |
| de | ip_address | 1.00 | 1.00 | 1.00 | 2 | 2 |
| de | location | 1.00 | 1.00 | 1.00 | 20 | 20 |
| de | national_id | 1.00 | 1.00 | 1.00 | 4 | 4 |
| de | organization | 1.00 | 1.00 | 1.00 | 12 | 12 |
| de | person | 0.96 | 1.00 | 0.98 | 54 | 52 |
| de | phone | 1.00 | 1.00 | 1.00 | 8 | 8 |
| de | url | 1.00 | 1.00 | 1.00 | 2 | 2 |
| en | address | 1.00 | 1.00 | 1.00 | 2 | 2 |
| en | credit_card | 1.00 | 1.00 | 1.00 | 4 | 4 |
| en | date_of_birth | 1.00 | 1.00 | 1.00 | 6 | 6 |
| en | email | 1.00 | 1.00 | 1.00 | 8 | 8 |
| en | iban | 1.00 | 1.00 | 1.00 | 4 | 4 |
| en | ip_address | 1.00 | 1.00 | 1.00 | 2 | 2 |
| en | location | 1.00 | 1.00 | 1.00 | 20 | 20 |
| en | national_id | 1.00 | 1.00 | 1.00 | 4 | 4 |
| en | organization | 1.00 | 1.00 | 1.00 | 12 | 12 |
| en | person | 0.96 | 1.00 | 0.98 | 54 | 52 |
| en | phone | 1.00 | 1.00 | 1.00 | 8 | 8 |
| en | url | 1.00 | 1.00 | 1.00 | 2 | 2 |
| fr | address | 1.00 | 1.00 | 1.00 | 2 | 2 |
| fr | credit_card | 1.00 | 1.00 | 1.00 | 4 | 4 |
| fr | date_of_birth | 1.00 | 1.00 | 1.00 | 6 | 6 |
| fr | email | 1.00 | 1.00 | 1.00 | 8 | 8 |
| fr | iban | 1.00 | 1.00 | 1.00 | 4 | 4 |
| fr | ip_address | 1.00 | 1.00 | 1.00 | 2 | 2 |
| fr | location | 1.00 | 1.00 | 1.00 | 20 | 20 |
| fr | national_id | 1.00 | 1.00 | 1.00 | 4 | 4 |
| fr | organization | 1.00 | 1.00 | 1.00 | 12 | 12 |
| fr | person | 0.96 | 1.00 | 0.98 | 54 | 52 |
| fr | phone | 1.00 | 1.00 | 1.00 | 8 | 8 |
| fr | url | 1.00 | 1.00 | 1.00 | 2 | 2 |
| lb | address | 1.00 | 1.00 | 1.00 | 2 | 2 |
| lb | credit_card | 1.00 | 1.00 | 1.00 | 4 | 4 |
| lb | date_of_birth | 1.00 | 1.00 | 1.00 | 6 | 6 |
| lb | email | 1.00 | 1.00 | 1.00 | 8 | 8 |
| lb | iban | 1.00 | 1.00 | 1.00 | 4 | 4 |
| lb | ip_address | 1.00 | 1.00 | 1.00 | 2 | 2 |
| lb | location | 1.00 | 1.00 | 1.00 | 20 | 20 |
| lb | national_id | 1.00 | 1.00 | 1.00 | 4 | 4 |
| lb | organization | 1.00 | 1.00 | 1.00 | 12 | 12 |
| lb | person | 0.96 | 1.00 | 0.98 | 54 | 52 |
| lb | phone | 1.00 | 1.00 | 1.00 | 8 | 8 |
| lb | url | 1.00 | 1.00 | 1.00 | 2 | 2 |

## Gold names found in the gazetteer

Share of distinct gold person, location and organization strings that the built-in gazetteer contains. High values mean rules-only recall on names partly reflects list overlap with the generator.

| type | distinct | in_gazetteer | share |
|---|---|---|---|
| location | 42 | 42 | 1.00 |
| organization | 11 | 11 | 1.00 |
| person | 50 | 40 | 0.80 |

## Missed spans (8)

| id | type | text | covered | sentence |
|---|---|---|---|---|
| en-standard-job-1 | person | Ellinor Haering | False | Ellinor Haering has worked for Deutsche Bank in London since 2019. |
| fr-standard-job-1 | person | Ellinor Haering | False | Ellinor Haering travaille chez Deutsche Bank à Londres depuis 2019. |
| de-standard-job-1 | person | Ellinor Haering | False | Ellinor Haering arbeitet seit 2019 bei Deutsche Bank in London. |
| lb-standard-job-1 | person | Ellinor Haering | False | Ellinor Haering schafft zënter 2019 bei Deutsche Bank zu London. |
| en-hard-family-1 | person | Romaine | False | My mother and Romaine are coming to Goesdorf on Sunday. |
| fr-hard-family-1 | person | Romaine | False | Ma mère et Romaine viennent à Goesdorf dimanche. |
| de-hard-family-1 | person | Romaine | False | Meine Mutter und Romaine kommen am Sonntag nach Goesdorf. |
| lb-hard-family-1 | person | Romaine | False | Meng Mamm an d'Romaine kommen e Sonndeg op Géisdref. |
