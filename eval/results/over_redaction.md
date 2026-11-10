# Over-redaction on everyday sentences

120 sentences, 30 per language, without any personal data. Every hit is wrong.

| config | en | fr | de | lb | total |
|---|---|---|---|---|---|
| rules | 1 | 0 | 0 | 1 | 2 |
| ner_only | 41 | 40 | 34 | 36 | 151 |
| rules_ner | 42 | 40 | 34 | 37 | 153 |
| rules_ner_post_tuned | 3 | 2 | 0 | 9 | 14 |

## Remaining hits of `rules_ner_post_tuned`

| lang | type | text | score | source | sentence |
|---|---|---|---|---|---|
| en | person | Stranger | 0.57 | ner | A Stranger asked the Bus Driver for the Timetable. |
| en | person | Carpenter | 0.84 | ner | The Carpenter built a Wardrobe for the Guest Room. |
| en | person | Shepherd | 0.92 | ner | The Shepherd led the Sheep across the Meadow. |
| fr | organization | Nouveau-né à la Maison | 0.83 | ner | La Sage-femme a rendu visite au Nouveau-né à la Maison. |
| fr | person | Berger | 0.92 | ner | Le Berger a mené les Moutons à travers la Prairie. |
| lb | person | Klempner | 0.67 | ner | De Klempner huet d'Spullbecken am Buedzëmmer virum Mëtteg gefléckt. |
| lb | person | Floumaart kaf | 0.61 | ner | Mäi Cousin huet e Vëlo um Floumaart kaf. |
| lb | person | Déierendoktesch | 0.81 | ner | D'Déierendoktesch huet den Hond an d'Kanéngchen de Moien ënnersicht. |
| lb | location | Feierdeeg | 0.42 | ner | De Musée mécht op Feierdeeg méi fréi zou. |
| lb | person | Benevollen | 0.64 | ner | De President huet de Benevollen an dem Caissier Merci gesot. |
| lb | organization | D'Zänndoktesch | 0.82 | ner | D'Zänndoktesch ass bis Enn vum Mount an der Vakanz. |
| lb | person | Hiewan | 0.80 | ner | D'Hiewan huet de Puppelchen doheem besicht. |
| lb | person | Schéifer | 0.79 | ner | De Schéifer huet d'Schof iwwer d'Wiss gefouert. |
| lb | person | Coiffer | 0.80 | ner | De Coiffer nieft der Bank huet méindes zou. |
