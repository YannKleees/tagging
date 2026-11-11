# Evaluation: rules_both

Config: `eval/configs/rules_both.json`. 288 sentences, 2637 words, 504 gold spans.

Recall comes first: a missed identifier leaks personal data, an over-redaction only costs readability.
`overlap` counts a prediction of the right type that overlaps the gold span, `exact` needs identical offsets, and `any_type_recall` counts a gold span as redacted if any prediction overlaps it.

Runtime: 7825.33 ms per 1,000 words (detection only).

## By language and subset

| lang | subset | recall_overlap | recall_exact | any_type_recall | precision_overlap | f1_overlap | f1_exact | n_gold | n_pred |
|---|---|---|---|---|---|---|---|---|---|
| de | hard | 0.97 | 0.93 | 0.98 | 0.98 | 0.97 | 0.94 | 60 | 59 |
| de | standard | 1.00 | 0.97 | 1.00 | 1.00 | 1.00 | 0.97 | 66 | 66 |
| en | hard | 1.00 | 0.98 | 1.00 | 1.00 | 1.00 | 0.98 | 60 | 60 |
| en | standard | 1.00 | 1.00 | 1.00 | 0.97 | 0.99 | 0.99 | 66 | 68 |
| fr | hard | 0.98 | 0.97 | 1.00 | 1.00 | 0.99 | 0.97 | 60 | 59 |
| fr | standard | 1.00 | 1.00 | 1.00 | 0.97 | 0.99 | 0.99 | 66 | 68 |
| lb | hard | 1.00 | 0.98 | 1.00 | 1.00 | 1.00 | 0.98 | 60 | 60 |
| lb | standard | 1.00 | 0.97 | 1.00 | 0.97 | 0.99 | 0.96 | 66 | 68 |

## Names only (person, organization, location, address)

| lang | subset | recall_overlap | recall_exact | precision_overlap | f1_overlap | n_gold | n_pred |
|---|---|---|---|---|---|---|---|
| de | hard | 0.96 | 0.91 | 0.98 | 0.97 | 46 | 45 |
| de | standard | 1.00 | 0.95 | 1.00 | 1.00 | 42 | 42 |
| en | hard | 1.00 | 0.98 | 1.00 | 1.00 | 46 | 46 |
| en | standard | 1.00 | 1.00 | 0.95 | 0.98 | 42 | 44 |
| fr | hard | 0.98 | 0.96 | 1.00 | 0.99 | 46 | 45 |
| fr | standard | 1.00 | 1.00 | 0.95 | 0.98 | 42 | 44 |
| lb | hard | 1.00 | 0.98 | 1.00 | 1.00 | 46 | 46 |
| lb | standard | 1.00 | 0.95 | 0.95 | 0.98 | 42 | 44 |

## False positives on the entity-free subset

| lang | false positives | sentences |
|---|---|---|
| en | 3 | 20 |
| fr | 4 | 20 |
| de | 3 | 20 |
| lb | 6 | 20 |

| id | type | text | score |
|---|---|---|---|
| en-entity_free-03 | person | mosquito stung Grandma | 0.70 |
| lb-entity_free-03 | person | Eng Méck | 0.89 |
| lb-entity_free-05 | organization | de Pak an der Gare | 0.64 |
| fr-entity_free-06 | person | Noël | 0.85 |
| lb-entity_free-10 | location | Gaart Fussball | 0.70 |
| en-entity_free-11 | organization | The Council | 0.90 |
| de-entity_free-11 | address | Straßen | 0.75 |
| lb-entity_free-11 | location | Stroossen | 0.55 |
| lb-entity_free-11 | address | Wanter | 0.61 |
| fr-entity_free-13 | location | La Réunion | 0.85 |
| en-entity_free-17 | person | Mett | 0.73 |
| fr-entity_free-17 | person | Mett | 0.70 |
| de-entity_free-17 | person | Mett | 0.70 |
| de-entity_free-17 | location | Zwiebeln | 0.70 |
| lb-entity_free-17 | person | Moies Mett | 0.70 |
| fr-entity_free-19 | person | Maîtresse à la Maternelle | 0.51 |

## By entity type

| type | recall_overlap | recall_exact | any_type_recall | precision_overlap | f1_overlap | f1_exact | n_gold | n_pred |
|---|---|---|---|---|---|---|---|---|
| address | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 8 | 8 |
| credit_card | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 16 | 16 |
| date_of_birth | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 24 | 24 |
| email | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 32 | 32 |
| iban | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 16 | 16 |
| ip_address | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 8 | 8 |
| location | 1.00 | 0.99 | 1.00 | 0.98 | 0.99 | 0.98 | 80 | 82 |
| national_id | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 16 | 16 |
| organization | 0.96 | 0.88 | 1.00 | 0.96 | 0.96 | 0.88 | 48 | 48 |
| person | 1.00 | 0.98 | 1.00 | 0.99 | 0.99 | 0.97 | 216 | 218 |
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
| de | organization | 0.92 | 1.00 | 0.96 | 12 | 11 |
| de | person | 0.98 | 0.98 | 0.98 | 54 | 54 |
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
| en | organization | 1.00 | 0.86 | 0.92 | 12 | 14 |
| en | person | 1.00 | 1.00 | 1.00 | 54 | 54 |
| en | phone | 1.00 | 1.00 | 1.00 | 8 | 8 |
| en | url | 1.00 | 1.00 | 1.00 | 2 | 2 |
| fr | address | 1.00 | 1.00 | 1.00 | 2 | 2 |
| fr | credit_card | 1.00 | 1.00 | 1.00 | 4 | 4 |
| fr | date_of_birth | 1.00 | 1.00 | 1.00 | 6 | 6 |
| fr | email | 1.00 | 1.00 | 1.00 | 8 | 8 |
| fr | iban | 1.00 | 1.00 | 1.00 | 4 | 4 |
| fr | ip_address | 1.00 | 1.00 | 1.00 | 2 | 2 |
| fr | location | 1.00 | 0.91 | 0.95 | 20 | 22 |
| fr | national_id | 1.00 | 1.00 | 1.00 | 4 | 4 |
| fr | organization | 0.92 | 1.00 | 0.96 | 12 | 11 |
| fr | person | 1.00 | 1.00 | 1.00 | 54 | 54 |
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
| lb | person | 1.00 | 0.96 | 0.98 | 54 | 56 |
| lb | phone | 1.00 | 1.00 | 1.00 | 8 | 8 |
| lb | url | 1.00 | 1.00 | 1.00 | 2 | 2 |

## Gold names found in the gazetteer

Share of distinct gold person, location and organization strings that the built-in gazetteer contains. High values mean rules-only recall on names partly reflects list overlap with the generator.

| type | distinct | in_gazetteer | share |
|---|---|---|---|
| location | 33 | 33 | 1.00 |
| organization | 10 | 10 | 1.00 |
| person | 50 | 41 | 0.82 |

## Missed spans (3)

| id | type | text | covered | sentence |
|---|---|---|---|---|
| fr-hard-letter-1 | organization | Cactus | True | Madame Schiltz de Cactus nous a écrit une lettre. |
| de-hard-letter-1 | organization | Cactus | True | Frau Schiltz von Cactus hat uns einen Brief geschrieben. |
| de-hard-family-0 | person | Tun | False | Meine Mutter und Tun kommen am Sonntag nach Haller. |
