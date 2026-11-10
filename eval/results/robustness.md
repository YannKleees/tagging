# Robustness

Recall only. `rules + NER` is the package default with the GLiNER model.

## The eval split with other names and damaged text

The names of the eval split are replaced by names written without looking at the word lists (40 first
names, 40 surnames, 20 small firms, 20 towns), or the text loses its diacritics or its casing. Columns
`person` to `structured types` count a span of the right type that overlaps the gold span.

| config | variant | person | organization | location | address | structured types | names covered, any type | names exact |
|---|---|---|---|---|---|---|---|---|
| rules | original eval split | 0.98 | 1.00 | 1.00 | 1.00 | 1.00 | 0.99 | 0.99 |
| rules | new person names | 0.56 | 1.00 | 1.00 | 1.00 | 1.00 | 0.74 | 0.73 |
| rules | new persons, firms and places | 0.52 | 0.06 | 0.53 | 1.00 | 1.00 | 0.52 | 0.46 |
| rules | no diacritics | 0.86 | 0.92 | 0.91 | 1.00 | 0.97 | 0.89 | 0.88 |
| rules | all lowercase | 0.72 | 0.67 | 0.96 | 0.00 | 1.00 | 0.77 | 0.53 |
| rules | ALL CAPITALS | 0.96 | 1.00 | 0.95 | 1.00 | 0.99 | 0.98 | 0.94 |
| rules + NER | original eval split | 1.00 | 0.98 | 1.00 | 1.00 | 1.00 | 1.00 | 0.97 |
| rules + NER | new person names | 0.94 | 0.94 | 1.00 | 1.00 | 1.00 | 0.97 | 0.92 |
| rules + NER | new persons, firms and places | 0.92 | 0.85 | 0.96 | 1.00 | 1.00 | 0.94 | 0.91 |
| rules + NER | no diacritics | 0.98 | 0.96 | 0.99 | 1.00 | 0.97 | 0.99 | 0.94 |
| rules + NER | all lowercase | 0.98 | 0.90 | 0.95 | 1.00 | 1.00 | 0.98 | 0.92 |
| rules + NER | ALL CAPITALS | 0.97 | 0.90 | 0.93 | 1.00 | 0.99 | 0.99 | 0.79 |

## WikiANN, 300 Wikipedia sentences per language

Silver labels for persons, organizations and locations, and a different idea of an entity than this tool
has (countries, sports clubs, historical figures). Columns `person` to `all covered` count an entity that
any predicted span overlaps.

| lang | config | person | organization | location | all covered | right type | predictions on a gold entity | entities |
|---|---|---|---|---|---|---|---|---|
| lb | rules | 0.84 | 0.34 | 0.51 | 0.55 | 0.48 | 0.98 | 382 |
| lb | rules + NER | 0.91 | 0.52 | 0.73 | 0.72 | 0.62 | 0.93 | 382 |
| de | rules | 0.72 | 0.24 | 0.37 | 0.44 | 0.38 | 0.97 | 398 |
| de | rules + NER | 0.94 | 0.61 | 0.77 | 0.78 | 0.70 | 0.92 | 398 |
| fr | rules | 0.80 | 0.36 | 0.38 | 0.51 | 0.38 | 0.99 | 399 |
| fr | rules + NER | 0.96 | 0.66 | 0.79 | 0.80 | 0.70 | 0.97 | 399 |
| en | rules | 0.76 | 0.28 | 0.31 | 0.47 | 0.37 | 0.98 | 387 |
| en | rules + NER | 0.93 | 0.57 | 0.77 | 0.77 | 0.64 | 0.94 | 387 |

## ai4privacy pii-masking-300k, 120 synthetic documents per language

Share of labelled items that any predicted span overlaps. Many labels have no detector here (passports,
driving licences, foreign ID numbers, passwords, times), and the formats are not Luxembourg's.

| lang | rules | rules + NER |
|---|---|---|
| de | 0.19 | 0.41 |
| en | 0.20 | 0.43 |
| fr | 0.16 | 0.39 |

| label | rules | rules + NER | items |
|---|---|---|---|
| BOD | 0.43 | 0.50 | 122 |
| BUILDING | 0.13 | 0.29 | 68 |
| CITY | 0.11 | 0.78 | 74 |
| COUNTRY | 0.38 | 0.41 | 68 |
| DATE | 0.05 | 0.05 | 77 |
| DRIVERLICENSE | 0.00 | 0.04 | 122 |
| EMAIL | 1.00 | 1.00 | 120 |
| GEOCOORD | 0.00 | 0.29 | 14 |
| GIVENNAME1 | 0.08 | 0.93 | 86 |
| GIVENNAME2 | 0.04 | 0.93 | 27 |
| IDCARD | 0.00 | 0.07 | 138 |
| IP | 1.00 | 1.00 | 92 |
| LASTNAME1 | 0.02 | 0.85 | 107 |
| LASTNAME2 | 0.00 | 0.92 | 26 |
| LASTNAME3 | 0.00 | 0.91 | 11 |
| PASS | 0.00 | 0.05 | 82 |
| PASSPORT | 0.03 | 0.07 | 150 |
| POSTCODE | 0.00 | 0.18 | 65 |
| SECADDRESS | 0.00 | 0.67 | 24 |
| SEX | 0.00 | 0.09 | 96 |
| SOCIALNUMBER | 0.06 | 0.12 | 111 |
| STATE | 0.06 | 0.16 | 67 |
| STREET | 0.22 | 0.93 | 68 |
| TEL | 0.79 | 0.79 | 76 |
| TIME | 0.00 | 0.03 | 181 |
| TITLE | 0.09 | 0.22 | 105 |
| USERNAME | 0.00 | 0.69 | 129 |
