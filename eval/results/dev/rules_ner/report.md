# Evaluation: dev/rules_ner

Config: `eval/configs/rules_ner.json`. 248 sentences, 2297 words, 504 gold spans.

Recall comes first: a missed identifier leaks personal data, an over-redaction only costs readability.
`overlap` counts a prediction of the right type that overlaps the gold span, `exact` needs identical offsets, and `any_type_recall` counts a gold span as redacted if any prediction overlaps it.

Runtime: 6021.63 ms per 1,000 words (detection only).

## By language and subset

| lang | subset | recall_overlap | recall_exact | any_type_recall | precision_overlap | f1_overlap | f1_exact | n_gold | n_pred |
|---|---|---|---|---|---|---|---|---|---|
| de | hard | 0.98 | 0.77 | 1.00 | 0.97 | 0.98 | 0.76 | 60 | 61 |
| de | standard | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 66 | 66 |
| en | hard | 0.98 | 0.77 | 1.00 | 0.95 | 0.97 | 0.75 | 60 | 62 |
| en | standard | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 66 | 66 |
| fr | hard | 0.98 | 0.77 | 1.00 | 0.98 | 0.98 | 0.77 | 60 | 60 |
| fr | standard | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 66 | 66 |
| lb | hard | 0.97 | 0.67 | 1.00 | 0.97 | 0.97 | 0.67 | 60 | 60 |
| lb | standard | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 66 | 66 |

## Names only (person, organization, location, address)

| lang | subset | recall_overlap | recall_exact | precision_overlap | f1_overlap | n_gold | n_pred |
|---|---|---|---|---|---|---|---|
| de | hard | 0.98 | 0.70 | 0.96 | 0.97 | 46 | 47 |
| de | standard | 1.00 | 1.00 | 1.00 | 1.00 | 42 | 42 |
| en | hard | 0.98 | 0.70 | 0.94 | 0.96 | 46 | 48 |
| en | standard | 1.00 | 1.00 | 1.00 | 1.00 | 42 | 42 |
| fr | hard | 0.98 | 0.70 | 0.98 | 0.98 | 46 | 46 |
| fr | standard | 1.00 | 1.00 | 1.00 | 1.00 | 42 | 42 |
| lb | hard | 0.96 | 0.57 | 0.96 | 0.96 | 46 | 46 |
| lb | standard | 1.00 | 1.00 | 1.00 | 1.00 | 42 | 42 |

## False positives on the entity-free subset

| lang | false positives | sentences |
|---|---|---|
| en | 12 | 10 |
| fr | 11 | 10 |
| de | 7 | 10 |
| lb | 7 | 10 |

| id | type | text | score |
|---|---|---|---|
| en-entity_free-00 | person | Grandma | 0.57 |
| en-entity_free-00 | person | Neighbour | 0.51 |
| fr-entity_free-00 | person | La Grand-mère et la Voisine | 0.84 |
| fr-entity_free-00 | organization | la Cuisine | 0.87 |
| de-entity_free-00 | person | Nachbarin | 0.55 |
| en-entity_free-01 | person | The Mayor | 0.76 |
| en-entity_free-01 | organization | new School | 0.75 |
| fr-entity_free-01 | person | Le Maire | 0.82 |
| fr-entity_free-01 | organization | École | 0.77 |
| de-entity_free-01 | person | Der Bürgermeister | 0.57 |
| de-entity_free-01 | organization | Schule | 0.62 |
| lb-entity_free-01 | person | De Buergermeeschter | 0.63 |
| lb-entity_free-01 | organization | Schoul | 0.54 |
| en-entity_free-02 | person | My Father | 0.53 |
| fr-entity_free-02 | person | Mon Père et mon Oncle | 0.56 |
| fr-entity_free-02 | organization | la Grange | 0.59 |
| de-entity_free-02 | person | Mein Vater | 0.63 |
| en-entity_free-04 | person | The Boy and the Girl | 0.58 |
| fr-entity_free-04 | person | Le Garçon et la Fille | 0.64 |
| lb-entity_free-04 | person | De Jong an d'Meedchen | 0.68 |
| en-entity_free-05 | organization | Pharmacist | 0.59 |
| en-entity_free-05 | person | Patient | 0.72 |
| fr-entity_free-05 | organization | Le Pharmacien | 0.96 |
| fr-entity_free-05 | person | Patient | 0.80 |
| de-entity_free-05 | person | Der Apotheker | 0.56 |
| lb-entity_free-05 | organization | Den Apdikter | 0.78 |
| en-entity_free-06 | person | We | 0.50 |
| de-entity_free-06 | person | Wir | 0.53 |
| lb-entity_free-06 | person | Mir | 0.57 |
| en-entity_free-07 | organization | The Coach and the Players | 0.82 |
| en-entity_free-07 | location | Changing Room | 0.59 |
| fr-entity_free-07 | person | L'Entraîneur et les Joueurs | 0.76 |
| lb-entity_free-07 | person | Den Trainer an d'Spiller | 0.55 |
| en-entity_free-09 | organization | The Baker and the Butcher | 0.96 |
| fr-entity_free-09 | organization | Le Boulanger et le Boucher | 0.96 |
| de-entity_free-09 | organization | Der Bäcker und der Metzger | 0.94 |
| lb-entity_free-09 | organization | De Bäcker an de Metzler | 0.82 |

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
| organization | 1.00 | 1.00 | 1.00 | 0.96 | 0.98 | 0.98 | 48 | 50 |
| person | 0.98 | 0.71 | 1.00 | 0.97 | 0.97 | 0.71 | 216 | 217 |
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
| de | person | 0.98 | 0.96 | 0.97 | 54 | 55 |
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
| en | person | 0.98 | 0.95 | 0.96 | 54 | 56 |
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
| fr | person | 0.98 | 0.98 | 0.98 | 54 | 54 |
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
| lb | organization | 1.00 | 0.86 | 0.92 | 12 | 14 |
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

## Missed spans (5)

| id | type | text | covered | sentence |
|---|---|---|---|---|
| en-hard-coordinated-0 | person | Charel | True | Lis and Charel drove to Betzdorf. |
| fr-hard-coordinated-0 | person | Charel | True | Lis et Charel sont partis à Betzdorf. |
| de-hard-coordinated-0 | person | Charel | True | Lis und Charel sind nach Betzdorf gefahren. |
| lb-hard-coordinated-0 | person | Charel | True | D'Lis an de Charel sinn op Betzder gefuer. |
| lb-hard-coordinated-1 | person | Léontine | True | De Méck an d'Léontine sinn op Buerschent gefuer. |
