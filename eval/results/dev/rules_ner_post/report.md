# Evaluation: dev/rules_ner_post

Config: `eval/configs/rules_ner_post.json`. 248 sentences, 2297 words, 504 gold spans.

Recall comes first: a missed identifier leaks personal data, an over-redaction only costs readability.
`overlap` counts a prediction of the right type that overlaps the gold span, `exact` needs identical offsets, and `any_type_recall` counts a gold span as redacted if any prediction overlaps it.

Runtime: 312935.51 ms per 1,000 words (detection only).

## By language and subset

| lang | subset | recall_overlap | recall_exact | any_type_recall | precision_overlap | f1_overlap | f1_exact | n_gold | n_pred |
|---|---|---|---|---|---|---|---|---|---|
| de | hard | 0.97 | 0.93 | 0.97 | 1.00 | 0.98 | 0.95 | 60 | 58 |
| de | standard | 0.98 | 0.98 | 0.98 | 1.00 | 0.99 | 0.99 | 66 | 65 |
| en | hard | 0.92 | 0.88 | 0.92 | 1.00 | 0.96 | 0.92 | 60 | 55 |
| en | standard | 0.97 | 0.94 | 1.00 | 1.00 | 0.98 | 0.95 | 66 | 64 |
| fr | hard | 0.98 | 0.95 | 0.98 | 1.00 | 0.99 | 0.96 | 60 | 59 |
| fr | standard | 0.98 | 0.98 | 0.98 | 1.00 | 0.99 | 0.99 | 66 | 65 |
| lb | hard | 0.90 | 0.87 | 0.93 | 0.98 | 0.94 | 0.90 | 60 | 55 |
| lb | standard | 0.98 | 0.98 | 0.98 | 1.00 | 0.99 | 0.99 | 66 | 65 |

## Names only (person, organization, location, address)

| lang | subset | recall_overlap | recall_exact | precision_overlap | f1_overlap | n_gold | n_pred |
|---|---|---|---|---|---|---|---|
| de | hard | 0.96 | 0.91 | 1.00 | 0.98 | 46 | 44 |
| de | standard | 0.98 | 0.98 | 1.00 | 0.99 | 42 | 41 |
| en | hard | 0.93 | 0.89 | 1.00 | 0.97 | 46 | 43 |
| en | standard | 0.95 | 0.90 | 1.00 | 0.98 | 42 | 40 |
| fr | hard | 0.98 | 0.93 | 1.00 | 0.99 | 46 | 45 |
| fr | standard | 0.98 | 0.98 | 1.00 | 0.99 | 42 | 41 |
| lb | hard | 0.87 | 0.83 | 0.98 | 0.92 | 46 | 41 |
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
| address | 1.00 | 0.75 | 1.00 | 1.00 | 1.00 | 0.75 | 8 | 8 |
| credit_card | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 16 | 16 |
| date_of_birth | 0.92 | 0.92 | 0.92 | 1.00 | 0.96 | 0.96 | 24 | 22 |
| email | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 32 | 32 |
| iban | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 16 | 16 |
| ip_address | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 8 | 8 |
| location | 0.96 | 0.96 | 1.00 | 1.00 | 0.98 | 0.98 | 80 | 77 |
| national_id | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 16 | 16 |
| organization | 0.98 | 0.96 | 0.98 | 0.98 | 0.98 | 0.96 | 48 | 48 |
| person | 0.94 | 0.91 | 0.94 | 1.00 | 0.97 | 0.94 | 216 | 203 |
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
| de | person | 0.94 | 1.00 | 0.97 | 54 | 51 |
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
| lb | location | 0.95 | 1.00 | 0.97 | 20 | 19 |
| lb | national_id | 1.00 | 1.00 | 1.00 | 4 | 4 |
| lb | organization | 0.92 | 0.92 | 0.92 | 12 | 12 |
| lb | person | 0.91 | 1.00 | 0.95 | 54 | 49 |
| lb | phone | 1.00 | 1.00 | 1.00 | 8 | 8 |
| lb | url | 1.00 | 1.00 | 1.00 | 2 | 2 |

## Gold names found in the gazetteer

Share of distinct gold person, location and organization strings that the built-in gazetteer contains. High values mean rules-only recall on names partly reflects list overlap with the generator.

| type | distinct | in_gazetteer | share |
|---|---|---|---|
| location | 42 | 32 | 0.76 |
| organization | 11 | 6 | 0.55 |
| person | 50 | 17 | 0.34 |

## Missed spans (19)

| id | type | text | covered | sentence |
|---|---|---|---|---|
| fr-standard-matricule-1 | person | Pero Dietz | False | Matricule de Pero Dietz : 1944051285228. |
| en-standard-address-0 | location | Dudelange | True | Jean Davies lives at 105, boulevard Royal, Dudelange. |
| en-standard-address-1 | location | Luxembourg | True | Rachel O'Donnell lives at 46, rue de la Gare, Luxembourg. |
| de-standard-meeting-1 | person | Ingo Butte | False | Tun Ries hat Ingo Butte am Montag in den Büros von KPMG getroffen. |
| lb-standard-invoice-1 | organization | Google | False | Schéckt d'Rechnung un nathalie.pons@web.de, Comptabilitéit vun Google. |
| lb-hard-letter-1 | person | Feltgen | False | Den Här Feltgen vun CNS huet eis e Bréif geschriwwen. |
| lb-hard-workplace-1 | location | Welschent | True | De Charel schafft zënter Joren bei ADEM zu Welschent. |
| en-hard-chat-1 | person | jang | False | jang and gusty are coming tonight |
| en-hard-chat-1 | person | gusty | False | jang and gusty are coming tonight |
| de-hard-chat-1 | person | jang | False | jang und gusty kommen heute abend |
| lb-hard-chat-1 | person | jang | False | jang an gusty kommen haut den owend |
| lb-hard-chat-1 | person | gusty | False | jang an gusty kommen haut den owend |
| en-hard-born-0 | date_of_birth | 08/02/1982 | False | Fernande, b. 08/02/1982 in Finsterthal. |
| en-hard-born-1 | date_of_birth | 23/06/1983 | False | Pol, b. 23/06/1983 in Reisdorf. |
| lb-hard-separated-0 | person | Ries | True | D'Doktesch Ries an den Här Weber hu sech getrennt. |
| en-hard-text-0 | person | Norbert | False | Email Norbert at norbert58@gmail.com or text 0170/2470822. |
| fr-hard-text-0 | person | Norbert | False | Écris à Norbert sur norbert58@gmail.com ou par SMS au 0170/2470822. |
| de-hard-text-0 | person | Norbert | False | Schreib Norbert an norbert58@gmail.com oder eine SMS an 0170/2470822. |
| lb-hard-text-0 | person | Norbert | False | Schreif dem Norbert op norbert58@gmail.com oder eng SMS op 0170/2470822. |
