# Evaluation: dev/rules_threshold_060

Config: `eval/configs/rules_threshold_060.json`. 248 sentences, 2297 words, 504 gold spans.

Recall comes first: a missed identifier leaks personal data, an over-redaction only costs readability.
`overlap` counts a prediction of the right type that overlaps the gold span, `exact` needs identical offsets, and `any_type_recall` counts a gold span as redacted if any prediction overlaps it.

Runtime: 18.96 ms per 1,000 words (detection only).

## By language and subset

| lang | subset | recall_overlap | recall_exact | any_type_recall | precision_overlap | f1_overlap | f1_exact | n_gold | n_pred |
|---|---|---|---|---|---|---|---|---|---|
| de | hard | 0.62 | 0.62 | 0.62 | 1.00 | 0.76 | 0.76 | 60 | 37 |
| de | standard | 0.50 | 0.44 | 0.50 | 1.00 | 0.67 | 0.59 | 66 | 33 |
| en | hard | 0.58 | 0.58 | 0.58 | 1.00 | 0.74 | 0.74 | 60 | 35 |
| en | standard | 0.50 | 0.44 | 0.50 | 1.00 | 0.67 | 0.59 | 66 | 33 |
| fr | hard | 0.62 | 0.62 | 0.62 | 1.00 | 0.76 | 0.76 | 60 | 37 |
| fr | standard | 0.50 | 0.44 | 0.50 | 1.00 | 0.67 | 0.59 | 66 | 33 |
| lb | hard | 0.62 | 0.62 | 0.62 | 1.00 | 0.76 | 0.76 | 60 | 37 |
| lb | standard | 0.50 | 0.44 | 0.50 | 1.00 | 0.67 | 0.59 | 66 | 33 |

## Names only (person, organization, location, address)

| lang | subset | recall_overlap | recall_exact | precision_overlap | f1_overlap | n_gold | n_pred |
|---|---|---|---|---|---|---|---|
| de | hard | 0.50 | 0.50 | 1.00 | 0.67 | 46 | 23 |
| de | standard | 0.21 | 0.12 | 1.00 | 0.35 | 42 | 9 |
| en | hard | 0.50 | 0.50 | 1.00 | 0.67 | 46 | 23 |
| en | standard | 0.21 | 0.12 | 1.00 | 0.35 | 42 | 9 |
| fr | hard | 0.50 | 0.50 | 1.00 | 0.67 | 46 | 23 |
| fr | standard | 0.21 | 0.12 | 1.00 | 0.35 | 42 | 9 |
| lb | hard | 0.50 | 0.50 | 1.00 | 0.67 | 46 | 23 |
| lb | standard | 0.21 | 0.12 | 1.00 | 0.35 | 42 | 9 |

## False positives on the entity-free subset

| lang | false positives | sentences |
|---|---|---|
| en | 1 | 10 |
| fr | 1 | 10 |
| de | 1 | 10 |
| lb | 1 | 10 |

| id | type | text | score |
|---|---|---|---|
| en-entity_free-08 | ip_address | 2.3.0.1 | 0.90 |
| fr-entity_free-08 | ip_address | 2.3.0.1 | 0.90 |
| de-entity_free-08 | ip_address | 2.3.0.1 | 0.90 |
| lb-entity_free-08 | ip_address | 2.3.0.1 | 0.90 |

## By entity type

| type | recall_overlap | recall_exact | any_type_recall | precision_overlap | f1_overlap | f1_exact | n_gold | n_pred |
|---|---|---|---|---|---|---|---|---|
| address | 0.00 | 0.00 | 0.00 | – | 0.00 | 0.00 | 8 | 0 |
| credit_card | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 16 | 16 |
| date_of_birth | 0.92 | 0.92 | 0.92 | 1.00 | 0.96 | 0.96 | 24 | 22 |
| email | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 32 | 32 |
| iban | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 16 | 16 |
| ip_address | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 8 | 8 |
| location | 0.65 | 0.65 | 0.65 | 1.00 | 0.79 | 0.79 | 80 | 52 |
| national_id | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 16 | 16 |
| organization | 0.50 | 0.50 | 0.50 | 1.00 | 0.67 | 0.67 | 48 | 24 |
| person | 0.24 | 0.17 | 0.24 | 1.00 | 0.39 | 0.27 | 216 | 52 |
| phone | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 32 | 32 |
| url | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 8 | 8 |

## By language and entity type (overlap)

