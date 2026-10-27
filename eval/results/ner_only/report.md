# Evaluation: ner_only

Config: `eval/configs/ner_only.json`. 288 sentences, 2637 words, 504 gold spans.

Recall comes first: a missed identifier leaks personal data, an over-redaction only costs readability.
`overlap` counts a prediction of the right type that overlaps the gold span, `exact` needs identical offsets, and `any_type_recall` counts a gold span as redacted if any prediction overlaps it.

Runtime: 46808.97 ms per 1,000 words (detection only).

## By language and subset

| lang | subset | recall_overlap | recall_exact | any_type_recall | precision_overlap | f1_overlap | f1_exact | n_gold | n_pred |
|---|---|---|---|---|---|---|---|---|---|
| de | hard | 0.70 | 0.52 | 0.77 | 0.88 | 0.78 | 0.57 | 60 | 48 |
| de | standard | 0.62 | 0.59 | 0.68 | 0.91 | 0.74 | 0.70 | 66 | 45 |
| en | hard | 0.67 | 0.47 | 0.70 | 0.91 | 0.77 | 0.54 | 60 | 44 |
| en | standard | 0.61 | 0.56 | 0.68 | 0.93 | 0.73 | 0.68 | 66 | 43 |
| fr | hard | 0.70 | 0.48 | 0.72 | 1.00 | 0.82 | 0.57 | 60 | 42 |
| fr | standard | 0.62 | 0.59 | 0.67 | 0.91 | 0.74 | 0.70 | 66 | 45 |
| lb | hard | 0.57 | 0.23 | 0.68 | 0.79 | 0.66 | 0.27 | 60 | 43 |
| lb | standard | 0.62 | 0.61 | 0.68 | 0.91 | 0.74 | 0.72 | 66 | 45 |

## Names only (person, organization, location, address)

| lang | subset | recall_overlap | recall_exact | precision_overlap | f1_overlap | n_gold | n_pred |
|---|---|---|---|---|---|---|---|
| de | hard | 0.91 | 0.67 | 0.88 | 0.89 | 46 | 48 |
| de | standard | 0.98 | 0.93 | 0.91 | 0.94 | 42 | 45 |
| en | hard | 0.87 | 0.61 | 0.91 | 0.89 | 46 | 44 |
| en | standard | 0.95 | 0.88 | 0.93 | 0.94 | 42 | 43 |
| fr | hard | 0.91 | 0.63 | 1.00 | 0.95 | 46 | 42 |
| fr | standard | 0.98 | 0.93 | 0.91 | 0.94 | 42 | 45 |
| lb | hard | 0.74 | 0.30 | 0.79 | 0.76 | 46 | 43 |
| lb | standard | 0.98 | 0.95 | 0.91 | 0.94 | 42 | 45 |

## False positives on the entity-free subset

| lang | false positives | sentences |
|---|---|---|
| en | 27 | 20 |
| fr | 23 | 20 |
| de | 19 | 20 |
| lb | 23 | 20 |

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
| en-entity_free-05 | organization | Post Office | 0.99 |
| en-entity_free-05 | location | Station | 0.60 |
| fr-entity_free-05 | organization | la Poste | 0.78 |
| de-entity_free-05 | organization | Post | 0.95 |
| lb-entity_free-05 | person | De Bréif | 0.86 |
| lb-entity_free-05 | organization | Post | 0.53 |
| en-entity_free-06 | person | Auntie and Uncle | 0.66 |
| fr-entity_free-06 | person | La Tante et l'Oncle | 0.69 |
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
| address | 1.00 | 0.75 | 1.00 | 1.00 | 1.00 | 0.75 | 8 | 8 |
| credit_card | 0.00 | 0.00 | 0.00 | – | 0.00 | 0.00 | 16 | 0 |
| date_of_birth | 0.00 | 0.00 | 0.00 | – | 0.00 | 0.00 | 24 | 0 |
| email | 0.00 | 0.00 | 0.47 | – | 0.00 | 0.00 | 32 | 0 |
| iban | 0.00 | 0.00 | 0.00 | – | 0.00 | 0.00 | 16 | 0 |
| ip_address | 0.00 | 0.00 | 0.00 | – | 0.00 | 0.00 | 8 | 0 |
| location | 0.95 | 0.95 | 0.97 | 0.89 | 0.92 | 0.92 | 80 | 85 |
| national_id | 0.00 | 0.00 | 0.00 | – | 0.00 | 0.00 | 16 | 0 |
| organization | 0.85 | 0.73 | 0.98 | 0.95 | 0.90 | 0.77 | 48 | 43 |
| person | 0.91 | 0.65 | 0.94 | 0.89 | 0.90 | 0.64 | 216 | 219 |
| phone | 0.00 | 0.00 | 0.00 | – | 0.00 | 0.00 | 32 | 0 |
| url | 0.00 | 0.00 | 0.12 | – | 0.00 | 0.00 | 8 | 0 |

