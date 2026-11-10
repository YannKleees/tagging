# Evaluation: dev/ner_only

Config: `eval/configs/ner_only.json`. 248 sentences, 2297 words, 504 gold spans.

Recall comes first: a missed identifier leaks personal data, an over-redaction only costs readability.
`overlap` counts a prediction of the right type that overlaps the gold span, `exact` needs identical offsets, and `any_type_recall` counts a gold span as redacted if any prediction overlaps it.

Runtime: 5773.69 ms per 1,000 words (detection only).

## By language and subset

| lang | subset | recall_overlap | recall_exact | any_type_recall | precision_overlap | f1_overlap | f1_exact | n_gold | n_pred |
|---|---|---|---|---|---|---|---|---|---|
| de | hard | 0.70 | 0.48 | 0.73 | 0.93 | 0.80 | 0.55 | 60 | 45 |
| de | standard | 0.62 | 0.62 | 0.65 | 0.95 | 0.75 | 0.75 | 66 | 43 |
| en | hard | 0.68 | 0.47 | 0.70 | 0.93 | 0.79 | 0.54 | 60 | 44 |
| en | standard | 0.61 | 0.58 | 0.65 | 0.98 | 0.75 | 0.71 | 66 | 41 |
| fr | hard | 0.72 | 0.50 | 0.75 | 0.96 | 0.82 | 0.57 | 60 | 45 |
| fr | standard | 0.62 | 0.62 | 0.65 | 0.95 | 0.75 | 0.75 | 66 | 43 |
| lb | hard | 0.52 | 0.20 | 0.58 | 0.91 | 0.66 | 0.26 | 60 | 34 |
| lb | standard | 0.62 | 0.62 | 0.67 | 0.93 | 0.75 | 0.75 | 66 | 44 |

## Names only (person, organization, location, address)

| lang | subset | recall_overlap | recall_exact | precision_overlap | f1_overlap | n_gold | n_pred |
|---|---|---|---|---|---|---|---|
| de | hard | 0.91 | 0.63 | 0.93 | 0.92 | 46 | 45 |
| de | standard | 0.98 | 0.98 | 0.95 | 0.96 | 42 | 43 |
| en | hard | 0.89 | 0.61 | 0.93 | 0.91 | 46 | 44 |
| en | standard | 0.95 | 0.90 | 0.98 | 0.96 | 42 | 41 |
| fr | hard | 0.93 | 0.65 | 0.96 | 0.95 | 46 | 45 |
| fr | standard | 0.98 | 0.98 | 0.95 | 0.96 | 42 | 43 |
| lb | hard | 0.67 | 0.26 | 0.91 | 0.78 | 46 | 34 |
| lb | standard | 0.98 | 0.98 | 0.93 | 0.95 | 42 | 44 |

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
| address | 1.00 | 0.75 | 1.00 | 1.00 | 1.00 | 0.75 | 8 | 8 |
| credit_card | 0.00 | 0.00 | 0.00 | – | 0.00 | 0.00 | 16 | 0 |
| date_of_birth | 0.00 | 0.00 | 0.00 | – | 0.00 | 0.00 | 24 | 0 |
| email | 0.00 | 0.00 | 0.31 | – | 0.00 | 0.00 | 32 | 0 |
| iban | 0.00 | 0.00 | 0.00 | – | 0.00 | 0.00 | 16 | 0 |
| ip_address | 0.00 | 0.00 | 0.00 | – | 0.00 | 0.00 | 8 | 0 |
| location | 0.91 | 0.91 | 0.95 | 1.00 | 0.95 | 0.95 | 80 | 73 |
| national_id | 0.00 | 0.00 | 0.00 | – | 0.00 | 0.00 | 16 | 0 |
| organization | 0.98 | 0.96 | 0.98 | 0.94 | 0.96 | 0.94 | 48 | 50 |
| person | 0.89 | 0.62 | 0.92 | 0.92 | 0.91 | 0.64 | 216 | 208 |
| phone | 0.00 | 0.00 | 0.00 | – | 0.00 | 0.00 | 32 | 0 |
| url | 0.00 | 0.00 | 0.00 | – | 0.00 | 0.00 | 8 | 0 |

## By language and entity type (overlap)

