# Robustness

Recall only. `rules + NER` is the package default with the GLiNER model, `rules + fast NER` the spaCy backend.

## The eval split with other names and damaged text

The names of the eval split are replaced by names written without looking at the word lists (40 first
names, 40 surnames, 20 small firms, 20 towns), or the text loses its diacritics or its casing. Columns
`person` to `structured types` count a span of the right type that overlaps the gold span.

| config | variant | person | organization | location | address | structured types | names covered, any type | names exact |
|---|---|---|---|---|---|---|---|---|
| rules | original eval split | 0.98 | 1.00 | 1.00 | 1.00 | 1.00 | 0.99 | 0.99 |
| rules | new person names | 0.58 | 1.00 | 1.00 | 1.00 | 1.00 | 0.75 | 0.74 |
| rules | new persons, firms and places | 0.54 | 0.06 | 0.60 | 1.00 | 1.00 | 0.54 | 0.48 |
| rules | no diacritics | 0.86 | 0.92 | 0.91 | 1.00 | 0.97 | 0.89 | 0.88 |
| rules | all lowercase | 0.72 | 0.67 | 0.96 | 0.00 | 1.00 | 0.77 | 0.53 |
| rules | ALL CAPITALS | 0.96 | 1.00 | 0.95 | 1.00 | 0.99 | 0.98 | 0.94 |
| rules + fast NER | original eval split | 0.97 | 0.96 | 1.00 | 1.00 | 1.00 | 0.98 | 0.97 |
| rules + fast NER | new person names | 0.80 | 0.96 | 1.00 | 1.00 | 1.00 | 0.92 | 0.84 |
| rules + fast NER | new persons, firms and places | 0.80 | 0.35 | 0.85 | 1.00 | 1.00 | 0.87 | 0.74 |
| rules + fast NER | no diacritics | 0.89 | 0.94 | 0.97 | 1.00 | 0.97 | 0.95 | 0.89 |
| rules + fast NER | all lowercase | 0.82 | 0.71 | 0.99 | 0.00 | 1.00 | 0.86 | 0.76 |
| rules + fast NER | ALL CAPITALS | 0.87 | 0.92 | 0.86 | 1.00 | 0.99 | 0.96 | 0.83 |
| rules + NER | original eval split | 1.00 | 0.98 | 1.00 | 1.00 | 1.00 | 1.00 | 0.98 |
| rules + NER | new person names | 0.94 | 0.94 | 1.00 | 1.00 | 1.00 | 0.97 | 0.93 |
| rules + NER | new persons, firms and places | 0.92 | 0.81 | 0.96 | 1.00 | 1.00 | 0.93 | 0.91 |
| rules + NER | no diacritics | 0.98 | 0.98 | 0.99 | 1.00 | 0.97 | 0.99 | 0.95 |
| rules + NER | all lowercase | 0.98 | 0.90 | 0.95 | 1.00 | 1.00 | 0.98 | 0.94 |
| rules + NER | ALL CAPITALS | 0.97 | 0.90 | 0.93 | 1.00 | 0.99 | 0.99 | 0.79 |

## WikiANN, 300 Wikipedia sentences per language

Silver labels for persons, organizations and locations, and a different idea of an entity than this tool
has (countries, sports clubs, historical figures). Columns `person` to `all covered` count an entity that
any predicted span overlaps.

| lang | config | person | organization | location | all covered | right type | predictions on a gold entity | entities |
|---|---|---|---|---|---|---|---|---|
| lb | rules | 0.84 | 0.34 | 0.56 | 0.57 | 0.50 | 0.98 | 382 |
| lb | rules + fast NER | 0.95 | 0.55 | 0.76 | 0.75 | 0.65 | 0.95 | 382 |
| lb | rules + NER | 0.91 | 0.47 | 0.73 | 0.70 | 0.62 | 0.94 | 382 |
| de | rules | 0.72 | 0.27 | 0.44 | 0.48 | 0.40 | 0.97 | 398 |
| de | rules + fast NER | 0.93 | 0.58 | 0.81 | 0.78 | 0.68 | 0.95 | 398 |
| de | rules + NER | 0.95 | 0.64 | 0.78 | 0.79 | 0.71 | 0.93 | 398 |
| fr | rules | 0.80 | 0.39 | 0.46 | 0.55 | 0.41 | 0.99 | 399 |
| fr | rules + fast NER | 0.95 | 0.66 | 0.80 | 0.80 | 0.64 | 0.95 | 399 |
| fr | rules + NER | 0.96 | 0.64 | 0.79 | 0.80 | 0.69 | 0.97 | 399 |
| en | rules | 0.76 | 0.31 | 0.49 | 0.53 | 0.42 | 0.98 | 387 |
| en | rules + fast NER | 0.94 | 0.63 | 0.77 | 0.79 | 0.62 | 0.96 | 387 |
| en | rules + NER | 0.95 | 0.60 | 0.84 | 0.81 | 0.67 | 0.94 | 387 |

## ai4privacy pii-masking-300k, 120 synthetic documents per language

Share of labelled items that any predicted span overlaps, with the default config: dates that are no birth
dates and times of day are off, and sex and title have no detector on purpose. A different sample of the
same dataset served to develop the form-field and ID rules; these documents were not looked at.

| lang | rules | rules + fast NER | rules + NER |
|---|---|---|---|
| de | 0.64 | 0.68 | 0.72 |
| en | 0.64 | 0.67 | 0.72 |
| fr | 0.61 | 0.66 | 0.70 |

| label | rules | rules + fast NER | rules + NER | items |
|---|---|---|---|---|
| BOD | 0.68 | 0.69 | 0.71 | 122 |
| BUILDING | 0.78 | 0.78 | 0.84 | 68 |
| CITY | 0.78 | 0.86 | 0.92 | 74 |
| COUNTRY | 0.69 | 0.74 | 0.71 | 68 |
| DATE | 0.22 | 0.23 | 0.22 | 77 |
| DRIVERLICENSE | 0.79 | 0.82 | 0.81 | 122 |
| EMAIL | 1.00 | 1.00 | 1.00 | 120 |
| GEOCOORD | 0.86 | 1.00 | 0.86 | 14 |
| GIVENNAME1 | 0.70 | 0.80 | 0.95 | 86 |
| GIVENNAME2 | 0.70 | 0.78 | 0.93 | 27 |
| IDCARD | 0.89 | 0.90 | 0.91 | 138 |
| IP | 1.00 | 1.00 | 1.00 | 92 |
| LASTNAME1 | 0.50 | 0.70 | 0.89 | 107 |
| LASTNAME2 | 0.65 | 0.77 | 0.88 | 26 |
| LASTNAME3 | 0.64 | 0.73 | 0.82 | 11 |
| PASS | 0.72 | 0.76 | 0.77 | 82 |
| PASSPORT | 0.91 | 0.92 | 0.91 | 150 |
| POSTCODE | 0.72 | 0.72 | 0.77 | 65 |
| SECADDRESS | 0.58 | 0.58 | 0.79 | 24 |
| SEX | 0.02 | 0.14 | 0.10 | 96 |
| SOCIALNUMBER | 0.78 | 0.78 | 0.78 | 111 |
| STATE | 0.73 | 0.81 | 0.76 | 67 |
| STREET | 0.75 | 0.81 | 0.93 | 68 |
| TEL | 0.96 | 0.96 | 0.96 | 76 |
| TIME | 0.02 | 0.02 | 0.03 | 181 |
| TITLE | 0.10 | 0.22 | 0.21 | 105 |
| USERNAME | 0.47 | 0.56 | 0.81 | 129 |