## By language and entity type (overlap)

| lang | type | recall_overlap | precision_overlap | f1_overlap | n_gold | n_pred |
|---|---|---|---|---|---|---|
| de | address | 1.00 | 1.00 | 1.00 | 2 | 2 |
| de | credit_card | 0.00 | – | 0.00 | 4 | 0 |
| de | date_of_birth | 0.00 | – | 0.00 | 6 | 0 |
| de | email | 0.00 | – | 0.00 | 8 | 0 |
| de | iban | 0.00 | – | 0.00 | 4 | 0 |
| de | ip_address | 0.00 | – | 0.00 | 2 | 0 |
| de | location | 1.00 | 0.87 | 0.93 | 20 | 23 |
| de | national_id | 0.00 | – | 0.00 | 4 | 0 |
| de | organization | 0.83 | 1.00 | 0.91 | 12 | 10 |
| de | person | 0.94 | 0.88 | 0.91 | 54 | 58 |
| de | phone | 0.00 | – | 0.00 | 8 | 0 |
| de | url | 0.00 | – | 0.00 | 2 | 0 |
| en | address | 1.00 | 1.00 | 1.00 | 2 | 2 |
| en | credit_card | 0.00 | – | 0.00 | 4 | 0 |
| en | date_of_birth | 0.00 | – | 0.00 | 6 | 0 |
| en | email | 0.00 | – | 0.00 | 8 | 0 |
| en | iban | 0.00 | – | 0.00 | 4 | 0 |
| en | ip_address | 0.00 | – | 0.00 | 2 | 0 |
| en | location | 0.90 | 0.90 | 0.90 | 20 | 20 |
| en | national_id | 0.00 | – | 0.00 | 4 | 0 |
| en | organization | 0.83 | 1.00 | 0.91 | 12 | 10 |
| en | person | 0.93 | 0.91 | 0.92 | 54 | 55 |
| en | phone | 0.00 | – | 0.00 | 8 | 0 |
| en | url | 0.00 | – | 0.00 | 2 | 0 |
| fr | address | 1.00 | 1.00 | 1.00 | 2 | 2 |
| fr | credit_card | 0.00 | – | 0.00 | 4 | 0 |
| fr | date_of_birth | 0.00 | – | 0.00 | 6 | 0 |
| fr | email | 0.00 | – | 0.00 | 8 | 0 |
| fr | iban | 0.00 | – | 0.00 | 4 | 0 |
| fr | ip_address | 0.00 | – | 0.00 | 2 | 0 |
| fr | location | 1.00 | 0.95 | 0.98 | 20 | 21 |
| fr | national_id | 0.00 | – | 0.00 | 4 | 0 |
| fr | organization | 0.92 | 1.00 | 0.96 | 12 | 11 |
| fr | person | 0.93 | 0.94 | 0.93 | 54 | 53 |
| fr | phone | 0.00 | – | 0.00 | 8 | 0 |
| fr | url | 0.00 | – | 0.00 | 2 | 0 |
| lb | address | 1.00 | 1.00 | 1.00 | 2 | 2 |
| lb | credit_card | 0.00 | – | 0.00 | 4 | 0 |
| lb | date_of_birth | 0.00 | – | 0.00 | 6 | 0 |
| lb | email | 0.00 | – | 0.00 | 8 | 0 |
| lb | iban | 0.00 | – | 0.00 | 4 | 0 |
| lb | ip_address | 0.00 | – | 0.00 | 2 | 0 |
| lb | location | 0.90 | 0.86 | 0.88 | 20 | 21 |
| lb | national_id | 0.00 | – | 0.00 | 4 | 0 |
| lb | organization | 0.83 | 0.83 | 0.83 | 12 | 12 |
| lb | person | 0.83 | 0.85 | 0.84 | 54 | 53 |
| lb | phone | 0.00 | – | 0.00 | 8 | 0 |
| lb | url | 0.00 | – | 0.00 | 2 | 0 |

## Gold names found in the gazetteer

Share of distinct gold person, location and organization strings that the built-in gazetteer contains. High values mean rules-only recall on names partly reflects list overlap with the generator.

| type | distinct | in_gazetteer | share |
|---|---|---|---|
| location | 33 | 29 | 0.88 |
| organization | 10 | 8 | 0.80 |
| person | 50 | 16 | 0.32 |

## Missed spans (183)

