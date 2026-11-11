# Presidio and this tool on the same data

Share of gold spans that a predicted span overlaps, whatever its type: what a redaction would have
covered. Presidio 2.2 runs with spaCy's large models for English, French and German and `xx_ent_wiki_sm`
for Luxembourgish, organizations on, phone regions extended to Luxembourg and its neighbours, and is told
the language of every text. This tool guesses the language and runs with `dates_and_times`, because
Presidio reports every date and time. `rules + NER` uses GLiNER, `rules + fast NER` the small spaCy
models, `rules + both` takes what either model finds.

None of these sets was used to develop the rules they test: see `eval/README.md` for what was.

## Eval split

Most names here are in this tool's word lists.

| gold type | presidio | rules | rules + fast NER | rules + NER | rules + both |
|---|---|---|---|---|---|
| all | 0.89 | 0.99 | 0.99 | 1.00 | 1.00 |
| person | 0.87 | 0.98 | 0.98 | 1.00 | 1.00 |
| organization | 0.90 | 1.00 | 0.98 | 1.00 | 1.00 |
| location | 0.94 | 1.00 | 1.00 | 1.00 | 1.00 |
| address | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| email | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| phone | 0.91 | 1.00 | 1.00 | 1.00 | 1.00 |
| iban | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| credit_card | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| national_id | 0.25 | 1.00 | 1.00 | 1.00 | 1.00 |
| ip_address | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| url | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| date_of_birth | 0.88 | 1.00 | 1.00 | 1.00 | 1.00 |

## Eval split with other names

Persons, firms and towns replaced by ones written without looking at any word list.

| gold type | presidio | rules | rules + fast NER | rules + NER | rules + both |
|---|---|---|---|---|---|
| all | 0.87 | 0.68 | 0.93 | 0.95 | 0.97 |
| person | 0.88 | 0.57 | 0.90 | 0.95 | 0.99 |
| organization | 0.75 | 0.31 | 0.83 | 0.88 | 0.94 |
| location | 0.86 | 0.54 | 0.90 | 0.91 | 0.90 |
| address | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| email | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| phone | 0.91 | 1.00 | 1.00 | 1.00 | 1.00 |
| iban | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| credit_card | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| national_id | 0.25 | 1.00 | 1.00 | 1.00 | 1.00 |
| ip_address | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| url | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| date_of_birth | 0.83 | 1.00 | 1.00 | 1.00 | 1.00 |

## Eval split with other names, Luxembourgish only



| gold type | presidio | rules | rules + fast NER | rules + NER | rules + both |
|---|---|---|---|---|---|
| all | 0.82 | 0.68 | 0.92 | 0.97 | 0.97 |
| person | 0.81 | 0.57 | 0.93 | 1.00 | 1.00 |
| organization | 0.75 | 0.33 | 0.75 | 0.92 | 0.92 |
| location | 0.80 | 0.55 | 0.85 | 0.85 | 0.85 |
| address | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| email | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| phone | 0.88 | 1.00 | 1.00 | 1.00 | 1.00 |
| iban | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| credit_card | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| national_id | 0.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| ip_address | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| url | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| date_of_birth | 0.83 | 1.00 | 1.00 | 1.00 | 1.00 |

## Hits on the 80 entity-free sentences of the eval split

Every hit that is no date or time is an over-redaction.

| tool | en | fr | de | lb | total | dates and times |
|---|---|---|---|---|---|---|
| presidio | 23 | 14 | 8 | 26 | 71 | 13 |
| rules | 1 | 3 | 2 | 4 | 10 | 8 |
| rules + fast NER | 3 | 3 | 2 | 5 | 13 | 8 |
| rules + NER | 3 | 4 | 1 | 5 | 13 | 8 |
| rules + both | 3 | 4 | 3 | 6 | 16 | 8 |

## Hits on 120 everyday sentences

Every hit that is no date or time is an over-redaction.

| tool | en | fr | de | lb | total | dates and times |
|---|---|---|---|---|---|---|
| presidio | 30 | 40 | 5 | 40 | 115 | 8 |
| rules | 1 | 0 | 0 | 1 | 2 | 0 |
| rules + fast NER | 2 | 3 | 2 | 2 | 9 | 0 |
| rules + NER | 3 | 3 | 0 | 1 | 7 | 0 |
| rules + both | 4 | 4 | 2 | 2 | 12 | 0 |

## WikiANN, 300 Wikipedia sentences per language

Test split, silver labels for persons, organizations and locations.