| lang | type | recall_overlap | precision_overlap | f1_overlap | n_gold | n_pred |
|---|---|---|---|---|---|---|
| de | address | 1.00 | 1.00 | 1.00 | 2 | 2 |
| de | credit_card | 0.00 | – | 0.00 | 4 | 0 |
| de | date_of_birth | 0.00 | – | 0.00 | 6 | 0 |
| de | email | 0.00 | – | 0.00 | 8 | 0 |
| de | iban | 0.00 | – | 0.00 | 4 | 0 |
| de | ip_address | 0.00 | – | 0.00 | 2 | 0 |
| de | location | 1.00 | 1.00 | 1.00 | 20 | 20 |
| de | national_id | 0.00 | – | 0.00 | 4 | 0 |
| de | organization | 1.00 | 1.00 | 1.00 | 12 | 12 |
| de | person | 0.91 | 0.91 | 0.91 | 54 | 54 |
| de | phone | 0.00 | – | 0.00 | 8 | 0 |
| de | url | 0.00 | – | 0.00 | 2 | 0 |
| en | address | 1.00 | 1.00 | 1.00 | 2 | 2 |
| en | credit_card | 0.00 | – | 0.00 | 4 | 0 |
| en | date_of_birth | 0.00 | – | 0.00 | 6 | 0 |
| en | email | 0.00 | – | 0.00 | 8 | 0 |
| en | iban | 0.00 | – | 0.00 | 4 | 0 |
| en | ip_address | 0.00 | – | 0.00 | 2 | 0 |
| en | location | 0.90 | 1.00 | 0.95 | 20 | 18 |
| en | national_id | 0.00 | – | 0.00 | 4 | 0 |
| en | organization | 1.00 | 1.00 | 1.00 | 12 | 12 |
| en | person | 0.91 | 0.92 | 0.92 | 54 | 53 |
| en | phone | 0.00 | – | 0.00 | 8 | 0 |
| en | url | 0.00 | – | 0.00 | 2 | 0 |
| fr | address | 1.00 | 1.00 | 1.00 | 2 | 2 |
| fr | credit_card | 0.00 | – | 0.00 | 4 | 0 |
| fr | date_of_birth | 0.00 | – | 0.00 | 6 | 0 |
| fr | email | 0.00 | – | 0.00 | 8 | 0 |
| fr | iban | 0.00 | – | 0.00 | 4 | 0 |
| fr | ip_address | 0.00 | – | 0.00 | 2 | 0 |
| fr | location | 1.00 | 1.00 | 1.00 | 20 | 20 |
| fr | national_id | 0.00 | – | 0.00 | 4 | 0 |
| fr | organization | 1.00 | 1.00 | 1.00 | 12 | 12 |
| fr | person | 0.93 | 0.93 | 0.93 | 54 | 54 |
| fr | phone | 0.00 | – | 0.00 | 8 | 0 |
| fr | url | 0.00 | – | 0.00 | 2 | 0 |
| lb | address | 1.00 | 1.00 | 1.00 | 2 | 2 |
| lb | credit_card | 0.00 | – | 0.00 | 4 | 0 |
| lb | date_of_birth | 0.00 | – | 0.00 | 6 | 0 |
| lb | email | 0.00 | – | 0.00 | 8 | 0 |
| lb | iban | 0.00 | – | 0.00 | 4 | 0 |
| lb | ip_address | 0.00 | – | 0.00 | 2 | 0 |
| lb | location | 0.75 | 1.00 | 0.86 | 20 | 15 |
| lb | national_id | 0.00 | – | 0.00 | 4 | 0 |
| lb | organization | 0.92 | 0.79 | 0.85 | 12 | 14 |
| lb | person | 0.81 | 0.94 | 0.87 | 54 | 47 |
| lb | phone | 0.00 | – | 0.00 | 8 | 0 |
| lb | url | 0.00 | – | 0.00 | 2 | 0 |

## Gold names found in the gazetteer

Share of distinct gold person, location and organization strings that the built-in gazetteer contains. High values mean rules-only recall on names partly reflects list overlap with the generator.

| type | distinct | in_gazetteer | share |
|---|---|---|---|
| location | 42 | 42 | 1.00 |
| organization | 11 | 11 | 1.00 |
| person | 50 | 40 | 0.80 |

## Missed spans (184)

