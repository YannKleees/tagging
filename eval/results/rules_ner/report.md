# Evaluation: rules_ner

Config: `eval/configs/rules_ner.json`. 288 sentences, 2637 words, 504 gold spans.

Recall comes first: a missed identifier leaks personal data, an over-redaction only costs readability.
`overlap` counts a prediction of the right type that overlaps the gold span, `exact` needs identical offsets, and `any_type_recall` counts a gold span as redacted if any prediction overlaps it.

Runtime: 5785.40 ms per 1,000 words (detection only).

## By language and subset

| lang | subset | recall_overlap | recall_exact | any_type_recall | precision_overlap | f1_overlap | f1_exact | n_gold | n_pred |
|---|---|---|---|---|---|---|---|---|---|
| de | hard | 1.00 | 0.82 | 1.00 | 0.97 | 0.98 | 0.80 | 60 | 62 |
| de | standard | 1.00 | 0.97 | 1.00 | 1.00 | 1.00 | 0.97 | 66 | 66 |
| en | hard | 1.00 | 0.80 | 1.00 | 0.97 | 0.98 | 0.79 | 60 | 62 |
| en | standard | 1.00 | 0.98 | 1.00 | 1.00 | 1.00 | 0.98 | 66 | 66 |
| fr | hard | 0.98 | 0.77 | 1.00 | 1.00 | 0.99 | 0.77 | 60 | 59 |
| fr | standard | 1.00 | 0.98 | 1.00 | 1.00 | 1.00 | 0.98 | 66 | 66 |
| lb | hard | 0.95 | 0.62 | 1.00 | 0.92 | 0.93 | 0.61 | 60 | 62 |
| lb | standard | 1.00 | 0.98 | 1.00 | 1.00 | 1.00 | 0.98 | 66 | 66 |

## Names only (person, organization, location, address)

| lang | subset | recall_overlap | recall_exact | precision_overlap | f1_overlap | n_gold | n_pred |
|---|---|---|---|---|---|---|---|
| de | hard | 1.00 | 0.76 | 0.96 | 0.98 | 46 | 48 |
| de | standard | 1.00 | 0.95 | 1.00 | 1.00 | 42 | 42 |
| en | hard | 1.00 | 0.74 | 0.96 | 0.98 | 46 | 48 |
| en | standard | 1.00 | 0.98 | 1.00 | 1.00 | 42 | 42 |
| fr | hard | 0.98 | 0.70 | 1.00 | 0.99 | 46 | 45 |
| fr | standard | 1.00 | 0.98 | 1.00 | 1.00 | 42 | 42 |
| lb | hard | 0.93 | 0.50 | 0.90 | 0.91 | 46 | 48 |
| lb | standard | 1.00 | 0.98 | 1.00 | 1.00 | 42 | 42 |

## False positives on the entity-free subset

| lang | false positives | sentences |
|---|---|---|
| en | 27 | 20 |
| fr | 25 | 20 |
| de | 21 | 20 |
| lb | 27 | 20 |

