# Evaluation: rules

Config: `eval/configs/rules.json`. 288 sentences, 2637 words, 504 gold spans.

Recall comes first: a missed identifier leaks personal data, an over-redaction only costs readability.
`overlap` counts a prediction of the right type that overlaps the gold span, `exact` needs identical offsets, and `any_type_recall` counts a gold span as redacted if any prediction overlaps it.

Runtime: 8.31 ms per 1,000 words (detection only).

## By language and subset

| lang | subset | recall_overlap | recall_exact | any_type_recall | precision_overlap | f1_overlap | f1_exact | n_gold | n_pred |
|---|---|---|---|---|---|---|---|---|---|
| de | hard | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 60 | 60 |
| de | standard | 0.98 | 0.98 | 0.98 | 1.00 | 0.99 | 0.99 | 66 | 65 |
| en | hard | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 60 | 60 |
| en | standard | 0.98 | 0.98 | 0.98 | 1.00 | 0.99 | 0.99 | 66 | 65 |
| fr | hard | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 60 | 60 |
| fr | standard | 0.98 | 0.98 | 0.98 | 1.00 | 0.99 | 0.99 | 66 | 65 |
| lb | hard | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 60 | 60 |
| lb | standard | 0.98 | 0.98 | 0.98 | 1.00 | 0.99 | 0.99 | 66 | 65 |

## Names only (person, organization, location, address)

| lang | subset | recall_overlap | recall_exact | precision_overlap | f1_overlap | n_gold | n_pred |
|---|---|---|---|---|---|---|---|
| de | hard | 1.00 | 1.00 | 1.00 | 1.00 | 46 | 46 |
| de | standard | 0.98 | 0.98 | 1.00 | 0.99 | 42 | 41 |
| en | hard | 1.00 | 1.00 | 1.00 | 1.00 | 46 | 46 |
| en | standard | 0.98 | 0.98 | 1.00 | 0.99 | 42 | 41 |
| fr | hard | 1.00 | 1.00 | 1.00 | 1.00 | 46 | 46 |
| fr | standard | 0.98 | 0.98 | 1.00 | 0.99 | 42 | 41 |
| lb | hard | 1.00 | 1.00 | 1.00 | 1.00 | 46 | 46 |
| lb | standard | 0.98 | 0.98 | 1.00 | 0.99 | 42 | 41 |

## False positives on the entity-free subset

| lang | false positives | sentences |
|---|---|---|
| en | 1 | 20 |
| fr | 3 | 20 |
| de | 2 | 20 |
| lb | 4 | 20 |

| id | type | text | score |
|---|---|---|---|
| de-entity_free-04 | location | Berg | 0.55 |
| lb-entity_free-04 | location | Bierg | 0.55 |
| fr-entity_free-05 | location | Gare | 0.55 |
| lb-entity_free-05 | location | Gare | 0.55 |
| fr-entity_free-06 | person | Noël | 0.85 |
| lb-entity_free-11 | location | Stroossen | 0.55 |
| fr-entity_free-13 | location | La Réunion | 0.85 |
| en-entity_free-17 | person | Mett | 0.55 |
| de-entity_free-17 | person | Mett | 0.55 |
| lb-entity_free-17 | person | Mett | 0.55 |

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
| person | 0.98 | 0.98 | 0.98 | 1.00 | 0.99 | 0.99 | 216 | 212 |
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
| de | person | 0.98 | 1.00 | 0.99 | 54 | 53 |
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
| en | person | 0.98 | 1.00 | 0.99 | 54 | 53 |
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
| fr | person | 0.98 | 1.00 | 0.99 | 54 | 53 |
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
| lb | person | 0.98 | 1.00 | 0.99 | 54 | 53 |
| lb | phone | 1.00 | 1.00 | 1.00 | 8 | 8 |
| lb | url | 1.00 | 1.00 | 1.00 | 2 | 2 |

## Gold names found in the gazetteer

Share of distinct gold person, location and organization strings that the built-in gazetteer contains. High values mean rules-only recall on names partly reflects list overlap with the generator.

| type | distinct | in_gazetteer | share |
|---|---|---|---|
| location | 33 | 33 | 1.00 |
| organization | 10 | 10 | 1.00 |
| person | 50 | 41 | 0.82 |

## Missed spans (4)

| id | type | text | covered | sentence |
|---|---|---|---|---|
| en-standard-appointment-0 | person | Cilly Noack | False | Appointment confirmed for Cilly Noack in Dudelange, call 0171 7539086 to cancel. |
| fr-standard-appointment-0 | person | Cilly Noack | False | Rendez-vous confirmé pour Cilly Noack à Dudelange, appelez le 0171 7539086 pour  |
| de-standard-appointment-0 | person | Cilly Noack | False | Termin für Cilly Noack in Düdelingen bestätigt, Absage unter 0171 7539086. |
| lb-standard-appointment-0 | person | Cilly Noack | False | Den Termin fir Cilly Noack zu Diddeleng ass confirméiert, rufft 0171 7539086 un  |