| id | type | text | covered | sentence |
|---|---|---|---|---|
| en-standard-contact-0 | email | gilbert.muller@post.lu | False | Gilbert Muller can be reached at gilbert.muller@post.lu or on 00352 691 042 147. |
| en-standard-contact-0 | phone | 00352 691 042 147 | False | Gilbert Muller can be reached at gilbert.muller@post.lu or on 00352 691 042 147. |
| fr-standard-contact-0 | email | gilbert.muller@post.lu | False | On peut joindre Gilbert Muller à l'adresse gilbert.muller@post.lu ou au 00352 69 |
| fr-standard-contact-0 | phone | 00352 691 042 147 | False | On peut joindre Gilbert Muller à l'adresse gilbert.muller@post.lu ou au 00352 69 |
| de-standard-contact-0 | email | gilbert.muller@post.lu | False | Gilbert Muller ist unter gilbert.muller@post.lu oder 00352 691 042 147 erreichba |
| de-standard-contact-0 | phone | 00352 691 042 147 | False | Gilbert Muller ist unter gilbert.muller@post.lu oder 00352 691 042 147 erreichba |
| lb-standard-contact-0 | email | gilbert.muller@post.lu | False | Gilbert Muller ass ënner gilbert.muller@post.lu oder um 00352 691 042 147 ze err |
| lb-standard-contact-0 | phone | 00352 691 042 147 | False | Gilbert Muller ass ënner gilbert.muller@post.lu oder um 00352 691 042 147 ze err |
| en-standard-contact-1 | email | jochen.hoffmann@example.com | False | Jochen Hoffmann can be reached at jochen.hoffmann@example.com or on +33 6 52 43  |
| en-standard-contact-1 | phone | +33 6 52 43 28 13 | False | Jochen Hoffmann can be reached at jochen.hoffmann@example.com or on +33 6 52 43  |
| fr-standard-contact-1 | email | jochen.hoffmann@example.com | False | On peut joindre Jochen Hoffmann à l'adresse jochen.hoffmann@example.com ou au +3 |
| fr-standard-contact-1 | phone | +33 6 52 43 28 13 | False | On peut joindre Jochen Hoffmann à l'adresse jochen.hoffmann@example.com ou au +3 |
| de-standard-contact-1 | email | jochen.hoffmann@example.com | False | Jochen Hoffmann ist unter jochen.hoffmann@example.com oder +33 6 52 43 28 13 err |
| de-standard-contact-1 | phone | +33 6 52 43 28 13 | False | Jochen Hoffmann ist unter jochen.hoffmann@example.com oder +33 6 52 43 28 13 err |
| lb-standard-contact-1 | email | jochen.hoffmann@example.com | False | Jochen Hoffmann ass ënner jochen.hoffmann@example.com oder um +33 6 52 43 28 13  |
| lb-standard-contact-1 | phone | +33 6 52 43 28 13 | False | Jochen Hoffmann ass ënner jochen.hoffmann@example.com oder um +33 6 52 43 28 13  |
| en-standard-birth-0 | date_of_birth | 03/02/1961 | False | Norbert Muller was born on 03/02/1961 in Luxembourg. |
| fr-standard-birth-0 | date_of_birth | 03/02/1961 | False | Norbert Muller (né(e) le 03/02/1961 à Luxembourg) a signé le contrat. |
| de-standard-birth-0 | date_of_birth | 03.02.1961 | False | Norbert Muller wurde am 03.02.1961 in Luxemburg geboren. |
| lb-standard-birth-0 | date_of_birth | 03.02.1961 | False | Norbert Muller ass den 03.02.1961 zu Lëtzebuerg gebuer. |
| en-standard-birth-1 | date_of_birth | 05/11/1955 | False | Manon Grondin was born on 05/11/1955 in Luxembourg. |
| fr-standard-birth-1 | date_of_birth | 05/11/1955 | False | Manon Grondin (né(e) le 05/11/1955 à Luxembourg) a signé le contrat. |
| de-standard-birth-1 | date_of_birth | 05.11.1955 | False | Manon Grondin wurde am 05.11.1955 in Luxemburg geboren. |
| lb-standard-birth-1 | date_of_birth | 05.11.1955 | False | Manon Grondin ass den 05.11.1955 zu Lëtzebuerg gebuer. |
| en-standard-transfer-0 | iban | DE21 3778 6504 8622 0152 34 | False | Please transfer the deposit to Olga Hecker, IBAN DE21 3778 6504 8622 0152 34. |
| fr-standard-transfer-0 | iban | DE21 3778 6504 8622 0152 34 | False | Veuillez virer la caution à Olga Hecker, IBAN DE21 3778 6504 8622 0152 34. |
| de-standard-transfer-0 | iban | DE21 3778 6504 8622 0152 34 | False | Bitte überweisen Sie die Kaution an Olga Hecker, IBAN DE21 3778 6504 8622 0152 3 |
| lb-standard-transfer-0 | iban | DE21 3778 6504 8622 0152 34 | False | Iwwerweist d'Kautioun w.e.g. un Olga Hecker, IBAN DE21 3778 6504 8622 0152 34. |
| en-standard-transfer-1 | iban | DE82 0521 5779 9605 8420 52 | False | Please transfer the deposit to Lucy Peron, IBAN DE82 0521 5779 9605 8420 52. |
| fr-standard-transfer-1 | iban | DE82 0521 5779 9605 8420 52 | False | Veuillez virer la caution à Lucy Peron, IBAN DE82 0521 5779 9605 8420 52. |
| de-standard-transfer-1 | iban | DE82 0521 5779 9605 8420 52 | False | Bitte überweisen Sie die Kaution an Lucy Peron, IBAN DE82 0521 5779 9605 8420 52 |
| lb-standard-transfer-1 | iban | DE82 0521 5779 9605 8420 52 | False | Iwwerweist d'Kautioun w.e.g. un Lucy Peron, IBAN DE82 0521 5779 9605 8420 52. |
| en-standard-card-0 | credit_card | 4406 2904 2147 6242 | False | Hans-Günther Ehlert paid the booking with card 4406 2904 2147 6242. |
| fr-standard-card-0 | credit_card | 4406 2904 2147 6242 | False | Hans-Günther Ehlert a payé la réservation avec la carte 4406 2904 2147 6242. |
| de-standard-card-0 | credit_card | 4406 2904 2147 6242 | False | Hans-Günther Ehlert hat die Buchung mit der Karte 4406 2904 2147 6242 bezahlt. |
| lb-standard-card-0 | credit_card | 4406 2904 2147 6242 | False | Hans-Günther Ehlert huet d'Reservatioun mat der Kaart 4406 2904 2147 6242 bezuel |
| en-standard-card-1 | credit_card | 3756 515947 89270 | False | Susan Baker paid the booking with card 3756 515947 89270. |
| fr-standard-card-1 | credit_card | 3756 515947 89270 | False | Susan Baker a payé la réservation avec la carte 3756 515947 89270. |
| de-standard-card-1 | credit_card | 3756 515947 89270 | False | Susan Baker hat die Buchung mit der Karte 3756 515947 89270 bezahlt. |
| lb-standard-card-1 | credit_card | 3756 515947 89270 | False | Susan Baker huet d'Reservatioun mat der Kaart 3756 515947 89270 bezuelt. |
| en-standard-matricule-0 | national_id | 1996022954388 | False | Social security number of Kätti Weber: 1996022954388. |
| fr-standard-matricule-0 | national_id | 1996022954388 | False | Matricule de Kätti Weber : 1996022954388. |
| de-standard-matricule-0 | national_id | 1996022954388 | False | Sozialversicherungsnummer von Kätti Weber: 1996022954388. |
| lb-standard-matricule-0 | national_id | 1996022954388 | False | D'Matrikelnummer fir Kätti Weber ass 1996022954388. |
| en-standard-matricule-1 | national_id | 1960032638746 | False | Social security number of Paula Noble: 1960032638746. |
| fr-standard-matricule-1 | person | Paula Noble | False | Matricule de Paula Noble : 1960032638746. |
| fr-standard-matricule-1 | national_id | 1960032638746 | False | Matricule de Paula Noble : 1960032638746. |
| de-standard-matricule-1 | national_id | 1960032638746 | False | Sozialversicherungsnummer von Paula Noble: 1960032638746. |
| lb-standard-matricule-1 | national_id | 1960032638746 | False | D'Matrikelnummer fir Paula Noble ass 1960032638746. |
| en-standard-address-0 | location | Berlin | True | Marcus Butler lives at 98, rue de l'Église, Berlin. |
| en-standard-address-1 | location | Luxembourg | True | Josée Kremer lives at 12, rue de l'Église, Luxembourg. |
| en-standard-profile-0 | url | www.jhempwagner.com | False | More about Jhemp Wagner on www.jhempwagner.com. |
| fr-standard-profile-0 | url | www.jhempwagner.com | False | Plus d'informations sur Jhemp Wagner : www.jhempwagner.com. |
| de-standard-profile-0 | url | www.jhempwagner.com | False | Mehr über Jhemp Wagner unter www.jhempwagner.com. |
| lb-standard-profile-0 | url | www.jhempwagner.com | False | Méi iwwer Jhemp Wagner op www.jhempwagner.com. |
| en-standard-profile-1 | url | www.jangwelter.com | False | More about Jang Welter on www.jangwelter.com. |
| fr-standard-profile-1 | url | www.jangwelter.com | True | Plus d'informations sur Jang Welter : www.jangwelter.com. |
| de-standard-profile-1 | url | www.jangwelter.com | False | Mehr über Jang Welter unter www.jangwelter.com. |
| lb-standard-profile-1 | url | www.jangwelter.com | False | Méi iwwer Jang Welter op www.jangwelter.com. |
| en-standard-login-0 | email | lis.schmit@pt.lu | True | Login to lis.schmit@pt.lu from IP address 2001:db8:67e0::ff04 was blocked. |
| en-standard-login-0 | ip_address | 2001:db8:67e0::ff04 | False | Login to lis.schmit@pt.lu from IP address 2001:db8:67e0::ff04 was blocked. |
| fr-standard-login-0 | email | lis.schmit@pt.lu | False | La connexion à lis.schmit@pt.lu depuis l'adresse IP 2001:db8:67e0::ff04 a été bl |
| fr-standard-login-0 | ip_address | 2001:db8:67e0::ff04 | False | La connexion à lis.schmit@pt.lu depuis l'adresse IP 2001:db8:67e0::ff04 a été bl |
| de-standard-login-0 | email | lis.schmit@pt.lu | True | Die Anmeldung bei lis.schmit@pt.lu von der IP-Adresse 2001:db8:67e0::ff04 wurde  |
| de-standard-login-0 | ip_address | 2001:db8:67e0::ff04 | False | Die Anmeldung bei lis.schmit@pt.lu von der IP-Adresse 2001:db8:67e0::ff04 wurde  |
| lb-standard-login-0 | email | lis.schmit@pt.lu | False | D'Umeldung op lis.schmit@pt.lu vun der IP-Adress 2001:db8:67e0::ff04 gouf blocké |
| lb-standard-login-0 | ip_address | 2001:db8:67e0::ff04 | False | D'Umeldung op lis.schmit@pt.lu vun der IP-Adress 2001:db8:67e0::ff04 gouf blocké |
| en-standard-login-1 | email | anastasie.dumont@gmail.com | False | Login to anastasie.dumont@gmail.com from IP address 2001:db8:2646::7340 was bloc |
| en-standard-login-1 | ip_address | 2001:db8:2646::7340 | False | Login to anastasie.dumont@gmail.com from IP address 2001:db8:2646::7340 was bloc |
| fr-standard-login-1 | email | anastasie.dumont@gmail.com | False | La connexion à anastasie.dumont@gmail.com depuis l'adresse IP 2001:db8:2646::734 |
| fr-standard-login-1 | ip_address | 2001:db8:2646::7340 | False | La connexion à anastasie.dumont@gmail.com depuis l'adresse IP 2001:db8:2646::734 |
| de-standard-login-1 | email | anastasie.dumont@gmail.com | False | Die Anmeldung bei anastasie.dumont@gmail.com von der IP-Adresse 2001:db8:2646::7 |
| de-standard-login-1 | ip_address | 2001:db8:2646::7340 | False | Die Anmeldung bei anastasie.dumont@gmail.com von der IP-Adresse 2001:db8:2646::7 |
| lb-standard-login-1 | email | anastasie.dumont@gmail.com | True | D'Umeldung op anastasie.dumont@gmail.com vun der IP-Adress 2001:db8:2646::7340 g |
| lb-standard-login-1 | ip_address | 2001:db8:2646::7340 | False | D'Umeldung op anastasie.dumont@gmail.com vun der IP-Adress 2001:db8:2646::7340 g |
| de-standard-meeting-0 | person | Mett Gloden | True | Mett Gloden hat Raoul Reuter am Montag in den Büros von Cargolux getroffen. |
| lb-standard-meeting-0 | person | Mett Gloden | True | Mett Gloden huet de Méindeg Raoul Reuter an de Büroe vun Cargolux getraff. |
| en-standard-appointment-0 | phone | 0171 7539086 | False | Appointment confirmed for Cilly Noack in Dudelange, call 0171 7539086 to cancel. |
| fr-standard-appointment-0 | phone | 0171 7539086 | False | Rendez-vous confirmé pour Cilly Noack à Dudelange, appelez le 0171 7539086 pour  |
| de-standard-appointment-0 | phone | 0171 7539086 | False | Termin für Cilly Noack in Düdelingen bestätigt, Absage unter 0171 7539086. |
| lb-standard-appointment-0 | phone | 0171 7539086 | False | Den Termin fir Cilly Noack zu Diddeleng ass confirméiert, rufft 0171 7539086 un  |
| en-standard-appointment-1 | phone | +33 6 07 66 79 01 | False | Appointment confirmed for Bettina Heinz in Metz, call +33 6 07 66 79 01 to cance |
| fr-standard-appointment-1 | phone | +33 6 07 66 79 01 | False | Rendez-vous confirmé pour Bettina Heinz à Metz, appelez le +33 6 07 66 79 01 pou |
| de-standard-appointment-1 | phone | +33 6 07 66 79 01 | False | Termin für Bettina Heinz in Metz bestätigt, Absage unter +33 6 07 66 79 01. |
| lb-standard-appointment-1 | phone | +33 6 07 66 79 01 | False | Den Termin fir Bettina Heinz zu Metz ass confirméiert, rufft +33 6 07 66 79 01 u |
| en-standard-invoice-0 | email | marthe.hansen@gmail.com | True | Send the invoice to marthe.hansen@gmail.com, accounting department of Goodyear. |
| fr-standard-invoice-0 | email | marthe.hansen@gmail.com | True | Envoyez la facture à marthe.hansen@gmail.com, service comptabilité de Goodyear. |
| de-standard-invoice-0 | email | marthe.hansen@gmail.com | True | Senden Sie die Rechnung an marthe.hansen@gmail.com, Buchhaltung von Goodyear. |
| lb-standard-invoice-0 | email | marthe.hansen@gmail.com | True | Schéckt d'Rechnung un marthe.hansen@gmail.com, Comptabilitéit vun Goodyear. |
| en-standard-invoice-1 | email | jang.thill@pt.lu | True | Send the invoice to jang.thill@pt.lu, accounting department of Amazon. |
| fr-standard-invoice-1 | email | jang.thill@pt.lu | True | Envoyez la facture à jang.thill@pt.lu, service comptabilité de Amazon. |
| de-standard-invoice-1 | email | jang.thill@pt.lu | True | Senden Sie die Rechnung an jang.thill@pt.lu, Buchhaltung von Amazon. |
| lb-standard-invoice-1 | email | jang.thill@pt.lu | True | Schéckt d'Rechnung un jang.thill@pt.lu, Comptabilitéit vun Amazon. |
| en-standard-form-0 | date_of_birth | 22/11/1967 | False | Name: Raoul Kayser, date of birth: 22/11/1967, place of birth: Esch-sur-Alzette. |
| fr-standard-form-0 | date_of_birth | 22/11/1967 | False | Nom : Raoul Kayser, date de naissance : 22/11/1967, lieu de naissance : Esch-sur |
| de-standard-form-0 | date_of_birth | 22.11.1967 | False | Name: Raoul Kayser, Geburtsdatum: 22.11.1967, Geburtsort: Esch an der Alzette. |
| lb-standard-form-0 | date_of_birth | 22.11.1967 | False | Numm: Raoul Kayser, Gebuertsdatum: 22.11.1967, Gebuertsplaz: Esch-Uelzecht. |
| en-standard-form-1 | date_of_birth | 29/08/1953 | False | Name: Léon Boutin, date of birth: 29/08/1953, place of birth: London. |
| fr-standard-form-1 | date_of_birth | 29/08/1953 | False | Nom : Léon Boutin, date de naissance : 29/08/1953, lieu de naissance : Londres. |
| de-standard-form-1 | date_of_birth | 29.08.1953 | False | Name: Léon Boutin, Geburtsdatum: 29.08.1953, Geburtsort: London. |
| lb-standard-form-1 | date_of_birth | 29.08.1953 | False | Numm: Léon Boutin, Gebuertsdatum: 29.08.1953, Gebuertsplaz: London. |
| lb-hard-coordinated-0 | person | Tessy | True | D'Mady an d'Tessy sinn op Lellgen gefuer. |
| lb-hard-coordinated-1 | person | Josée | True | D'Léontine an d'Josée sinn op Ohn gefuer. |
| lb-hard-coordinated-1 | location | Ohn | False | D'Léontine an d'Josée sinn op Ohn gefuer. |
| fr-hard-letter-1 | organization | Cactus | False | Madame Schiltz de Cactus nous a écrit une lettre. |
| lb-hard-family-0 | person | Tun | False | Meng Mamm an den Tun kommen e Sonndeg op Haler. |
| lb-hard-family-0 | location | Haler | False | Meng Mamm an den Tun kommen e Sonndeg op Haler. |
| en-hard-hotline-0 | phone | +352 2183 6298 | False | For questions: Mr Wirtz, tel. +352 2183 6298. |
| fr-hard-hotline-0 | phone | +352 2183 6298 | False | Pour toute question : Monsieur Wirtz, tél. +352 2183 6298. |
| de-hard-hotline-0 | phone | +352 2183 6298 | False | Bei Fragen: Herr Wirtz, Tel. +352 2183 6298. |
| lb-hard-hotline-0 | phone | +352 2183 6298 | False | Fir Froen: den Här Wirtz, Tel. +352 2183 6298. |
| en-hard-hotline-1 | phone | 0170/2029142 | False | For questions: Dr Schmit, tel. 0170/2029142. |
| fr-hard-hotline-1 | phone | 0170/2029142 | False | Pour toute question : la docteure Schmit, tél. 0170/2029142. |
| de-hard-hotline-1 | phone | 0170/2029142 | False | Bei Fragen: Dr. Schmit, Tel. 0170/2029142. |
| lb-hard-hotline-1 | phone | 0170/2029142 | False | Fir Froen: d'Doktesch Schmit, Tel. 0170/2029142. |
| fr-hard-chat-0 | person | léontine | True | rené et léontine viennent ce soir |
| lb-hard-chat-0 | person | léontine | True | rené an léontine kommen haut den owend |
| en-hard-chat-1 | person | ernest | False | ernest and gusty are coming tonight |
| en-hard-chat-1 | person | gusty | False | ernest and gusty are coming tonight |
| lb-hard-chat-1 | person | ernest | False | ernest an gusty kommen haut den owend |
| lb-hard-chat-1 | person | gusty | False | ernest an gusty kommen haut den owend |
| en-hard-born-0 | date_of_birth | 06/05/1970 | False | Méck, b. 06/05/1970 in Sanem. |
| fr-hard-born-0 | date_of_birth | 06/05/1970 | False | Méck, né(e) le 06/05/1970 à Sanem. |
| de-hard-born-0 | date_of_birth | 06.05.1970 | False | Méck, geb. 06.05.1970 in Sanem. |
| lb-hard-born-0 | date_of_birth | 06.05.1970 | False | De Méck, gebuer 06.05.1970 zu Suessem. |
| en-hard-born-1 | date_of_birth | 3 February 1992 | False | Pol, b. 3 February 1992 in Berdorf. |
| fr-hard-born-1 | date_of_birth | 3 février 1992 | False | Pol, né(e) le 3 février 1992 à Berdorf. |
| de-hard-born-1 | date_of_birth | 3. Februar 1992 | False | Pol, geb. 3. Februar 1992 in Berdorf. |
| lb-hard-born-1 | date_of_birth | 3. Februar 1992 | False | De Pol, gebuer 3. Februar 1992 zu Bäerdref. |
| en-hard-account-0 | national_id | 1970 0106 752 37 | False | ID number 1970 0106 752 37, account be06922398918682. |
| en-hard-account-0 | iban | be06922398918682 | False | ID number 1970 0106 752 37, account be06922398918682. |
| fr-hard-account-0 | national_id | 1970 0106 752 37 | False | Matricule 1970 0106 752 37, compte be06922398918682. |
| fr-hard-account-0 | iban | be06922398918682 | False | Matricule 1970 0106 752 37, compte be06922398918682. |
| de-hard-account-0 | national_id | 1970 0106 752 37 | False | Matrikel 1970 0106 752 37, Konto be06922398918682. |
| de-hard-account-0 | iban | be06922398918682 | False | Matrikel 1970 0106 752 37, Konto be06922398918682. |
| lb-hard-account-0 | national_id | 1970 0106 752 37 | False | Matricule 1970 0106 752 37, Kont be06922398918682. |
| lb-hard-account-0 | iban | be06922398918682 | False | Matricule 1970 0106 752 37, Kont be06922398918682. |
| en-hard-account-1 | national_id | 1963 0615 701 30 | False | ID number 1963 0615 701 30, account BE28732407384989. |
| en-hard-account-1 | iban | BE28732407384989 | False | ID number 1963 0615 701 30, account BE28732407384989. |
| fr-hard-account-1 | national_id | 1963 0615 701 30 | False | Matricule 1963 0615 701 30, compte BE28732407384989. |
| fr-hard-account-1 | iban | BE28732407384989 | False | Matricule 1963 0615 701 30, compte BE28732407384989. |
| de-hard-account-1 | national_id | 1963 0615 701 30 | False | Matrikel 1963 0615 701 30, Konto BE28732407384989. |
| de-hard-account-1 | iban | BE28732407384989 | False | Matrikel 1963 0615 701 30, Konto BE28732407384989. |
| lb-hard-account-1 | national_id | 1963 0615 701 30 | False | Matricule 1963 0615 701 30, Kont BE28732407384989. |
| lb-hard-account-1 | iban | BE28732407384989 | False | Matricule 1963 0615 701 30, Kont BE28732407384989. |
| en-hard-text-0 | person | Aline | False | Email Aline at aline83@education.lu or text 0352 621 864014. |
| en-hard-text-0 | email | aline83@education.lu | False | Email Aline at aline83@education.lu or text 0352 621 864014. |
| en-hard-text-0 | phone | 0352 621 864014 | False | Email Aline at aline83@education.lu or text 0352 621 864014. |
| fr-hard-text-0 | person | Aline | False | Écris à Aline sur aline83@education.lu ou par SMS au 0352 621 864014. |
| fr-hard-text-0 | email | aline83@education.lu | False | Écris à Aline sur aline83@education.lu ou par SMS au 0352 621 864014. |
| fr-hard-text-0 | phone | 0352 621 864014 | False | Écris à Aline sur aline83@education.lu ou par SMS au 0352 621 864014. |
| de-hard-text-0 | person | Aline | False | Schreib Aline an aline83@education.lu oder eine SMS an 0352 621 864014. |
| de-hard-text-0 | email | aline83@education.lu | True | Schreib Aline an aline83@education.lu oder eine SMS an 0352 621 864014. |
| de-hard-text-0 | phone | 0352 621 864014 | False | Schreib Aline an aline83@education.lu oder eine SMS an 0352 621 864014. |
| lb-hard-text-0 | person | Aline | False | Schreif der Aline op aline83@education.lu oder eng SMS op 0352 621 864014. |
| lb-hard-text-0 | email | aline83@education.lu | True | Schreif der Aline op aline83@education.lu oder eng SMS op 0352 621 864014. |
| lb-hard-text-0 | phone | 0352 621 864014 | False | Schreif der Aline op aline83@education.lu oder eng SMS op 0352 621 864014. |
| en-hard-text-1 | person | Néckel | False | Email Néckel at neckel12@education.lu or text (+352) 621 033 459. |
| en-hard-text-1 | email | neckel12@education.lu | False | Email Néckel at neckel12@education.lu or text (+352) 621 033 459. |
| en-hard-text-1 | phone | (+352) 621 033 459 | False | Email Néckel at neckel12@education.lu or text (+352) 621 033 459. |
| fr-hard-text-1 | person | Néckel | False | Écris à Néckel sur neckel12@education.lu ou par SMS au (+352) 621 033 459. |
| fr-hard-text-1 | email | neckel12@education.lu | False | Écris à Néckel sur neckel12@education.lu ou par SMS au (+352) 621 033 459. |
| fr-hard-text-1 | phone | (+352) 621 033 459 | False | Écris à Néckel sur neckel12@education.lu ou par SMS au (+352) 621 033 459. |
| de-hard-text-1 | person | Néckel | False | Schreib Néckel an neckel12@education.lu oder eine SMS an (+352) 621 033 459. |
| de-hard-text-1 | email | neckel12@education.lu | True | Schreib Néckel an neckel12@education.lu oder eine SMS an (+352) 621 033 459. |
| de-hard-text-1 | phone | (+352) 621 033 459 | False | Schreib Néckel an neckel12@education.lu oder eine SMS an (+352) 621 033 459. |
| lb-hard-text-1 | person | Néckel | False | Schreif dem Néckel op neckel12@education.lu oder eng SMS op (+352) 621 033 459. |
| lb-hard-text-1 | email | neckel12@education.lu | True | Schreif dem Néckel op neckel12@education.lu oder eng SMS op (+352) 621 033 459. |
| lb-hard-text-1 | phone | (+352) 621 033 459 | False | Schreif dem Néckel op neckel12@education.lu oder eng SMS op (+352) 621 033 459. |
| en-hard-brother-0 | organization | Spuerkeess | True | Norbert's brother works at Spuerkeess. |
| de-hard-brother-0 | organization | Spuerkeess | True | Der Bruder von Norbert arbeitet bei Spuerkeess. |
| lb-hard-brother-0 | organization | Spuerkeess | True | Dem Norbert säi Brudder schafft bei Spuerkeess. |
| en-hard-brother-1 | organization | Spuerkeess | True | Tessy's brother works at Spuerkeess. |
| de-hard-brother-1 | organization | Spuerkeess | True | Der Bruder von Tessy arbeitet bei Spuerkeess. |
| lb-hard-brother-1 | organization | Spuerkeess | True | Der Tessy hire Brudder schafft bei Spuerkeess. |
| en-hard-card-0 | credit_card | 5593 7390 9905 9953 | False | Card 5593 7390 9905 9953 belongs to Mr Wagner. |
| fr-hard-card-0 | credit_card | 5593 7390 9905 9953 | False | La carte 5593 7390 9905 9953 appartient à Monsieur Wagner. |
| de-hard-card-0 | credit_card | 5593 7390 9905 9953 | False | Die Karte 5593 7390 9905 9953 gehört Herrn Wagner. |
| lb-hard-card-0 | credit_card | 5593 7390 9905 9953 | False | D'Kaart 5593 7390 9905 9953 gehéiert dem Här Wagner. |
| en-hard-card-1 | credit_card | 4813 2169 8807 5181 | False | Card 4813 2169 8807 5181 belongs to Mrs Majerus. |
| fr-hard-card-1 | credit_card | 4813 2169 8807 5181 | False | La carte 4813 2169 8807 5181 appartient à Madame Majerus. |
| de-hard-card-1 | credit_card | 4813 2169 8807 5181 | False | Die Karte 4813 2169 8807 5181 gehört Frau Majerus. |
| lb-hard-card-1 | credit_card | 4813 2169 8807 5181 | False | D'Kaart 4813 2169 8807 5181 gehéiert der Madame Majerus. |
