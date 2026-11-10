"""Builds world_places.txt and world_organizations.txt from Wikidata: countries, well-known cities,
towns around Luxembourg and well-known companies, universities and international bodies."""

import csv
import io
import re
import sys
from pathlib import Path

from build_names import common_words, fetch

HERE = Path(__file__).parent
RAW = HERE / "raw"
LABELS = """
OPTIONAL{?item rdfs:label ?lb FILTER(lang(?lb)="lb")}
OPTIONAL{?item rdfs:label ?fr FILTER(lang(?fr)="fr")}
OPTIONAL{?item rdfs:label ?de FILTER(lang(?de)="de")}
OPTIONAL{?item rdfs:label ?en FILTER(lang(?en)="en")}
"""
# Sitelinks count the Wikipedia language editions with an article, a cheap measure of how well
# known something is.
CITY_TYPES = "wd:Q515 wd:Q1549591 wd:Q1637706 wd:Q5119 wd:Q174844 wd:Q200250 wd:Q208511"
PLACE_QUERIES = {
    "countries.csv": f"""SELECT ?item ?lb ?fr ?de ?en WHERE {{
?item wdt:P31 wd:Q3624078. FILTER NOT EXISTS {{?item wdt:P576 ?end}} {LABELS} }}""",
    "cities.csv": f"""SELECT DISTINCT ?item ?lb ?fr ?de ?en ?sl WHERE {{
VALUES ?t {{{CITY_TYPES}}} ?item wdt:P31 ?t; wikibase:sitelinks ?sl. FILTER(?sl >= 90) {LABELS} }}""",
    # Populated places within 110 km of Luxembourg City: Trier, Metz, Arlon, Thionville...
    "greater_region.csv": f"""SELECT DISTINCT ?item ?lb ?fr ?de ?en ?sl WHERE {{
SERVICE wikibase:around {{ ?item wdt:P625 ?loc. bd:serviceParam wikibase:center "Point(6.13 49.61)"^^geo:wktLiteral;
  wikibase:radius "110". }}
?item wdt:P1082 ?pop; wikibase:sitelinks ?sl. FILTER(?pop >= 8000 && ?sl >= 20)
FILTER NOT EXISTS {{ ?item wdt:P17 wd:Q32 }} {LABELS} }}""",
}
# Business, enterprise, public company, bank, airline, technology company, car maker, retail chain,
# conglomerate, electronics company, multinational, organization, nonprofit, international
# organization, intergovernmental organization, news media, academic and research institutions,
# university, video game developer, software and aerospace companies, political organizations,
# standards bodies and public universities.
ORG_TYPES = (
    "Q4830453 Q6881511 Q891723 Q22687 Q46970 Q18388277 Q786820 Q507619 Q219577 Q1589009 Q161726 Q43229 Q163740 "
    "Q484652 Q245065 Q1616075 Q2085381 Q3918 Q210167 Q1058914 Q783794 Q4671277 Q7210356 Q79913 Q15911314 Q902104 "
    "Q875538"
).split()
ORG_QUERY = """SELECT ?item ?lb ?fr ?de ?en ?sl WHERE {{
{{ SELECT ?item ?sl WHERE {{ ?item wdt:P31 wd:{type}; wikibase:sitelinks ?sl. FILTER(?sl >= 50) }} }} {labels} }}"""
# `organization` alone is too broad a type: towns and events carry it too.
GENERIC_ORG = "Q43229"

# Administrative areas around Luxembourg that nobody writes in an address.
NOT_PLACES = re.compile(r"arrondissement|district|community|kreis|verband|territory|province|bezirk", re.IGNORECASE)
# Names that are everyday words but too rare in Wikipedia text for common_words.txt to know them.
AMBIGUOUS = {
    "Essen", "Nice", "Bath", "Reading", "Split", "Mobile", "Tours", "Bari", "Male", "Meta", "Next", "Gap", "Mensa",
}  # fmt: skip
# Large employers in Luxembourg that fall below the sitelink threshold or have another type.
EXTRA_ORGS = [
    ["Deloitte"], ["KPMG"], ["PwC", "PricewaterhouseCoopers"], ["EY", "Ernst & Young"], ["Ferrero"],
    ["Goodyear"], ["DuPont"], ["PayPal"], ["Vodafone"], ["Clearstream"], ["Euroclear"], ["BNP Paribas"],
    ["Société Générale"], ["Commerzbank"], ["J.P. Morgan", "JPMorgan"], ["State Street"], ["Allianz"], ["AXA"],
    ["Lombard International"], ["European Investment Bank", "Banque européenne d'investissement", "EIB", "BEI"],
    ["Eurostat"], ["Cour de justice de l'Union européenne", "Court of Justice of the European Union", "CJUE"],
]  # fmt: skip