| lang | type | recall_overlap | precision_overlap | f1_overlap | n_gold | n_pred |
|---|---|---|---|---|---|---|
| de | address | 0.00 | – | 0.00 | 2 | 0 |
| de | credit_card | 1.00 | 1.00 | 1.00 | 4 | 4 |
| de | date_of_birth | 1.00 | 1.00 | 1.00 | 6 | 6 |
| de | email | 1.00 | 1.00 | 1.00 | 8 | 8 |
| de | iban | 1.00 | 1.00 | 1.00 | 4 | 4 |
| de | ip_address | 1.00 | 1.00 | 1.00 | 2 | 2 |
| de | location | 0.65 | 1.00 | 0.79 | 20 | 13 |
| de | national_id | 1.00 | 1.00 | 1.00 | 4 | 4 |
| de | organization | 0.50 | 1.00 | 0.67 | 12 | 6 |
| de | person | 0.24 | 1.00 | 0.39 | 54 | 13 |
| de | phone | 1.00 | 1.00 | 1.00 | 8 | 8 |
| de | url | 1.00 | 1.00 | 1.00 | 2 | 2 |
| en | address | 0.00 | – | 0.00 | 2 | 0 |
| en | credit_card | 1.00 | 1.00 | 1.00 | 4 | 4 |
| en | date_of_birth | 0.67 | 1.00 | 0.80 | 6 | 4 |
| en | email | 1.00 | 1.00 | 1.00 | 8 | 8 |
| en | iban | 1.00 | 1.00 | 1.00 | 4 | 4 |
| en | ip_address | 1.00 | 1.00 | 1.00 | 2 | 2 |
| en | location | 0.65 | 1.00 | 0.79 | 20 | 13 |
| en | national_id | 1.00 | 1.00 | 1.00 | 4 | 4 |
| en | organization | 0.50 | 1.00 | 0.67 | 12 | 6 |
| en | person | 0.24 | 1.00 | 0.39 | 54 | 13 |
| en | phone | 1.00 | 1.00 | 1.00 | 8 | 8 |
| en | url | 1.00 | 1.00 | 1.00 | 2 | 2 |
| fr | address | 0.00 | – | 0.00 | 2 | 0 |
| fr | credit_card | 1.00 | 1.00 | 1.00 | 4 | 4 |
| fr | date_of_birth | 1.00 | 1.00 | 1.00 | 6 | 6 |
| fr | email | 1.00 | 1.00 | 1.00 | 8 | 8 |
| fr | iban | 1.00 | 1.00 | 1.00 | 4 | 4 |
| fr | ip_address | 1.00 | 1.00 | 1.00 | 2 | 2 |
| fr | location | 0.65 | 1.00 | 0.79 | 20 | 13 |
| fr | national_id | 1.00 | 1.00 | 1.00 | 4 | 4 |
| fr | organization | 0.50 | 1.00 | 0.67 | 12 | 6 |
| fr | person | 0.24 | 1.00 | 0.39 | 54 | 13 |
| fr | phone | 1.00 | 1.00 | 1.00 | 8 | 8 |
| fr | url | 1.00 | 1.00 | 1.00 | 2 | 2 |
| lb | address | 0.00 | – | 0.00 | 2 | 0 |
| lb | credit_card | 1.00 | 1.00 | 1.00 | 4 | 4 |
| lb | date_of_birth | 1.00 | 1.00 | 1.00 | 6 | 6 |
| lb | email | 1.00 | 1.00 | 1.00 | 8 | 8 |
| lb | iban | 1.00 | 1.00 | 1.00 | 4 | 4 |
| lb | ip_address | 1.00 | 1.00 | 1.00 | 2 | 2 |
| lb | location | 0.65 | 1.00 | 0.79 | 20 | 13 |
| lb | national_id | 1.00 | 1.00 | 1.00 | 4 | 4 |
| lb | organization | 0.50 | 1.00 | 0.67 | 12 | 6 |
| lb | person | 0.24 | 1.00 | 0.39 | 54 | 13 |
| lb | phone | 1.00 | 1.00 | 1.00 | 8 | 8 |
| lb | url | 1.00 | 1.00 | 1.00 | 2 | 2 |

## Gold names found in the gazetteer

Share of distinct gold person, location and organization strings that the built-in gazetteer contains. High values mean rules-only recall on names partly reflects list overlap with the generator.

| type | distinct | in_gazetteer | share |
|---|---|---|---|
| location | 42 | 32 | 0.76 |
| organization | 11 | 6 | 0.55 |
| person | 50 | 17 | 0.34 |

## Missed spans (226)