| id | type | text | score |
|---|---|---|---|
| en-entity_free-00 | person | Mother and Grandpa | 0.60 |
| fr-entity_free-00 | person | La mère et le grand-père | 0.58 |
| de-entity_free-00 | person | Opa | 0.52 |
| lb-entity_free-00 | organization | D'Mamm an de Bopa | 0.74 |
| en-entity_free-01 | person | The Neighbour | 0.77 |
| en-entity_free-01 | organization | Garage | 0.78 |
| fr-entity_free-01 | person | Le voisin | 0.80 |
| de-entity_free-01 | person | Der Nachbar | 0.63 |
| lb-entity_free-01 | person | Den Noper | 0.70 |
| en-entity_free-02 | person | Director | 0.83 |
| en-entity_free-02 | person | Teacher | 0.81 |
| en-entity_free-02 | person | parents | 0.54 |
| fr-entity_free-02 | person | Le directeur | 0.55 |
| fr-entity_free-02 | person | professeur | 0.55 |
| fr-entity_free-02 | person | parents | 0.62 |
| de-entity_free-02 | person | Der Direktor | 0.61 |
| de-entity_free-02 | person | Eltern | 0.51 |
| lb-entity_free-02 | person | De Direkter an de Proff | 0.80 |
| en-entity_free-03 | person | Grandma | 0.62 |
| fr-entity_free-03 | person | grand-mère | 0.53 |
| de-entity_free-03 | person | Oma | 0.52 |
| lb-entity_free-03 | person | Eng Méck | 0.86 |
| lb-entity_free-03 | organization | Bomi | 0.52 |
| de-entity_free-04 | organization | Wir | 0.54 |
| de-entity_free-04 | location | Berg | 0.55 |
| lb-entity_free-04 | location | Bierg | 0.55 |
| en-entity_free-05 | organization | Post Office | 0.99 |
| en-entity_free-05 | location | Station | 0.60 |
| fr-entity_free-05 | organization | la Poste | 0.78 |
| fr-entity_free-05 | location | Gare | 0.55 |
| de-entity_free-05 | organization | Post | 0.95 |
| lb-entity_free-05 | person | De Bréif | 0.86 |
| lb-entity_free-05 | organization | Post | 0.53 |
| lb-entity_free-05 | location | Gare | 0.55 |
| en-entity_free-06 | person | Auntie and Uncle | 0.66 |
| fr-entity_free-06 | person | La Tante et l'Oncle | 0.69 |
| fr-entity_free-06 | person | Noël | 0.85 |
| lb-entity_free-06 | organization | D'Tatta an de Monni | 0.65 |
| en-entity_free-07 | person | My Sister | 0.74 |
| en-entity_free-07 | organization | Hospital | 0.79 |
| fr-entity_free-07 | person | Ma Sœur | 0.67 |
| fr-entity_free-07 | organization | Hôpital | 0.73 |
| de-entity_free-07 | person | Meine Schwester | 0.60 |
| de-entity_free-07 | organization | Krankenhaus | 0.84 |
| lb-entity_free-07 | person | Meng Schwëster | 0.60 |
| lb-entity_free-07 | organization | Infirmière am Spidol | 0.93 |
| fr-entity_free-09 | organization | Serveur du Bureau | 0.64 |
| en-entity_free-10 | person | Father and Son | 0.61 |
| fr-entity_free-10 | organization | Le Père et le Fils | 0.87 |
| lb-entity_free-10 | person | De Papp an de Jong | 0.80 |
| lb-entity_free-10 | organization | Gaart Fussball | 0.70 |
| en-entity_free-11 | organization | The Council | 0.83 |
| fr-entity_free-11 | organization | La Commune | 0.98 |
| fr-entity_free-11 | address | Rues | 0.80 |
| de-entity_free-11 | organization | Die Gemeinde | 0.87 |
| de-entity_free-11 | address | Straßen | 0.58 |
| lb-entity_free-11 | organization | D'Gemeng | 0.83 |
| lb-entity_free-11 | location | Stroossen | 0.55 |
| lb-entity_free-11 | address | Wanter | 0.88 |
| en-entity_free-12 | person | Nurse | 0.61 |
| en-entity_free-12 | person | Doctor | 0.64 |
| fr-entity_free-12 | organization | L'Infirmière et le Médecin | 0.76 |
| de-entity_free-12 | person | Die Krankenschwester | 0.54 |
| de-entity_free-12 | person | Arzt | 0.52 |
| lb-entity_free-12 | person | D'Infirmière an den Dokter | 0.67 |
| fr-entity_free-13 | location | La Réunion | 0.81 |
| en-entity_free-14 | person | The Boss | 0.70 |
| en-entity_free-14 | person | Secretary | 0.62 |
| en-entity_free-14 | organization | Conference | 0.68 |
| fr-entity_free-14 | person | Le Chef et la Secrétaire | 0.77 |
| de-entity_free-14 | person | Sekretärin | 0.53 |
| lb-entity_free-14 | person | De Chef an d'Sekretärin | 0.69 |
| en-entity_free-15 | person | My Cousin and my Niece | 0.62 |
| en-entity_free-15 | location | Countryside | 0.68 |
| fr-entity_free-15 | person | Mon Cousin et ma Nièce | 0.56 |
| fr-entity_free-15 | location | la Campagne | 0.64 |
| de-entity_free-15 | person | Mein Cousin | 0.58 |
| de-entity_free-15 | person | meine Nichte | 0.52 |
| lb-entity_free-15 | person | Mäi Cousin an d'Niess | 0.73 |
| en-entity_free-16 | person | The Farmer | 0.88 |
| en-entity_free-16 | organization | Winery | 0.84 |
| fr-entity_free-16 | person | Le Paysan | 0.85 |
| fr-entity_free-16 | organization | Vin à la Cave | 0.53 |
| de-entity_free-16 | person | Der Bauer | 0.75 |
| de-entity_free-16 | organization | Kellerei | 0.73 |
| lb-entity_free-16 | person | De Bauer | 0.75 |
| lb-entity_free-16 | organization | Wäin | 0.51 |
| lb-entity_free-16 | organization | Kellerei | 0.55 |
| en-entity_free-17 | person | Mett | 0.55 |
| de-entity_free-17 | person | Mett | 0.55 |
| lb-entity_free-17 | person | Mett | 0.55 |
| en-entity_free-18 | organization | The Train into Town | 0.65 |
| fr-entity_free-18 | organization | Le Train pour la Ville | 0.93 |
| lb-entity_free-18 | organization | Den Zuch op d'Stad | 0.84 |
| en-entity_free-19 | person | The Children | 0.60 |
| en-entity_free-19 | organization | Nursery | 0.64 |
| fr-entity_free-19 | organization | Les Enfants | 0.63 |
| de-entity_free-19 | organization | Kindergarten | 0.90 |
| lb-entity_free-19 | organization | D'Kanner | 0.89 |
| lb-entity_free-19 | location | Kannergaart | 0.62 |

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
| organization | 1.00 | 0.90 | 1.00 | 0.96 | 0.98 | 0.88 | 48 | 50 |
| person | 0.98 | 0.72 | 1.00 | 0.97 | 0.97 | 0.72 | 216 | 219 |
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
| de | person | 1.00 | 0.96 | 0.98 | 54 | 56 |
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
| en | person | 1.00 | 0.96 | 0.98 | 54 | 56 |
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
| lb | organization | 1.00 | 0.86 | 0.92 | 12 | 14 |
| lb | person | 0.94 | 0.94 | 0.94 | 54 | 54 |
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
| lb-hard-coordinated-0 | person | Tessy | True | D'Mady an d'Tessy sinn op Lellgen gefuer. |
| lb-hard-coordinated-1 | person | Josée | True | D'Léontine an d'Josée sinn op Ohn gefuer. |
| fr-hard-chat-0 | person | léontine | True | rené et léontine viennent ce soir |
| lb-hard-chat-0 | person | léontine | True | rené an léontine kommen haut den owend |
