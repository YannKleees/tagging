# Evaluation: rules_ner_post

Config: `eval/configs/rules_ner_post.json`. 288 sentences, 2637 words, 504 gold spans.

Recall comes first: a missed identifier leaks personal data, an over-redaction only costs readability.
`overlap` counts a prediction of the right type that overlaps the gold span, `exact` needs identical offsets, and `any_type_recall` counts a gold span as redacted if any prediction overlaps it.

Runtime: 46825.49 ms per 1,000 words (detection only).

## By language and subset

| lang | subset | recall_overlap | recall_exact | any_type_recall | precision_overlap | f1_overlap | f1_exact | n_gold | n_pred |
|---|---|---|---|---|---|---|---|---|---|
| de | hard | 0.98 | 0.97 | 0.98 | 1.00 | 0.99 | 0.97 | 60 | 59 |
| de | standard | 0.98 | 0.95 | 1.00 | 0.98 | 0.98 | 0.95 | 66 | 66 |
| en | hard | 0.92 | 0.88 | 0.92 | 1.00 | 0.96 | 0.92 | 60 | 55 |
| en | standard | 0.97 | 0.92 | 1.00 | 1.00 | 0.98 | 0.94 | 66 | 64 |
| fr | hard | 0.97 | 0.93 | 0.97 | 1.00 | 0.98 | 0.95 | 60 | 58 |
| fr | standard | 0.98 | 0.97 | 0.98 | 1.00 | 0.99 | 0.98 | 66 | 65 |
| lb | hard | 0.93 | 0.92 | 0.93 | 1.00 | 0.97 | 0.95 | 60 | 56 |
| lb | standard | 0.98 | 0.97 | 1.00 | 0.98 | 0.98 | 0.97 | 66 | 66 |

## Names only (person, organization, location, address)

| lang | subset | recall_overlap | recall_exact | precision_overlap | f1_overlap | n_gold | n_pred |
|---|---|---|---|---|---|---|---|
| de | hard | 0.98 | 0.96 | 1.00 | 0.99 | 46 | 45 |
| de | standard | 0.98 | 0.93 | 0.98 | 0.98 | 42 | 42 |
| en | hard | 0.93 | 0.89 | 1.00 | 0.97 | 46 | 43 |
| en | standard | 0.95 | 0.88 | 1.00 | 0.98 | 42 | 40 |
| fr | hard | 0.96 | 0.91 | 1.00 | 0.98 | 46 | 44 |
| fr | standard | 0.98 | 0.95 | 1.00 | 0.99 | 42 | 41 |
| lb | hard | 0.91 | 0.89 | 1.00 | 0.95 | 46 | 42 |
| lb | standard | 0.98 | 0.95 | 0.98 | 0.98 | 42 | 42 |

## False positives on the entity-free subset

| lang | false positives | sentences |
|---|---|---|
| en | 9 | 20 |
| fr | 7 | 20 |
| de | 4 | 20 |
| lb | 10 | 20 |

| id | type | text | score |
|---|---|---|---|
| en-entity_free-02 | person | Director | 0.83 |
| lb-entity_free-03 | person | Eng Méck | 0.86 |
| en-entity_free-05 | organization | Post Office | 0.99 |
| de-entity_free-05 | organization | Post | 0.95 |
| lb-entity_free-05 | person | Bréif | 0.86 |
| lb-entity_free-05 | organization | Post | 0.53 |
| lb-entity_free-07 | organization | Infirmière am Spidol | 0.93 |
| fr-entity_free-09 | organization | Serveur du Bureau | 0.64 |
| lb-entity_free-10 | organization | Gaart Fussball | 0.70 |
| en-entity_free-11 | organization | The Council | 0.83 |
| fr-entity_free-11 | address | Rues | 0.80 |
| de-entity_free-11 | address | Straßen | 0.58 |
| lb-entity_free-11 | address | Wanter | 0.88 |
| fr-entity_free-13 | location | La Réunion | 0.81 |
| en-entity_free-14 | organization | Conference | 0.68 |
| en-entity_free-15 | location | Countryside | 0.68 |
| fr-entity_free-15 | location | la Campagne | 0.64 |
| en-entity_free-16 | organization | Winery | 0.84 |
| fr-entity_free-16 | person | Paysan | 0.85 |
| fr-entity_free-16 | organization | Vin à la Cave | 0.53 |
| de-entity_free-16 | organization | Kellerei | 0.73 |
| lb-entity_free-16 | organization | Wäin | 0.51 |
| lb-entity_free-16 | organization | Kellerei | 0.55 |
| en-entity_free-17 | person | Mett | 0.55 |
| en-entity_free-18 | organization | The Train into Town | 0.65 |
| fr-entity_free-18 | organization | Le Train pour la Ville | 0.93 |
| lb-entity_free-18 | organization | Den Zuch op d'Stad | 0.84 |
| en-entity_free-19 | organization | Nursery | 0.64 |
| de-entity_free-19 | organization | Kindergarten | 0.90 |
| lb-entity_free-19 | location | Kannergaart | 0.62 |

## By entity type