| id | type | text | covered | sentence |
|---|---|---|---|---|
| en-standard-contact-0 | person | Tristan Fernandes | False | Tristan Fernandes can be reached at tristan.fernandes@education.lu or on 0470 86 |
| fr-standard-contact-0 | person | Tristan Fernandes | False | On peut joindre Tristan Fernandes à l'adresse tristan.fernandes@education.lu ou  |
| de-standard-contact-0 | person | Tristan Fernandes | False | Tristan Fernandes ist unter tristan.fernandes@education.lu oder 0470 86 57 97 er |
| lb-standard-contact-0 | person | Tristan Fernandes | False | Tristan Fernandes ass ënner tristan.fernandes@education.lu oder um 0470 86 57 97 |
| en-standard-birth-0 | person | Rico Siering | False | Rico Siering was born on 03/09/1945 in Berlin. |
| en-standard-birth-0 | location | Berlin | False | Rico Siering was born on 03/09/1945 in Berlin. |
| fr-standard-birth-0 | person | Rico Siering | False | Rico Siering (né(e) le 03/09/1945 à Berlin) a signé le contrat. |
| fr-standard-birth-0 | location | Berlin | False | Rico Siering (né(e) le 03/09/1945 à Berlin) a signé le contrat. |
| de-standard-birth-0 | person | Rico Siering | False | Rico Siering wurde am 03.09.1945 in Berlin geboren. |
| de-standard-birth-0 | location | Berlin | False | Rico Siering wurde am 03.09.1945 in Berlin geboren. |
| lb-standard-birth-0 | person | Rico Siering | False | Rico Siering ass den 03.09.1945 zu Berlin gebuer. |
| lb-standard-birth-0 | location | Berlin | False | Rico Siering ass den 03.09.1945 zu Berlin gebuer. |
| en-standard-birth-1 | person | Valentine Regnier | False | Valentine Regnier was born on 02/08/1985 in Metz. |
| en-standard-birth-1 | location | Metz | False | Valentine Regnier was born on 02/08/1985 in Metz. |
| fr-standard-birth-1 | person | Valentine Regnier | False | Valentine Regnier (né(e) le 02/08/1985 à Metz) a signé le contrat. |
| fr-standard-birth-1 | location | Metz | False | Valentine Regnier (né(e) le 02/08/1985 à Metz) a signé le contrat. |
| de-standard-birth-1 | person | Valentine Regnier | False | Valentine Regnier wurde am 02.08.1985 in Metz geboren. |
| de-standard-birth-1 | location | Metz | False | Valentine Regnier wurde am 02.08.1985 in Metz geboren. |
| lb-standard-birth-1 | person | Valentine Regnier | False | Valentine Regnier ass den 02.08.1985 zu Metz gebuer. |
| lb-standard-birth-1 | location | Metz | False | Valentine Regnier ass den 02.08.1985 zu Metz gebuer. |
| en-standard-transfer-0 | person | Cora Bohlander | False | Please transfer the deposit to Cora Bohlander, IBAN FR63 1372 1590 1092 8159 013 |
| fr-standard-transfer-0 | person | Cora Bohlander | False | Veuillez virer la caution à Cora Bohlander, IBAN FR63 1372 1590 1092 8159 0139 6 |
| de-standard-transfer-0 | person | Cora Bohlander | False | Bitte überweisen Sie die Kaution an Cora Bohlander, IBAN FR63 1372 1590 1092 815 |
| lb-standard-transfer-0 | person | Cora Bohlander | False | Iwwerweist d'Kautioun w.e.g. un Cora Bohlander, IBAN FR63 1372 1590 1092 8159 01 |
| en-standard-transfer-1 | person | Corinne Benoit | False | Please transfer the deposit to Corinne Benoit, IBAN BE47 1177 7741 2154. |
| fr-standard-transfer-1 | person | Corinne Benoit | False | Veuillez virer la caution à Corinne Benoit, IBAN BE47 1177 7741 2154. |
| de-standard-transfer-1 | person | Corinne Benoit | False | Bitte überweisen Sie die Kaution an Corinne Benoit, IBAN BE47 1177 7741 2154. |
| lb-standard-transfer-1 | person | Corinne Benoit | False | Iwwerweist d'Kautioun w.e.g. un Corinne Benoit, IBAN BE47 1177 7741 2154. |
| en-standard-card-0 | person | Auguste Fernandez | False | Auguste Fernandez paid the booking with card 4131 8609 1390 9967. |
| fr-standard-card-0 | person | Auguste Fernandez | False | Auguste Fernandez a payé la réservation avec la carte 4131 8609 1390 9967. |
| de-standard-card-0 | person | Auguste Fernandez | False | Auguste Fernandez hat die Buchung mit der Karte 4131 8609 1390 9967 bezahlt. |
| lb-standard-card-0 | person | Auguste Fernandez | False | Auguste Fernandez huet d'Reservatioun mat der Kaart 4131 8609 1390 9967 bezuelt. |
| en-standard-matricule-0 | person | Edward Salz | False | Social security number of Edward Salz: 1961052184125. |
| fr-standard-matricule-0 | person | Edward Salz | False | Matricule de Edward Salz : 1961052184125. |
| de-standard-matricule-0 | person | Edward Salz | False | Sozialversicherungsnummer von Edward Salz: 1961052184125. |
| lb-standard-matricule-0 | person | Edward Salz | False | D'Matrikelnummer fir Edward Salz ass 1961052184125. |
| en-standard-matricule-1 | person | Pero Dietz | False | Social security number of Pero Dietz: 1944051285228. |
| fr-standard-matricule-1 | person | Pero Dietz | False | Matricule de Pero Dietz : 1944051285228. |
| de-standard-matricule-1 | person | Pero Dietz | False | Sozialversicherungsnummer von Pero Dietz: 1944051285228. |
| lb-standard-matricule-1 | person | Pero Dietz | False | D'Matrikelnummer fir Pero Dietz ass 1944051285228. |
| en-standard-job-0 | person | Geneviève Lopez | False | Geneviève Lopez has worked for Deloitte in Berlin since 2019. |
| en-standard-job-0 | organization | Deloitte | False | Geneviève Lopez has worked for Deloitte in Berlin since 2019. |
| en-standard-job-0 | location | Berlin | False | Geneviève Lopez has worked for Deloitte in Berlin since 2019. |
| fr-standard-job-0 | person | Geneviève Lopez | False | Geneviève Lopez travaille chez Deloitte à Berlin depuis 2019. |
| fr-standard-job-0 | organization | Deloitte | False | Geneviève Lopez travaille chez Deloitte à Berlin depuis 2019. |
| fr-standard-job-0 | location | Berlin | False | Geneviève Lopez travaille chez Deloitte à Berlin depuis 2019. |
| de-standard-job-0 | person | Geneviève Lopez | False | Geneviève Lopez arbeitet seit 2019 bei Deloitte in Berlin. |
| de-standard-job-0 | organization | Deloitte | False | Geneviève Lopez arbeitet seit 2019 bei Deloitte in Berlin. |
| de-standard-job-0 | location | Berlin | False | Geneviève Lopez arbeitet seit 2019 bei Deloitte in Berlin. |
| lb-standard-job-0 | person | Geneviève Lopez | False | Geneviève Lopez schafft zënter 2019 bei Deloitte zu Berlin. |
| lb-standard-job-0 | organization | Deloitte | False | Geneviève Lopez schafft zënter 2019 bei Deloitte zu Berlin. |
| lb-standard-job-0 | location | Berlin | False | Geneviève Lopez schafft zënter 2019 bei Deloitte zu Berlin. |
| en-standard-job-1 | person | Ellinor Haering | False | Ellinor Haering has worked for Deutsche Bank in London since 2019. |
| en-standard-job-1 | organization | Deutsche Bank | False | Ellinor Haering has worked for Deutsche Bank in London since 2019. |
| en-standard-job-1 | location | London | False | Ellinor Haering has worked for Deutsche Bank in London since 2019. |
| fr-standard-job-1 | person | Ellinor Haering | False | Ellinor Haering travaille chez Deutsche Bank à Londres depuis 2019. |
| fr-standard-job-1 | organization | Deutsche Bank | False | Ellinor Haering travaille chez Deutsche Bank à Londres depuis 2019. |
| fr-standard-job-1 | location | Londres | False | Ellinor Haering travaille chez Deutsche Bank à Londres depuis 2019. |
| de-standard-job-1 | person | Ellinor Haering | False | Ellinor Haering arbeitet seit 2019 bei Deutsche Bank in London. |
| de-standard-job-1 | organization | Deutsche Bank | False | Ellinor Haering arbeitet seit 2019 bei Deutsche Bank in London. |
| de-standard-job-1 | location | London | False | Ellinor Haering arbeitet seit 2019 bei Deutsche Bank in London. |
| lb-standard-job-1 | person | Ellinor Haering | False | Ellinor Haering schafft zënter 2019 bei Deutsche Bank zu London. |
| lb-standard-job-1 | organization | Deutsche Bank | False | Ellinor Haering schafft zënter 2019 bei Deutsche Bank zu London. |
| lb-standard-job-1 | location | London | False | Ellinor Haering schafft zënter 2019 bei Deutsche Bank zu London. |
| en-standard-address-0 | address | 105, boulevard Royal | False | Jean Davies lives at 105, boulevard Royal, Dudelange. |
| fr-standard-address-0 | address | 105, boulevard Royal | False | Jean Davies habite 105, boulevard Royal à Dudelange. |
| de-standard-address-0 | address | 105, boulevard Royal | False | Jean Davies wohnt unter der Adresse 105, boulevard Royal in Düdelingen. |
| lb-standard-address-0 | address | 105, boulevard Royal | False | Jean Davies wunnt op der Adress 105, boulevard Royal zu Diddeleng. |
| en-standard-address-1 | person | Rachel O'Donnell | False | Rachel O'Donnell lives at 46, rue de la Gare, Luxembourg. |
| en-standard-address-1 | address | 46, rue de la Gare | False | Rachel O'Donnell lives at 46, rue de la Gare, Luxembourg. |
| fr-standard-address-1 | person | Rachel O'Donnell | False | Rachel O'Donnell habite 46, rue de la Gare à Luxembourg. |
| fr-standard-address-1 | address | 46, rue de la Gare | False | Rachel O'Donnell habite 46, rue de la Gare à Luxembourg. |
| de-standard-address-1 | person | Rachel O'Donnell | False | Rachel O'Donnell wohnt unter der Adresse 46, rue de la Gare in Luxemburg. |
| de-standard-address-1 | address | 46, rue de la Gare | False | Rachel O'Donnell wohnt unter der Adresse 46, rue de la Gare in Luxemburg. |
| lb-standard-address-1 | person | Rachel O'Donnell | False | Rachel O'Donnell wunnt op der Adress 46, rue de la Gare zu Lëtzebuerg. |
| lb-standard-address-1 | address | 46, rue de la Gare | False | Rachel O'Donnell wunnt op der Adress 46, rue de la Gare zu Lëtzebuerg. |
| en-standard-profile-0 | person | Chantal Mary | False | More about Chantal Mary on https://www.linkedin.com/in/chantal-mary. |
| fr-standard-profile-0 | person | Chantal Mary | False | Plus d'informations sur Chantal Mary : https://www.linkedin.com/in/chantal-mary. |
| de-standard-profile-0 | person | Chantal Mary | False | Mehr über Chantal Mary unter https://www.linkedin.com/in/chantal-mary. |
| lb-standard-profile-0 | person | Chantal Mary | False | Méi iwwer Chantal Mary op https://www.linkedin.com/in/chantal-mary. |
| en-standard-profile-1 | person | Stefania Metz | False | More about Stefania Metz on https://stefania-metz.lu. |
| fr-standard-profile-1 | person | Stefania Metz | False | Plus d'informations sur Stefania Metz : https://stefania-metz.lu. |
| de-standard-profile-1 | person | Stefania Metz | False | Mehr über Stefania Metz unter https://stefania-metz.lu. |
| lb-standard-profile-1 | person | Stefania Metz | False | Méi iwwer Stefania Metz op https://stefania-metz.lu. |
| en-standard-meeting-0 | person | Iwan Girschner | False | Iwan Girschner met Marthe Valette at the Cargolux offices on Monday. |
| en-standard-meeting-0 | person | Marthe Valette | False | Iwan Girschner met Marthe Valette at the Cargolux offices on Monday. |
| fr-standard-meeting-0 | person | Iwan Girschner | False | Iwan Girschner a rencontré Marthe Valette dans les bureaux de Cargolux lundi. |
| fr-standard-meeting-0 | person | Marthe Valette | False | Iwan Girschner a rencontré Marthe Valette dans les bureaux de Cargolux lundi. |
| de-standard-meeting-0 | person | Iwan Girschner | False | Iwan Girschner hat Marthe Valette am Montag in den Büros von Cargolux getroffen. |
| de-standard-meeting-0 | person | Marthe Valette | False | Iwan Girschner hat Marthe Valette am Montag in den Büros von Cargolux getroffen. |
| lb-standard-meeting-0 | person | Iwan Girschner | False | Iwan Girschner huet de Méindeg Marthe Valette an de Büroe vun Cargolux getraff. |
| lb-standard-meeting-0 | person | Marthe Valette | False | Iwan Girschner huet de Méindeg Marthe Valette an de Büroe vun Cargolux getraff. |
| en-standard-meeting-1 | person | Tun Ries | False | Tun Ries met Ingo Butte at the KPMG offices on Monday. |
| en-standard-meeting-1 | person | Ingo Butte | False | Tun Ries met Ingo Butte at the KPMG offices on Monday. |
| en-standard-meeting-1 | organization | KPMG | False | Tun Ries met Ingo Butte at the KPMG offices on Monday. |
| fr-standard-meeting-1 | person | Tun Ries | False | Tun Ries a rencontré Ingo Butte dans les bureaux de KPMG lundi. |
| fr-standard-meeting-1 | person | Ingo Butte | False | Tun Ries a rencontré Ingo Butte dans les bureaux de KPMG lundi. |
| fr-standard-meeting-1 | organization | KPMG | False | Tun Ries a rencontré Ingo Butte dans les bureaux de KPMG lundi. |
| de-standard-meeting-1 | person | Tun Ries | False | Tun Ries hat Ingo Butte am Montag in den Büros von KPMG getroffen. |
| de-standard-meeting-1 | person | Ingo Butte | False | Tun Ries hat Ingo Butte am Montag in den Büros von KPMG getroffen. |
| de-standard-meeting-1 | organization | KPMG | False | Tun Ries hat Ingo Butte am Montag in den Büros von KPMG getroffen. |
| lb-standard-meeting-1 | person | Tun Ries | False | Tun Ries huet de Méindeg Ingo Butte an de Büroe vun KPMG getraff. |
| lb-standard-meeting-1 | person | Ingo Butte | False | Tun Ries huet de Méindeg Ingo Butte an de Büroe vun KPMG getraff. |
| lb-standard-meeting-1 | organization | KPMG | False | Tun Ries huet de Méindeg Ingo Butte an de Büroe vun KPMG getraff. |
| en-standard-appointment-0 | person | Beverley Hodgson | False | Appointment confirmed for Beverley Hodgson in Paris, call 06 63 30 43 48 to canc |
| en-standard-appointment-0 | location | Paris | False | Appointment confirmed for Beverley Hodgson in Paris, call 06 63 30 43 48 to canc |
| fr-standard-appointment-0 | person | Beverley Hodgson | False | Rendez-vous confirmé pour Beverley Hodgson à Paris, appelez le 06 63 30 43 48 po |
| fr-standard-appointment-0 | location | Paris | False | Rendez-vous confirmé pour Beverley Hodgson à Paris, appelez le 06 63 30 43 48 po |
| de-standard-appointment-0 | person | Beverley Hodgson | False | Termin für Beverley Hodgson in Paris bestätigt, Absage unter 06 63 30 43 48. |
| de-standard-appointment-0 | location | Paris | False | Termin für Beverley Hodgson in Paris bestätigt, Absage unter 06 63 30 43 48. |
| lb-standard-appointment-0 | person | Beverley Hodgson | False | Den Termin fir Beverley Hodgson zu Paräis ass confirméiert, rufft 06 63 30 43 48 |
| lb-standard-appointment-0 | location | Paräis | False | Den Termin fir Beverley Hodgson zu Paräis ass confirméiert, rufft 06 63 30 43 48 |
| en-standard-appointment-1 | location | Berlin | False | Appointment confirmed for Olivia Gardiner in Berlin, call 06 67 51 36 13 to canc |
| fr-standard-appointment-1 | location | Berlin | False | Rendez-vous confirmé pour Olivia Gardiner à Berlin, appelez le 06 67 51 36 13 po |
| de-standard-appointment-1 | location | Berlin | False | Termin für Olivia Gardiner in Berlin bestätigt, Absage unter 06 67 51 36 13. |
| lb-standard-appointment-1 | location | Berlin | False | Den Termin fir Olivia Gardiner zu Berlin ass confirméiert, rufft 06 67 51 36 13  |
| en-standard-invoice-1 | organization | Google | False | Send the invoice to nathalie.pons@web.de, accounting department of Google. |
| fr-standard-invoice-1 | organization | Google | False | Envoyez la facture à nathalie.pons@web.de, service comptabilité de Google. |
| de-standard-invoice-1 | organization | Google | False | Senden Sie die Rechnung an nathalie.pons@web.de, Buchhaltung von Google. |
| lb-standard-invoice-1 | organization | Google | False | Schéckt d'Rechnung un nathalie.pons@web.de, Comptabilitéit vun Google. |
| en-standard-form-0 | person | Léontine Goedert | False | Name: Léontine Goedert, date of birth: 11/10/1945, place of birth: Brussels. |
| en-standard-form-0 | location | Brussels | False | Name: Léontine Goedert, date of birth: 11/10/1945, place of birth: Brussels. |
| fr-standard-form-0 | person | Léontine Goedert | False | Nom : Léontine Goedert, date de naissance : 11/10/1945, lieu de naissance : Brux |
| fr-standard-form-0 | location | Bruxelles | False | Nom : Léontine Goedert, date de naissance : 11/10/1945, lieu de naissance : Brux |
| de-standard-form-0 | person | Léontine Goedert | False | Name: Léontine Goedert, Geburtsdatum: 11.10.1945, Geburtsort: Brüssel. |
| de-standard-form-0 | location | Brüssel | False | Name: Léontine Goedert, Geburtsdatum: 11.10.1945, Geburtsort: Brüssel. |
| lb-standard-form-0 | person | Léontine Goedert | False | Numm: Léontine Goedert, Gebuertsdatum: 11.10.1945, Gebuertsplaz: Bréissel. |
| lb-standard-form-0 | location | Bréissel | False | Numm: Léontine Goedert, Gebuertsdatum: 11.10.1945, Gebuertsplaz: Bréissel. |
| en-standard-form-1 | person | Terence Lewis | False | Name: Terence Lewis, date of birth: 15/05/1982, place of birth: Esch-sur-Alzette |
| fr-standard-form-1 | person | Terence Lewis | False | Nom : Terence Lewis, date de naissance : 15/05/1982, lieu de naissance : Esch-su |
| de-standard-form-1 | person | Terence Lewis | False | Name: Terence Lewis, Geburtsdatum: 15.05.1982, Geburtsort: Esch an der Alzette. |
| lb-standard-form-1 | person | Terence Lewis | False | Numm: Terence Lewis, Gebuertsdatum: 15.05.1982, Gebuertsplaz: Esch-Uelzecht. |
| en-hard-coordinated-0 | person | Lis | False | Lis and Charel drove to Betzdorf. |
| fr-hard-coordinated-0 | person | Lis | False | Lis et Charel sont partis à Betzdorf. |
| de-hard-coordinated-0 | person | Lis | False | Lis und Charel sind nach Betzdorf gefahren. |
| lb-hard-coordinated-0 | person | Lis | False | D'Lis an de Charel sinn op Betzder gefuer. |
| en-hard-coordinated-1 | person | Méck | False | Méck and Léontine drove to Bourscheid. |
| en-hard-coordinated-1 | person | Léontine | False | Méck and Léontine drove to Bourscheid. |
| fr-hard-coordinated-1 | person | Méck | False | Méck et Léontine sont partis à Bourscheid. |
| fr-hard-coordinated-1 | person | Léontine | False | Méck et Léontine sont partis à Bourscheid. |
| de-hard-coordinated-1 | person | Méck | False | Méck und Léontine sind nach Burscheid gefahren. |
| de-hard-coordinated-1 | person | Léontine | False | Méck und Léontine sind nach Burscheid gefahren. |
| lb-hard-coordinated-1 | person | Méck | False | De Méck an d'Léontine sinn op Buerschent gefuer. |
| lb-hard-coordinated-1 | person | Léontine | False | De Méck an d'Léontine sinn op Buerschent gefuer. |
| en-hard-letter-0 | person | Welter | False | Mrs Welter from CNS wrote us a letter. |
| fr-hard-letter-0 | person | Welter | False | Madame Welter de CNS nous a écrit une lettre. |
| de-hard-letter-0 | person | Welter | False | Frau Welter von CNS hat uns einen Brief geschrieben. |
| lb-hard-letter-0 | person | Welter | False | D'Madame Welter vun CNS huet eis e Bréif geschriwwen. |
| en-hard-letter-1 | person | Feltgen | False | Mr Feltgen from CNS wrote us a letter. |
| fr-hard-letter-1 | person | Feltgen | False | Monsieur Feltgen de CNS nous a écrit une lettre. |
| de-hard-letter-1 | person | Feltgen | False | Herr Feltgen von CNS hat uns einen Brief geschrieben. |
| lb-hard-letter-1 | person | Feltgen | False | Den Här Feltgen vun CNS huet eis e Bréif geschriwwen. |
| en-hard-family-1 | person | Romaine | False | My mother and Romaine are coming to Goesdorf on Sunday. |
| fr-hard-family-1 | person | Romaine | False | Ma mère et Romaine viennent à Goesdorf dimanche. |
| de-hard-family-1 | person | Romaine | False | Meine Mutter und Romaine kommen am Sonntag nach Goesdorf. |
| lb-hard-family-1 | person | Romaine | False | Meng Mamm an d'Romaine kommen e Sonndeg op Géisdref. |
| en-hard-workplace-0 | organization | Brasserie Schuman | False | Néckel has worked at Brasserie Schuman in Lorentzweiler for years. |
| fr-hard-workplace-0 | organization | Brasserie Schuman | False | Néckel travaille depuis des années chez Brasserie Schuman à Lorentzweiler. |
| de-hard-workplace-0 | organization | Brasserie Schuman | False | Néckel arbeitet seit Jahren bei Brasserie Schuman in Lorentzweiler. |
| lb-hard-workplace-0 | organization | Brasserie Schuman | False | Den Néckel schafft zënter Joren bei Brasserie Schuman zu Luerenzweiler. |
| en-hard-hotline-0 | person | Schmit | False | For questions: Dr Schmit, tel. +49 (0)30 70455853. |
| fr-hard-hotline-0 | person | Schmit | False | Pour toute question : le docteur Schmit, tél. +49 (0)30 70455853. |
| de-hard-hotline-0 | person | Schmit | False | Bei Fragen: Dr. Schmit, Tel. +49 (0)30 70455853. |
| lb-hard-hotline-0 | person | Schmit | False | Fir Froen: den Dokter Schmit, Tel. +49 (0)30 70455853. |
| en-hard-hotline-1 | person | Kremer | False | For questions: Dr Kremer, tel. (+352) 621 920 868. |
| fr-hard-hotline-1 | person | Kremer | False | Pour toute question : le docteur Kremer, tél. (+352) 621 920 868. |
| de-hard-hotline-1 | person | Kremer | False | Bei Fragen: Dr. Kremer, Tel. (+352) 621 920 868. |
| lb-hard-hotline-1 | person | Kremer | False | Fir Froen: den Dokter Kremer, Tel. (+352) 621 920 868. |
| en-hard-chat-0 | person | jhemp | False | jhemp and nelly are coming tonight |
| en-hard-chat-0 | person | nelly | False | jhemp and nelly are coming tonight |
| fr-hard-chat-0 | person | jhemp | False | jhemp et nelly viennent ce soir |
| fr-hard-chat-0 | person | nelly | False | jhemp et nelly viennent ce soir |
| de-hard-chat-0 | person | jhemp | False | jhemp und nelly kommen heute abend |
| de-hard-chat-0 | person | nelly | False | jhemp und nelly kommen heute abend |
| lb-hard-chat-0 | person | jhemp | False | jhemp an nelly kommen haut den owend |
| lb-hard-chat-0 | person | nelly | False | jhemp an nelly kommen haut den owend |
| en-hard-chat-1 | person | jang | False | jang and gusty are coming tonight |
| en-hard-chat-1 | person | gusty | False | jang and gusty are coming tonight |
| fr-hard-chat-1 | person | jang | False | jang et gusty viennent ce soir |
| fr-hard-chat-1 | person | gusty | False | jang et gusty viennent ce soir |
| de-hard-chat-1 | person | jang | False | jang und gusty kommen heute abend |
| de-hard-chat-1 | person | gusty | False | jang und gusty kommen heute abend |
| lb-hard-chat-1 | person | jang | False | jang an gusty kommen haut den owend |
| lb-hard-chat-1 | person | gusty | False | jang an gusty kommen haut den owend |
| en-hard-born-0 | person | Fernande | False | Fernande, b. 08/02/1982 in Finsterthal. |
| en-hard-born-0 | date_of_birth | 08/02/1982 | False | Fernande, b. 08/02/1982 in Finsterthal. |
| fr-hard-born-0 | person | Fernande | False | Fernande, né(e) le 08/02/1982 à Finsterthal. |
| de-hard-born-0 | person | Fernande | False | Fernande, geb. 08.02.1982 in Finsterthal. |
| lb-hard-born-0 | person | Fernande | False | D'Fernande, gebuer 08.02.1982 zu Fënsterdall. |
| en-hard-born-1 | person | Pol | False | Pol, b. 23/06/1983 in Reisdorf. |
| en-hard-born-1 | date_of_birth | 23/06/1983 | False | Pol, b. 23/06/1983 in Reisdorf. |
| fr-hard-born-1 | person | Pol | False | Pol, né(e) le 23/06/1983 à Reisdorf. |
| de-hard-born-1 | person | Pol | False | Pol, geb. 23.06.1983 in Reisdorf. |
| lb-hard-born-1 | person | Pol | False | De Pol, gebuer 23.06.1983 zu Reisduerf. |
| en-hard-separated-0 | person | Ries | False | Dr Ries and Mr Weber have separated. |
| en-hard-separated-0 | person | Weber | False | Dr Ries and Mr Weber have separated. |
| fr-hard-separated-0 | person | Ries | False | La docteure Ries et Monsieur Weber se sont séparés. |
| fr-hard-separated-0 | person | Weber | False | La docteure Ries et Monsieur Weber se sont séparés. |
| de-hard-separated-0 | person | Ries | False | Dr. Ries und Herr Weber haben sich getrennt. |
| de-hard-separated-0 | person | Weber | False | Dr. Ries und Herr Weber haben sich getrennt. |
| lb-hard-separated-0 | person | Ries | False | D'Doktesch Ries an den Här Weber hu sech getrennt. |
| lb-hard-separated-0 | person | Weber | False | D'Doktesch Ries an den Här Weber hu sech getrennt. |
| en-hard-separated-1 | person | Wagner | False | Dr Wagner and Mrs Mangen have separated. |
| en-hard-separated-1 | person | Mangen | False | Dr Wagner and Mrs Mangen have separated. |
| fr-hard-separated-1 | person | Wagner | False | Le docteur Wagner et Madame Mangen se sont séparés. |
| fr-hard-separated-1 | person | Mangen | False | Le docteur Wagner et Madame Mangen se sont séparés. |
| de-hard-separated-1 | person | Wagner | False | Dr. Wagner und Frau Mangen haben sich getrennt. |
| de-hard-separated-1 | person | Mangen | False | Dr. Wagner und Frau Mangen haben sich getrennt. |
| lb-hard-separated-1 | person | Wagner | False | Den Dokter Wagner an d'Madame Mangen hu sech getrennt. |
| lb-hard-separated-1 | person | Mangen | False | Den Dokter Wagner an d'Madame Mangen hu sech getrennt. |
| en-hard-text-0 | person | Norbert | False | Email Norbert at norbert58@gmail.com or text 0170/2470822. |
| fr-hard-text-0 | person | Norbert | False | Écris à Norbert sur norbert58@gmail.com ou par SMS au 0170/2470822. |
| de-hard-text-0 | person | Norbert | False | Schreib Norbert an norbert58@gmail.com oder eine SMS an 0170/2470822. |
| lb-hard-text-0 | person | Norbert | False | Schreif dem Norbert op norbert58@gmail.com oder eng SMS op 0170/2470822. |
| en-hard-brother-1 | organization | Cactus | False | Gréit's brother works at Cactus. |
| fr-hard-brother-1 | organization | Cactus | False | Le frère de Gréit travaille chez Cactus. |
| de-hard-brother-1 | organization | Cactus | False | Der Bruder von Gréit arbeitet bei Cactus. |
| lb-hard-brother-1 | organization | Cactus | False | Der Gréit hire Brudder schafft bei Cactus. |
| en-hard-card-0 | person | Hansen | False | Card 5249 7465 0752 9178 belongs to Dr Hansen. |
| fr-hard-card-0 | person | Hansen | False | La carte 5249 7465 0752 9178 appartient à la docteure Hansen. |
| de-hard-card-0 | person | Hansen | False | Die Karte 5249 7465 0752 9178 gehört Dr. Hansen. |
| lb-hard-card-0 | person | Hansen | False | D'Kaart 5249 7465 0752 9178 gehéiert der Doktesch Hansen. |
| en-hard-card-1 | person | Weber | False | Card 3436 671276 84266 belongs to Mrs Weber. |
| fr-hard-card-1 | person | Weber | False | La carte 3436 671276 84266 appartient à Madame Weber. |
| de-hard-card-1 | person | Weber | False | Die Karte 3436 671276 84266 gehört Frau Weber. |
| lb-hard-card-1 | person | Weber | False | D'Kaart 3436 671276 84266 gehéiert der Madame Weber. |