| id | type | text | covered | sentence |
|---|---|---|---|---|
| en-standard-contact-0 | email | tristan.fernandes@education.lu | False | Tristan Fernandes can be reached at tristan.fernandes@education.lu or on 0470 86 |
| en-standard-contact-0 | phone | 0470 86 57 97 | False | Tristan Fernandes can be reached at tristan.fernandes@education.lu or on 0470 86 |
| fr-standard-contact-0 | email | tristan.fernandes@education.lu | False | On peut joindre Tristan Fernandes à l'adresse tristan.fernandes@education.lu ou  |
| fr-standard-contact-0 | phone | 0470 86 57 97 | False | On peut joindre Tristan Fernandes à l'adresse tristan.fernandes@education.lu ou  |
| de-standard-contact-0 | email | tristan.fernandes@education.lu | False | Tristan Fernandes ist unter tristan.fernandes@education.lu oder 0470 86 57 97 er |
| de-standard-contact-0 | phone | 0470 86 57 97 | False | Tristan Fernandes ist unter tristan.fernandes@education.lu oder 0470 86 57 97 er |
| lb-standard-contact-0 | email | tristan.fernandes@education.lu | False | Tristan Fernandes ass ënner tristan.fernandes@education.lu oder um 0470 86 57 97 |
| lb-standard-contact-0 | phone | 0470 86 57 97 | False | Tristan Fernandes ass ënner tristan.fernandes@education.lu oder um 0470 86 57 97 |
| en-standard-contact-1 | email | tessy.hoffmann@education.lu | False | Tessy Hoffmann can be reached at tessy.hoffmann@education.lu or on 06-85995289. |
| en-standard-contact-1 | phone | 06-85995289 | False | Tessy Hoffmann can be reached at tessy.hoffmann@education.lu or on 06-85995289. |
| fr-standard-contact-1 | email | tessy.hoffmann@education.lu | False | On peut joindre Tessy Hoffmann à l'adresse tessy.hoffmann@education.lu ou au 06- |
| fr-standard-contact-1 | phone | 06-85995289 | False | On peut joindre Tessy Hoffmann à l'adresse tessy.hoffmann@education.lu ou au 06- |
| de-standard-contact-1 | email | tessy.hoffmann@education.lu | False | Tessy Hoffmann ist unter tessy.hoffmann@education.lu oder 06-85995289 erreichbar |
| de-standard-contact-1 | phone | 06-85995289 | False | Tessy Hoffmann ist unter tessy.hoffmann@education.lu oder 06-85995289 erreichbar |
| lb-standard-contact-1 | email | tessy.hoffmann@education.lu | False | Tessy Hoffmann ass ënner tessy.hoffmann@education.lu oder um 06-85995289 ze erre |
| lb-standard-contact-1 | phone | 06-85995289 | False | Tessy Hoffmann ass ënner tessy.hoffmann@education.lu oder um 06-85995289 ze erre |
| en-standard-birth-0 | date_of_birth | 03/09/1945 | False | Rico Siering was born on 03/09/1945 in Berlin. |
| fr-standard-birth-0 | date_of_birth | 03/09/1945 | False | Rico Siering (né(e) le 03/09/1945 à Berlin) a signé le contrat. |
| de-standard-birth-0 | date_of_birth | 03.09.1945 | False | Rico Siering wurde am 03.09.1945 in Berlin geboren. |
| lb-standard-birth-0 | date_of_birth | 03.09.1945 | False | Rico Siering ass den 03.09.1945 zu Berlin gebuer. |
| en-standard-birth-1 | date_of_birth | 02/08/1985 | False | Valentine Regnier was born on 02/08/1985 in Metz. |
| fr-standard-birth-1 | date_of_birth | 02/08/1985 | False | Valentine Regnier (né(e) le 02/08/1985 à Metz) a signé le contrat. |
| de-standard-birth-1 | date_of_birth | 02.08.1985 | False | Valentine Regnier wurde am 02.08.1985 in Metz geboren. |
| lb-standard-birth-1 | date_of_birth | 02.08.1985 | False | Valentine Regnier ass den 02.08.1985 zu Metz gebuer. |
| en-standard-transfer-0 | iban | FR63 1372 1590 1092 8159 0139 624 | False | Please transfer the deposit to Cora Bohlander, IBAN FR63 1372 1590 1092 8159 013 |
| fr-standard-transfer-0 | iban | FR63 1372 1590 1092 8159 0139 624 | False | Veuillez virer la caution à Cora Bohlander, IBAN FR63 1372 1590 1092 8159 0139 6 |
| de-standard-transfer-0 | iban | FR63 1372 1590 1092 8159 0139 624 | False | Bitte überweisen Sie die Kaution an Cora Bohlander, IBAN FR63 1372 1590 1092 815 |
| lb-standard-transfer-0 | iban | FR63 1372 1590 1092 8159 0139 624 | False | Iwwerweist d'Kautioun w.e.g. un Cora Bohlander, IBAN FR63 1372 1590 1092 8159 01 |
| en-standard-transfer-1 | iban | BE47 1177 7741 2154 | False | Please transfer the deposit to Corinne Benoit, IBAN BE47 1177 7741 2154. |
| fr-standard-transfer-1 | iban | BE47 1177 7741 2154 | False | Veuillez virer la caution à Corinne Benoit, IBAN BE47 1177 7741 2154. |
| de-standard-transfer-1 | iban | BE47 1177 7741 2154 | False | Bitte überweisen Sie die Kaution an Corinne Benoit, IBAN BE47 1177 7741 2154. |
| lb-standard-transfer-1 | iban | BE47 1177 7741 2154 | False | Iwwerweist d'Kautioun w.e.g. un Corinne Benoit, IBAN BE47 1177 7741 2154. |
| en-standard-card-0 | credit_card | 4131 8609 1390 9967 | False | Auguste Fernandez paid the booking with card 4131 8609 1390 9967. |
| fr-standard-card-0 | credit_card | 4131 8609 1390 9967 | False | Auguste Fernandez a payé la réservation avec la carte 4131 8609 1390 9967. |
| de-standard-card-0 | credit_card | 4131 8609 1390 9967 | False | Auguste Fernandez hat die Buchung mit der Karte 4131 8609 1390 9967 bezahlt. |
| lb-standard-card-0 | credit_card | 4131 8609 1390 9967 | False | Auguste Fernandez huet d'Reservatioun mat der Kaart 4131 8609 1390 9967 bezuelt. |
| en-standard-card-1 | credit_card | 3408 246281 94823 | False | Kätti Hoffmann paid the booking with card 3408 246281 94823. |
| fr-standard-card-1 | credit_card | 3408 246281 94823 | False | Kätti Hoffmann a payé la réservation avec la carte 3408 246281 94823. |
| de-standard-card-1 | credit_card | 3408 246281 94823 | False | Kätti Hoffmann hat die Buchung mit der Karte 3408 246281 94823 bezahlt. |
| lb-standard-card-1 | credit_card | 3408 246281 94823 | False | Kätti Hoffmann huet d'Reservatioun mat der Kaart 3408 246281 94823 bezuelt. |
| en-standard-matricule-0 | national_id | 1961052184125 | False | Social security number of Edward Salz: 1961052184125. |
| fr-standard-matricule-0 | national_id | 1961052184125 | False | Matricule de Edward Salz : 1961052184125. |
| de-standard-matricule-0 | national_id | 1961052184125 | False | Sozialversicherungsnummer von Edward Salz: 1961052184125. |
| lb-standard-matricule-0 | national_id | 1961052184125 | False | D'Matrikelnummer fir Edward Salz ass 1961052184125. |
| en-standard-matricule-1 | national_id | 1944051285228 | False | Social security number of Pero Dietz: 1944051285228. |
| fr-standard-matricule-1 | person | Pero Dietz | False | Matricule de Pero Dietz : 1944051285228. |
| fr-standard-matricule-1 | national_id | 1944051285228 | False | Matricule de Pero Dietz : 1944051285228. |
| de-standard-matricule-1 | national_id | 1944051285228 | False | Sozialversicherungsnummer von Pero Dietz: 1944051285228. |
| lb-standard-matricule-1 | national_id | 1944051285228 | False | D'Matrikelnummer fir Pero Dietz ass 1944051285228. |
| en-standard-address-0 | location | Dudelange | True | Jean Davies lives at 105, boulevard Royal, Dudelange. |
| en-standard-address-1 | location | Luxembourg | True | Rachel O'Donnell lives at 46, rue de la Gare, Luxembourg. |
| en-standard-profile-0 | url | https://www.linkedin.com/in/chantal-mary | False | More about Chantal Mary on https://www.linkedin.com/in/chantal-mary. |
| fr-standard-profile-0 | url | https://www.linkedin.com/in/chantal-mary | False | Plus d'informations sur Chantal Mary : https://www.linkedin.com/in/chantal-mary. |
| de-standard-profile-0 | url | https://www.linkedin.com/in/chantal-mary | False | Mehr über Chantal Mary unter https://www.linkedin.com/in/chantal-mary. |
| lb-standard-profile-0 | url | https://www.linkedin.com/in/chantal-mary | False | Méi iwwer Chantal Mary op https://www.linkedin.com/in/chantal-mary. |
| en-standard-profile-1 | url | https://stefania-metz.lu | False | More about Stefania Metz on https://stefania-metz.lu. |
| fr-standard-profile-1 | url | https://stefania-metz.lu | False | Plus d'informations sur Stefania Metz : https://stefania-metz.lu. |
| de-standard-profile-1 | url | https://stefania-metz.lu | False | Mehr über Stefania Metz unter https://stefania-metz.lu. |
| lb-standard-profile-1 | url | https://stefania-metz.lu | False | Méi iwwer Stefania Metz op https://stefania-metz.lu. |
| en-standard-login-0 | email | agnes.ribeiro@web.de | False | Login to agnes.ribeiro@web.de from IP address 245.250.94.21 was blocked. |
| en-standard-login-0 | ip_address | 245.250.94.21 | False | Login to agnes.ribeiro@web.de from IP address 245.250.94.21 was blocked. |
| fr-standard-login-0 | email | agnes.ribeiro@web.de | True | La connexion à agnes.ribeiro@web.de depuis l'adresse IP 245.250.94.21 a été bloq |
| fr-standard-login-0 | ip_address | 245.250.94.21 | False | La connexion à agnes.ribeiro@web.de depuis l'adresse IP 245.250.94.21 a été bloq |
| de-standard-login-0 | email | agnes.ribeiro@web.de | False | Die Anmeldung bei agnes.ribeiro@web.de von der IP-Adresse 245.250.94.21 wurde bl |
| de-standard-login-0 | ip_address | 245.250.94.21 | False | Die Anmeldung bei agnes.ribeiro@web.de von der IP-Adresse 245.250.94.21 wurde bl |
| lb-standard-login-0 | email | agnes.ribeiro@web.de | True | D'Umeldung op agnes.ribeiro@web.de vun der IP-Adress 245.250.94.21 gouf blockéie |
| lb-standard-login-0 | ip_address | 245.250.94.21 | False | D'Umeldung op agnes.ribeiro@web.de vun der IP-Adress 245.250.94.21 gouf blockéie |
| en-standard-login-1 | email | fernande.majerus@gmail.com | False | Login to fernande.majerus@gmail.com from IP address 2001:db8:9fc3::fc3a was bloc |
| en-standard-login-1 | ip_address | 2001:db8:9fc3::fc3a | False | Login to fernande.majerus@gmail.com from IP address 2001:db8:9fc3::fc3a was bloc |
| fr-standard-login-1 | email | fernande.majerus@gmail.com | False | La connexion à fernande.majerus@gmail.com depuis l'adresse IP 2001:db8:9fc3::fc3 |
| fr-standard-login-1 | ip_address | 2001:db8:9fc3::fc3a | False | La connexion à fernande.majerus@gmail.com depuis l'adresse IP 2001:db8:9fc3::fc3 |
| de-standard-login-1 | email | fernande.majerus@gmail.com | False | Die Anmeldung bei fernande.majerus@gmail.com von der IP-Adresse 2001:db8:9fc3::f |
| de-standard-login-1 | ip_address | 2001:db8:9fc3::fc3a | False | Die Anmeldung bei fernande.majerus@gmail.com von der IP-Adresse 2001:db8:9fc3::f |
| lb-standard-login-1 | email | fernande.majerus@gmail.com | False | D'Umeldung op fernande.majerus@gmail.com vun der IP-Adress 2001:db8:9fc3::fc3a g |
| lb-standard-login-1 | ip_address | 2001:db8:9fc3::fc3a | False | D'Umeldung op fernande.majerus@gmail.com vun der IP-Adress 2001:db8:9fc3::fc3a g |
| de-standard-meeting-1 | person | Ingo Butte | False | Tun Ries hat Ingo Butte am Montag in den Büros von KPMG getroffen. |
| en-standard-appointment-0 | phone | 06 63 30 43 48 | False | Appointment confirmed for Beverley Hodgson in Paris, call 06 63 30 43 48 to canc |
| fr-standard-appointment-0 | phone | 06 63 30 43 48 | False | Rendez-vous confirmé pour Beverley Hodgson à Paris, appelez le 06 63 30 43 48 po |
| de-standard-appointment-0 | phone | 06 63 30 43 48 | False | Termin für Beverley Hodgson in Paris bestätigt, Absage unter 06 63 30 43 48. |
| lb-standard-appointment-0 | phone | 06 63 30 43 48 | False | Den Termin fir Beverley Hodgson zu Paräis ass confirméiert, rufft 06 63 30 43 48 |
| en-standard-appointment-1 | phone | 06 67 51 36 13 | False | Appointment confirmed for Olivia Gardiner in Berlin, call 06 67 51 36 13 to canc |
| fr-standard-appointment-1 | phone | 06 67 51 36 13 | False | Rendez-vous confirmé pour Olivia Gardiner à Berlin, appelez le 06 67 51 36 13 po |
| de-standard-appointment-1 | phone | 06 67 51 36 13 | False | Termin für Olivia Gardiner in Berlin bestätigt, Absage unter 06 67 51 36 13. |
| lb-standard-appointment-1 | phone | 06 67 51 36 13 | False | Den Termin fir Olivia Gardiner zu Berlin ass confirméiert, rufft 06 67 51 36 13  |
| en-standard-invoice-0 | email | gilbert.kremer@example.com | False | Send the invoice to gilbert.kremer@example.com, accounting department of BGL BNP |
| fr-standard-invoice-0 | email | gilbert.kremer@example.com | False | Envoyez la facture à gilbert.kremer@example.com, service comptabilité de BGL BNP |
| de-standard-invoice-0 | email | gilbert.kremer@example.com | True | Senden Sie die Rechnung an gilbert.kremer@example.com, Buchhaltung von BGL BNP P |
| lb-standard-invoice-0 | email | gilbert.kremer@example.com | True | Schéckt d'Rechnung un gilbert.kremer@example.com, Comptabilitéit vun BGL BNP Par |
| en-standard-invoice-1 | email | nathalie.pons@web.de | True | Send the invoice to nathalie.pons@web.de, accounting department of Google. |
| fr-standard-invoice-1 | email | nathalie.pons@web.de | True | Envoyez la facture à nathalie.pons@web.de, service comptabilité de Google. |
| de-standard-invoice-1 | email | nathalie.pons@web.de | True | Senden Sie die Rechnung an nathalie.pons@web.de, Buchhaltung von Google. |
| lb-standard-invoice-1 | email | nathalie.pons@web.de | True | Schéckt d'Rechnung un nathalie.pons@web.de, Comptabilitéit vun Google. |
| lb-standard-invoice-1 | organization | Google | False | Schéckt d'Rechnung un nathalie.pons@web.de, Comptabilitéit vun Google. |
| en-standard-form-0 | date_of_birth | 11/10/1945 | False | Name: Léontine Goedert, date of birth: 11/10/1945, place of birth: Brussels. |
| fr-standard-form-0 | date_of_birth | 11/10/1945 | False | Nom : Léontine Goedert, date de naissance : 11/10/1945, lieu de naissance : Brux |
| de-standard-form-0 | date_of_birth | 11.10.1945 | False | Name: Léontine Goedert, Geburtsdatum: 11.10.1945, Geburtsort: Brüssel. |
| lb-standard-form-0 | date_of_birth | 11.10.1945 | False | Numm: Léontine Goedert, Gebuertsdatum: 11.10.1945, Gebuertsplaz: Bréissel. |
| en-standard-form-1 | date_of_birth | 15/05/1982 | False | Name: Terence Lewis, date of birth: 15/05/1982, place of birth: Esch-sur-Alzette |
| fr-standard-form-1 | date_of_birth | 15/05/1982 | False | Nom : Terence Lewis, date de naissance : 15/05/1982, lieu de naissance : Esch-su |
| de-standard-form-1 | date_of_birth | 15.05.1982 | False | Name: Terence Lewis, Geburtsdatum: 15.05.1982, Geburtsort: Esch an der Alzette. |
| lb-standard-form-1 | date_of_birth | 15.05.1982 | False | Numm: Terence Lewis, Gebuertsdatum: 15.05.1982, Gebuertsplaz: Esch-Uelzecht. |
| en-hard-coordinated-0 | person | Charel | True | Lis and Charel drove to Betzdorf. |
| fr-hard-coordinated-0 | person | Charel | True | Lis et Charel sont partis à Betzdorf. |
| de-hard-coordinated-0 | person | Charel | True | Lis und Charel sind nach Betzdorf gefahren. |
| lb-hard-coordinated-0 | person | Charel | True | D'Lis an de Charel sinn op Betzder gefuer. |
| lb-hard-coordinated-1 | person | Léontine | True | De Méck an d'Léontine sinn op Buerschent gefuer. |
| lb-hard-letter-1 | person | Feltgen | False | Den Här Feltgen vun CNS huet eis e Bréif geschriwwen. |
| lb-hard-family-0 | location | Diänjen | False | Meng Mamm an de Jang kommen e Sonndeg op Diänjen. |
| lb-hard-family-1 | location | Géisdref | False | Meng Mamm an d'Romaine kommen e Sonndeg op Géisdref. |
| lb-hard-workplace-0 | person | Néckel | False | Den Néckel schafft zënter Joren bei Brasserie Schuman zu Luerenzweiler. |
| lb-hard-workplace-0 | location | Luerenzweiler | False | Den Néckel schafft zënter Joren bei Brasserie Schuman zu Luerenzweiler. |
| lb-hard-workplace-1 | location | Welschent | True | De Charel schafft zënter Joren bei ADEM zu Welschent. |
| en-hard-hotline-0 | phone | +49 (0)30 70455853 | False | For questions: Dr Schmit, tel. +49 (0)30 70455853. |
| fr-hard-hotline-0 | phone | +49 (0)30 70455853 | False | Pour toute question : le docteur Schmit, tél. +49 (0)30 70455853. |
| de-hard-hotline-0 | phone | +49 (0)30 70455853 | False | Bei Fragen: Dr. Schmit, Tel. +49 (0)30 70455853. |
| lb-hard-hotline-0 | phone | +49 (0)30 70455853 | False | Fir Froen: den Dokter Schmit, Tel. +49 (0)30 70455853. |
| en-hard-hotline-1 | phone | (+352) 621 920 868 | False | For questions: Dr Kremer, tel. (+352) 621 920 868. |
| fr-hard-hotline-1 | phone | (+352) 621 920 868 | False | Pour toute question : le docteur Kremer, tél. (+352) 621 920 868. |
| de-hard-hotline-1 | phone | (+352) 621 920 868 | False | Bei Fragen: Dr. Kremer, Tel. (+352) 621 920 868. |
| lb-hard-hotline-1 | phone | (+352) 621 920 868 | False | Fir Froen: den Dokter Kremer, Tel. (+352) 621 920 868. |
| en-hard-chat-1 | person | jang | False | jang and gusty are coming tonight |
| en-hard-chat-1 | person | gusty | False | jang and gusty are coming tonight |
| de-hard-chat-1 | person | jang | False | jang und gusty kommen heute abend |
| lb-hard-chat-1 | person | jang | False | jang an gusty kommen haut den owend |
| lb-hard-chat-1 | person | gusty | False | jang an gusty kommen haut den owend |
| en-hard-born-0 | date_of_birth | 08/02/1982 | False | Fernande, b. 08/02/1982 in Finsterthal. |
| fr-hard-born-0 | date_of_birth | 08/02/1982 | False | Fernande, né(e) le 08/02/1982 à Finsterthal. |
| de-hard-born-0 | date_of_birth | 08.02.1982 | False | Fernande, geb. 08.02.1982 in Finsterthal. |
| lb-hard-born-0 | date_of_birth | 08.02.1982 | False | D'Fernande, gebuer 08.02.1982 zu Fënsterdall. |
| en-hard-born-1 | date_of_birth | 23/06/1983 | False | Pol, b. 23/06/1983 in Reisdorf. |
| fr-hard-born-1 | date_of_birth | 23/06/1983 | False | Pol, né(e) le 23/06/1983 à Reisdorf. |
| de-hard-born-1 | date_of_birth | 23.06.1983 | False | Pol, geb. 23.06.1983 in Reisdorf. |
| lb-hard-born-1 | date_of_birth | 23.06.1983 | False | De Pol, gebuer 23.06.1983 zu Reisduerf. |
| en-hard-account-0 | national_id | 1997 1127 617 00 | False | ID number 1997 1127 617 00, account DE78093192544992070741. |
| en-hard-account-0 | iban | DE78093192544992070741 | False | ID number 1997 1127 617 00, account DE78093192544992070741. |
| fr-hard-account-0 | national_id | 1997 1127 617 00 | False | Matricule 1997 1127 617 00, compte DE78093192544992070741. |
| fr-hard-account-0 | iban | DE78093192544992070741 | False | Matricule 1997 1127 617 00, compte DE78093192544992070741. |
| de-hard-account-0 | national_id | 1997 1127 617 00 | False | Matrikel 1997 1127 617 00, Konto DE78093192544992070741. |
| de-hard-account-0 | iban | DE78093192544992070741 | False | Matrikel 1997 1127 617 00, Konto DE78093192544992070741. |
| lb-hard-account-0 | national_id | 1997 1127 617 00 | False | Matricule 1997 1127 617 00, Kont DE78093192544992070741. |
| lb-hard-account-0 | iban | DE78093192544992070741 | False | Matricule 1997 1127 617 00, Kont DE78093192544992070741. |
| en-hard-account-1 | national_id | 2000 0504 847 18 | False | ID number 2000 0504 847 18, account BE07718341704718. |
| en-hard-account-1 | iban | BE07718341704718 | False | ID number 2000 0504 847 18, account BE07718341704718. |
| fr-hard-account-1 | national_id | 2000 0504 847 18 | False | Matricule 2000 0504 847 18, compte BE07718341704718. |
| fr-hard-account-1 | iban | BE07718341704718 | False | Matricule 2000 0504 847 18, compte BE07718341704718. |
| de-hard-account-1 | national_id | 2000 0504 847 18 | False | Matrikel 2000 0504 847 18, Konto BE07718341704718. |
| de-hard-account-1 | iban | BE07718341704718 | False | Matrikel 2000 0504 847 18, Konto BE07718341704718. |
| lb-hard-account-1 | national_id | 2000 0504 847 18 | False | Matricule 2000 0504 847 18, Kont BE07718341704718. |
| lb-hard-account-1 | iban | BE07718341704718 | False | Matricule 2000 0504 847 18, Kont BE07718341704718. |
| lb-hard-separated-0 | person | Ries | True | D'Doktesch Ries an den Här Weber hu sech getrennt. |
| lb-hard-grandma-0 | location | Léiler | False | De Jhemp wunnt elo bei der Bomi zu Léiler. |
| en-hard-text-0 | person | Norbert | False | Email Norbert at norbert58@gmail.com or text 0170/2470822. |
| en-hard-text-0 | email | norbert58@gmail.com | False | Email Norbert at norbert58@gmail.com or text 0170/2470822. |
| en-hard-text-0 | phone | 0170/2470822 | False | Email Norbert at norbert58@gmail.com or text 0170/2470822. |
| fr-hard-text-0 | person | Norbert | False | Écris à Norbert sur norbert58@gmail.com ou par SMS au 0170/2470822. |
| fr-hard-text-0 | email | norbert58@gmail.com | True | Écris à Norbert sur norbert58@gmail.com ou par SMS au 0170/2470822. |
| fr-hard-text-0 | phone | 0170/2470822 | False | Écris à Norbert sur norbert58@gmail.com ou par SMS au 0170/2470822. |
| de-hard-text-0 | person | Norbert | False | Schreib Norbert an norbert58@gmail.com oder eine SMS an 0170/2470822. |
| de-hard-text-0 | email | norbert58@gmail.com | True | Schreib Norbert an norbert58@gmail.com oder eine SMS an 0170/2470822. |
| de-hard-text-0 | phone | 0170/2470822 | False | Schreib Norbert an norbert58@gmail.com oder eine SMS an 0170/2470822. |
| lb-hard-text-0 | person | Norbert | False | Schreif dem Norbert op norbert58@gmail.com oder eng SMS op 0170/2470822. |
| lb-hard-text-0 | email | norbert58@gmail.com | False | Schreif dem Norbert op norbert58@gmail.com oder eng SMS op 0170/2470822. |
| lb-hard-text-0 | phone | 0170/2470822 | False | Schreif dem Norbert op norbert58@gmail.com oder eng SMS op 0170/2470822. |
| en-hard-text-1 | person | Kätti | False | Email Kätti at katti48@gmail.com or text +352 2576 2833. |
| en-hard-text-1 | email | katti48@gmail.com | False | Email Kätti at katti48@gmail.com or text +352 2576 2833. |
| en-hard-text-1 | phone | +352 2576 2833 | False | Email Kätti at katti48@gmail.com or text +352 2576 2833. |
| fr-hard-text-1 | person | Kätti | False | Écris à Kätti sur katti48@gmail.com ou par SMS au +352 2576 2833. |
| fr-hard-text-1 | email | katti48@gmail.com | False | Écris à Kätti sur katti48@gmail.com ou par SMS au +352 2576 2833. |
| fr-hard-text-1 | phone | +352 2576 2833 | False | Écris à Kätti sur katti48@gmail.com ou par SMS au +352 2576 2833. |
| de-hard-text-1 | person | Kätti | False | Schreib Kätti an katti48@gmail.com oder eine SMS an +352 2576 2833. |
| de-hard-text-1 | email | katti48@gmail.com | False | Schreib Kätti an katti48@gmail.com oder eine SMS an +352 2576 2833. |
| de-hard-text-1 | phone | +352 2576 2833 | False | Schreib Kätti an katti48@gmail.com oder eine SMS an +352 2576 2833. |
| lb-hard-text-1 | person | Kätti | False | Schreif der Kätti op katti48@gmail.com oder eng SMS op +352 2576 2833. |
| lb-hard-text-1 | email | katti48@gmail.com | False | Schreif der Kätti op katti48@gmail.com oder eng SMS op +352 2576 2833. |
| lb-hard-text-1 | phone | +352 2576 2833 | False | Schreif der Kätti op katti48@gmail.com oder eng SMS op +352 2576 2833. |
| lb-hard-brother-0 | person | Charel | False | Dem Charel säi Brudder schafft bei CHL. |
| en-hard-card-0 | credit_card | 5249 7465 0752 9178 | False | Card 5249 7465 0752 9178 belongs to Dr Hansen. |
| fr-hard-card-0 | credit_card | 5249 7465 0752 9178 | False | La carte 5249 7465 0752 9178 appartient à la docteure Hansen. |
| de-hard-card-0 | credit_card | 5249 7465 0752 9178 | False | Die Karte 5249 7465 0752 9178 gehört Dr. Hansen. |
| lb-hard-card-0 | credit_card | 5249 7465 0752 9178 | False | D'Kaart 5249 7465 0752 9178 gehéiert der Doktesch Hansen. |
| en-hard-card-1 | credit_card | 3436 671276 84266 | False | Card 3436 671276 84266 belongs to Mrs Weber. |
| fr-hard-card-1 | credit_card | 3436 671276 84266 | False | La carte 3436 671276 84266 appartient à Madame Weber. |
| de-hard-card-1 | credit_card | 3436 671276 84266 | False | Die Karte 3436 671276 84266 gehört Frau Weber. |
| lb-hard-card-1 | credit_card | 3436 671276 84266 | False | D'Kaart 3436 671276 84266 gehéiert der Madame Weber. |