| type | recall_overlap | recall_exact | any_type_recall | precision_overlap | f1_overlap | f1_exact | n_gold | n_pred |
|---|---|---|---|---|---|---|---|---|
| address | 1.00 | 0.75 | 1.00 | 1.00 | 1.00 | 0.75 | 8 | 8 |
| credit_card | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 16 | 16 |
| date_of_birth | 0.92 | 0.92 | 0.92 | 1.00 | 0.96 | 0.96 | 24 | 22 |
| email | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 32 | 32 |
| iban | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 16 | 16 |
| ip_address | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 8 | 8 |
| location | 0.97 | 0.97 | 1.00 | 0.97 | 0.97 | 0.97 | 80 | 80 |
| national_id | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 16 | 16 |
| organization | 0.98 | 0.88 | 0.98 | 1.00 | 0.99 | 0.88 | 48 | 47 |
| person | 0.94 | 0.92 | 0.95 | 1.00 | 0.97 | 0.94 | 216 | 204 |
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
| de | location | 1.00 | 0.95 | 0.98 | 20 | 21 |
| de | national_id | 1.00 | 1.00 | 1.00 | 4 | 4 |
| de | organization | 1.00 | 1.00 | 1.00 | 12 | 12 |
| de | person | 0.96 | 1.00 | 0.98 | 54 | 52 |
| de | phone | 1.00 | 1.00 | 1.00 | 8 | 8 |
| de | url | 1.00 | 1.00 | 1.00 | 2 | 2 |
| en | address | 1.00 | 1.00 | 1.00 | 2 | 2 |
| en | credit_card | 1.00 | 1.00 | 1.00 | 4 | 4 |
| en | date_of_birth | 0.67 | 1.00 | 0.80 | 6 | 4 |
| en | email | 1.00 | 1.00 | 1.00 | 8 | 8 |
| en | iban | 1.00 | 1.00 | 1.00 | 4 | 4 |
| en | ip_address | 1.00 | 1.00 | 1.00 | 2 | 2 |
| en | location | 0.90 | 1.00 | 0.95 | 20 | 18 |
| en | national_id | 1.00 | 1.00 | 1.00 | 4 | 4 |
| en | organization | 1.00 | 1.00 | 1.00 | 12 | 12 |
| en | person | 0.94 | 1.00 | 0.97 | 54 | 51 |
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
| fr | organization | 0.92 | 1.00 | 0.96 | 12 | 11 |
| fr | person | 0.96 | 1.00 | 0.98 | 54 | 52 |
| fr | phone | 1.00 | 1.00 | 1.00 | 8 | 8 |
| fr | url | 1.00 | 1.00 | 1.00 | 2 | 2 |
| lb | address | 1.00 | 1.00 | 1.00 | 2 | 2 |
| lb | credit_card | 1.00 | 1.00 | 1.00 | 4 | 4 |
| lb | date_of_birth | 1.00 | 1.00 | 1.00 | 6 | 6 |
| lb | email | 1.00 | 1.00 | 1.00 | 8 | 8 |
| lb | iban | 1.00 | 1.00 | 1.00 | 4 | 4 |
| lb | ip_address | 1.00 | 1.00 | 1.00 | 2 | 2 |
| lb | location | 1.00 | 0.95 | 0.98 | 20 | 21 |
| lb | national_id | 1.00 | 1.00 | 1.00 | 4 | 4 |
| lb | organization | 1.00 | 1.00 | 1.00 | 12 | 12 |
| lb | person | 0.91 | 1.00 | 0.95 | 54 | 49 |
| lb | phone | 1.00 | 1.00 | 1.00 | 8 | 8 |
| lb | url | 1.00 | 1.00 | 1.00 | 2 | 2 |

## Gold names found in the gazetteer

Share of distinct gold person, location and organization strings that the built-in gazetteer contains. High values mean rules-only recall on names partly reflects list overlap with the generator.

| type | distinct | in_gazetteer | share |
|---|---|---|---|
| location | 33 | 29 | 0.88 |
| organization | 10 | 8 | 0.80 |
| person | 50 | 16 | 0.32 |

## Missed spans (17)

| id | type | text | covered | sentence |
|---|---|---|---|---|
| fr-standard-matricule-1 | person | Paula Noble | False | Matricule de Paula Noble : 1960032638746. |
| en-standard-address-0 | location | Berlin | True | Marcus Butler lives at 98, rue de l'Église, Berlin. |
| en-standard-address-1 | location | Luxembourg | True | Josée Kremer lives at 12, rue de l'Église, Luxembourg. |
| de-standard-meeting-0 | person | Mett Gloden | True | Mett Gloden hat Raoul Reuter am Montag in den Büros von Cargolux getroffen. |
| lb-standard-meeting-0 | person | Mett Gloden | True | Mett Gloden huet de Méindeg Raoul Reuter an de Büroe vun Cargolux getraff. |
| fr-hard-letter-1 | organization | Cactus | False | Madame Schiltz de Cactus nous a écrit une lettre. |
| lb-hard-family-0 | person | Tun | False | Meng Mamm an den Tun kommen e Sonndeg op Haler. |
| en-hard-chat-1 | person | ernest | False | ernest and gusty are coming tonight |
| en-hard-chat-1 | person | gusty | False | ernest and gusty are coming tonight |
| lb-hard-chat-1 | person | ernest | False | ernest an gusty kommen haut den owend |
| lb-hard-chat-1 | person | gusty | False | ernest an gusty kommen haut den owend |
| en-hard-born-0 | date_of_birth | 06/05/1970 | False | Méck, b. 06/05/1970 in Sanem. |
| en-hard-born-1 | date_of_birth | 3 February 1992 | False | Pol, b. 3 February 1992 in Berdorf. |
| en-hard-text-0 | person | Aline | False | Email Aline at aline83@education.lu or text 0352 621 864014. |
| fr-hard-text-0 | person | Aline | False | Écris à Aline sur aline83@education.lu ou par SMS au 0352 621 864014. |
| de-hard-text-0 | person | Aline | False | Schreib Aline an aline83@education.lu oder eine SMS an 0352 621 864014. |
| lb-hard-text-0 | person | Aline | False | Schreif der Aline op aline83@education.lu oder eng SMS op 0352 621 864014. |
