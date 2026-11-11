# Over-redaction on everyday sentences

120 sentences, 30 per language, without any personal data. Every hit is wrong.

| config | en | fr | de | lb | total |
|---|---|---|---|---|---|
| rules | 1 | 0 | 0 | 1 | 2 |
| ner_only | 41 | 40 | 34 | 36 | 151 |
| rules_ner | 42 | 40 | 34 | 37 | 153 |
| rules_fast | 2 | 3 | 2 | 2 | 9 |
| rules_both | 4 | 4 | 2 | 2 | 12 |
| rules_ner_post_tuned | 3 | 3 | 0 | 1 | 7 |

## Remaining hits of `rules_ner_post_tuned`

| lang | type | text | score | source | sentence |
|---|---|---|---|---|---|
| en | person | Stranger | 0.57 | ner | A Stranger asked the Bus Driver for the Timetable. |
| en | person | Carpenter | 0.84 | ner | The Carpenter built a Wardrobe for the Guest Room. |
| en | person | Shepherd | 0.92 | ner | The Shepherd led the Sheep across the Meadow. |
| fr | location | la Réunion | 0.56 | ner | Le Comité a reporté la Réunion à l'Automne. |
| fr | organization | Nouveau-né à la Maison | 0.83 | ner | La Sage-femme a rendu visite au Nouveau-né à la Maison. |
| fr | person | Berger | 0.92 | ner | Le Berger a mené les Moutons à travers la Prairie. |
| lb | person | Klempner | 0.67 | ner | De Klempner huet d'Spullbecken am Buedzëmmer virum Mëtteg gefléckt. |