def read(name):
    return list(csv.DictReader(io.StringIO((RAW / name).read_text(encoding="utf-8"))))


def clean(x):
    x = re.sub(r"\s*\(.*\)$", "", x).strip()
    return x if len(x) >= 2 and x[0].isupper() and not re.search(r"\d", x) else ""


def labels(row, max_words):
    names = []
    for k in ("lb", "fr", "de", "en"):
        name = clean(row.get(k) or "")
        if name and len(name.split()) <= max_words and name not in names:
            names.append(name)
    return names


# One line per item with its spellings. A single-word spelling that is an everyday word goes on its
# own line: `?` if only a cue word in front of it makes it a name (`bei Orange`, `in Essen`), `~` if
# it is a frequent family name as well (`Florence`, `Jackson`).
def lines(items, plain, marked):
    out = set()
    for names in items:
        # An acronym only matches in capitals, which is evidence enough.
        single = [n for n in names if " " not in n and not n.isupper() and [n] not in EXTRA_ORGS]
        context = [n for n in single if n.lower() in plain or n in AMBIGUOUS]
        ambiguous = [n for n in single if n not in context and n.lower() in marked]
        clear = [n for n in names if n not in context + ambiguous]
        out |= {f"?{n}" for n in context} | {f"~{n}" for n in ambiguous}
        if clear:
            out.add("|".join(clear))
    return sorted(out, key=lambda x: x.lstrip("~?"))


def write(path, header, body):
    path.write_text("\n".join(header + body) + "\n", encoding="utf-8", newline="\n")
    print(f"wrote {len(body)} lines to {path.name}")


def main():
    if "--offline" not in sys.argv:
        for name, q in PLACE_QUERIES.items():
            (RAW / name).write_text(fetch(q), encoding="utf-8", newline="")
        rows, head = set(), ""
        for t in ORG_TYPES:
            head, *body = fetch(ORG_QUERY.format(type=t, labels=LABELS)).splitlines()
            rows |= {f"{line},{t}" for line in body}
        (RAW / "organizations.csv").write_text("\n".join([f"{head},type", *sorted(rows)]) + "\n", encoding="utf-8")

    plain, marked = common_words()
    luxembourg = (HERE / "lu_places.txt").read_text(encoding="utf-8")
    known = {n.lstrip("~") for line in luxembourg.splitlines() for n in line.split("|")}

    places = [labels(r, 3) for name in ("countries.csv", "cities.csv") for r in read(name)]
    places += [labels(r, 3) for r in read("greater_region.csv") if not NOT_PLACES.search(r["en"] + r["fr"] + r["de"])]
    places = [[n for n in names if n not in known] for names in places]
    write(
        HERE / "world_places.txt",
        [
            "# Countries, well-known cities and towns around Luxembourg (Luxembourgish|French|German|English labels).",
            "# Generated by build_world.py from Wikidata (CC0); raw query results are in raw/.",
            "# '~' and '?' mark names that are also common words; see README.md for their scores.",
        ],
        lines(places, plain, marked),
    )

    rows = read("organizations.csv")
    specific = {r["item"] for r in rows if r["type"] != GENERIC_ORG}
    place_names = {n for names in places for n in names} | known
    orgs = {r["item"]: labels(r, 5) for r in rows if r["item"] in specific}
    orgs = [[n for n in names if n not in place_names] for names in orgs.values()] + EXTRA_ORGS
    write(
        HERE / "world_organizations.txt",
        [
            "# Well-known companies, universities and international organizations, plus large employers in Luxembourg.",
            "# Generated by build_world.py from Wikidata (CC0); raw query results are in raw/.",
            "# All-caps acronyms only match with the same case. '~' and '?' mark names that are also common words.",
        ],
        lines(orgs, plain, marked),
    )


if __name__ == "__main__":
    main()