| lang | gold type | presidio | rules | rules + fast NER | rules + NER | rules + both |
|---|---|---|---|---|---|---|
| en | all names | 0.79 | 0.53 | 0.79 | 0.81 | 0.86 |
| en | person | 0.92 | 0.76 | 0.94 | 0.95 | 0.97 |
| en | organization | 0.60 | 0.31 | 0.63 | 0.60 | 0.70 |
| en | location | 0.83 | 0.49 | 0.77 | 0.84 | 0.89 |
| fr | all names | 0.82 | 0.55 | 0.80 | 0.80 | 0.82 |
| fr | person | 0.93 | 0.80 | 0.95 | 0.96 | 0.97 |
| fr | organization | 0.63 | 0.39 | 0.66 | 0.64 | 0.70 |
| fr | location | 0.87 | 0.46 | 0.80 | 0.79 | 0.81 |
| de | all names | 0.87 | 0.48 | 0.78 | 0.79 | 0.85 |
| de | person | 0.96 | 0.72 | 0.93 | 0.95 | 0.97 |
| de | organization | 0.74 | 0.27 | 0.58 | 0.64 | 0.69 |
| de | location | 0.91 | 0.44 | 0.81 | 0.78 | 0.88 |
| lb | all names | 0.83 | 0.57 | 0.75 | 0.70 | 0.77 |
| lb | person | 0.96 | 0.84 | 0.95 | 0.91 | 0.95 |
| lb | organization | 0.69 | 0.34 | 0.55 | 0.47 | 0.59 |
| lb | location | 0.85 | 0.56 | 0.76 | 0.73 | 0.78 |

## WikiANN, how much of what was marked is an entity



| tool | predictions | on a labelled item |
|---|---|---|
| presidio | 1673.00 | 0.82 |
| rules | 901.00 | 0.97 |
| rules + fast NER | 1360.00 | 0.95 |
| rules + NER | 1299.00 | 0.94 |
| rules + both | 1389.00 | 0.92 |

## ai4privacy pii-masking-300k, 360 synthetic documents

English, French and German. Sex and title have no detector in this tool on purpose.

| lang | gold type | presidio | rules | rules + fast NER | rules + NER | rules + both |
|---|---|---|---|---|---|---|
| en | all | 0.59 | 0.73 | 0.75 | 0.80 | 0.80 |
| fr | all | 0.51 | 0.72 | 0.77 | 0.81 | 0.84 |
| de | all | 0.49 | 0.74 | 0.78 | 0.82 | 0.84 |

## ai4privacy by label



| gold type | presidio | rules | rules + fast NER | rules + NER | rules + both |
|---|---|---|---|---|---|
| BOD | 0.60 | 1.00 | 1.00 | 1.00 | 1.00 |
| BUILDING | 0.01 | 0.78 | 0.78 | 0.84 | 0.84 |
| CITY | 0.73 | 0.78 | 0.86 | 0.92 | 0.95 |
| COUNTRY | 0.78 | 0.69 | 0.74 | 0.71 | 0.74 |
| DATE | 0.57 | 0.96 | 0.96 | 0.96 | 0.96 |
| DRIVERLICENSE | 0.58 | 0.79 | 0.82 | 0.81 | 0.84 |
| EMAIL | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| GEOCOORD | 0.86 | 0.86 | 1.00 | 0.86 | 1.00 |
| GIVENNAME1 | 0.58 | 0.71 | 0.80 | 0.95 | 0.98 |
| GIVENNAME2 | 0.59 | 0.70 | 0.78 | 0.93 | 0.93 |
| IDCARD | 0.39 | 0.90 | 0.91 | 0.91 | 0.91 |
| IP | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| LASTNAME1 | 0.60 | 0.50 | 0.69 | 0.89 | 0.93 |
| LASTNAME2 | 0.54 | 0.65 | 0.77 | 0.88 | 0.92 |
| LASTNAME3 | 0.36 | 0.64 | 0.73 | 0.82 | 0.73 |
| PASS | 0.22 | 0.72 | 0.76 | 0.77 | 0.80 |
| PASSPORT | 0.52 | 0.91 | 0.92 | 0.91 | 0.92 |
| POSTCODE | 0.65 | 0.72 | 0.72 | 0.77 | 0.77 |
| SECADDRESS | 0.29 | 0.58 | 0.58 | 0.75 | 0.75 |
| SEX | 0.14 | 0.02 | 0.14 | 0.10 | 0.20 |
| SOCIALNUMBER | 0.71 | 0.78 | 0.78 | 0.78 | 0.78 |
| STATE | 0.46 | 0.73 | 0.81 | 0.76 | 0.79 |
| STREET | 0.62 | 0.75 | 0.81 | 0.93 | 0.94 |
| TEL | 0.87 | 0.96 | 0.96 | 0.96 | 0.96 |
| TIME | 0.17 | 0.75 | 0.75 | 0.76 | 0.76 |
| TITLE | 0.40 | 0.10 | 0.20 | 0.21 | 0.26 |
| USERNAME | 0.31 | 0.47 | 0.56 | 0.81 | 0.84 |

## ai4privacy, how much of what was marked is labelled



| tool | predictions | on a labelled item |
|---|---|---|
| presidio | 2261.00 | 0.77 |
| rules | 1769.00 | 0.93 |
| rules + fast NER | 2052.00 | 0.85 |
| rules + NER | 2039.00 | 0.89 |
| rules + both | 2234.00 | 0.83 |

## Runtime

Two CPU cores, models loaded. Presidio text by text over all sets, this tool on the eval split in one call.

| tool | ms per 1,000 words |
|---|---|
| presidio | 432.56 |
| rules | 4.10 |
| rules + fast NER | 89.99 |
| rules + NER | 7798.80 |
| rules + both | 7650.67 |
