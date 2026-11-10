"""Builds tagging/common_words.txt: words that are everyday vocabulary in English, French, German or
Luxembourgish. The name lists use it to tell `Rose` the flower from `Rose` the person, and the NER
post-processing uses it to drop spans such as `Post Office` or `Kindergarten`.

Needs `pip install huggingface_hub pyarrow` and downloads about 1 GB of Wikipedia text. The word
counts are cached in core/data/.cache/, so a second run only applies the thresholds.
"""

import csv
import pickle
import re
from collections import Counter
from pathlib import Path

HERE = Path(__file__).parent
OUT = HERE.parents[1] / "tagging" / "common_words.txt"
DATASET = "wikimedia/wikipedia"
SHARDS = {
    "lb": "20231101.lb/train-00000-of-00001.parquet",
    "de": "20231101.de/train-00010-of-00020.parquet",
    "fr": "20231101.fr/train-00009-of-00017.parquet",
    "en": "20231101.en/train-00020-of-00041.parquet",
}
MAX_CHARS = 150_000_000
TOKEN = re.compile(r"[^\W\d_]+(?:-[^\W\d_]+)*|[.!?:\n]")
SENTENCE_END = ".!?:\n"
# German and Luxembourgish capitalize every noun, so lowercase use says nothing about them. A noun
# shows itself by the determiner in front of it. Definite articles are counted separately: they
# also stand before proper nouns (`die NATO`, `der Irak`, Luxembourgish `de Jhemp`).
DETERMINERS = {
    "de": "ein eine einen einem einer eines kein keine keinen mein meine sein seine ihr ihre unser unsere dieser "
    "diese dieses jeder jede jedes",
    "lb": "e en eng engem enger kee keen keng meng mäin mäi säin säi seng hir hiren eis eisen all vill zwee zwou "
    "dräi dësen dës dëst",
}
DEFINITE = {"de": "der die das den dem des zum zur im am vom beim", "lb": "d de den dem der"}

# Lowercase use: at least MIN_LOW times and LOW_RATIO times as often as capitalized mid-sentence.
MIN_LOW, LOW_RATIO = 10, 0.3
MIN_DET, DET_RATIO = 4, 0.04
MIN_DEFINITE, DEFINITE_RATIO = 5, {"de": 0.1, "lb": 0.15}
# A word this much more frequent in lowercase is an everyday word even if people are named so (`will`).
CLEARLY_COMMON = 2
# Thresholds above which a word counts as a name: people with that given or family name in raw/.
NAME_GIVEN, NAME_FAMILY, NAME_LU = 15, 40, 2


def capitalized(w):
    return w[:1].upper() + w[1:]


def count(lang):
    cache = HERE / ".cache" / f"counts_{lang}.pkl"
    if cache.exists():
        return pickle.loads(cache.read_bytes())
    import pyarrow.parquet as pq
    from huggingface_hub import hf_hub_download

    path = hf_hub_download(DATASET, SHARDS[lang], repo_type="dataset")
    determiners = set(DETERMINERS.get(lang, "").split())
    articles = set(DEFINITE.get(lang, "").split())
    low, cap, det, definite = Counter(), Counter(), Counter(), Counter()
    chars = 0
    for batch in pq.ParquetFile(path).iter_batches(batch_size=2000, columns=["text"]):
        for text in batch.column(0).to_pylist():
            chars += len(text)
            prev = "."
            for t in TOKEN.findall(text):
                if t[0].isupper():
                    # Sentence-initial words are capitalized whatever they are.
                    if prev not in SENTENCE_END:
                        cap[t] += 1
                        det[t] += prev.lower() in determiners
                        definite[t] += prev.lower() in articles
                elif t[0].islower():
                    low[t] += 1
                prev = t
        if chars > MAX_CHARS:
            break
    # Rare words only add noise and make the cache large.
    counts = tuple(Counter({w: n for w, n in c.items() if n >= 3}) for c in (low, cap, det, definite))
    cache.parent.mkdir(exist_ok=True)
    cache.write_bytes(pickle.dumps(counts))
    return counts


def names():
    out = {}
    for kind in ("given", "family"):
        with open(HERE / "raw" / f"{kind}_names.csv", encoding="utf-8") as f:
            out[kind] = {r["name"]: (int(r["people"]), int(r["people_lu"])) for r in csv.DictReader(f)}
    return out["given"], out["family"]


# One-word names in the hand-curated lists and the raw Wikidata labels, so that `d'ADEM`,
# `de Charel` or `en Allemagne` does not turn a name into a noun.
def listed_names():
    names = set()
    for path in [*HERE.glob("lu_*.txt"), *(HERE / "raw").glob("*.csv")]:
        if not path.name.endswith("_names.csv"):
            names |= set(re.split(r"[|,~\n]", path.read_text(encoding="utf-8")))
    return {n for n in names if n and " " not in n}


def main():
    given, family = names()
    listed = listed_names()

    def name_like(w):
        g, g_lu = given.get(capitalized(w), (0, 0))
        f, f_lu = family.get(capitalized(w), (0, 0))
        return g >= NAME_GIVEN or f >= NAME_FAMILY or g_lu >= NAME_LU or f_lu >= NAME_LU

    counts = {lang: count(lang) for lang in SHARDS}
    # `canada` or `nato` turn up in lowercase in every corpus (file names, quoted foreign titles).
    # Summed over the four languages, a real everyday word is still mostly lowercase.
    low_all, cap_all = Counter(), Counter()
    for low, cap, _, _ in counts.values():
        low_all.update(low)
        cap_all.update(cap)

    plain, marked = set(), set()
    for lang, (low, cap, det, definite) in counts.items():
        found = {}
        for w, n in low.items():
            c = cap[capitalized(w)]
            mostly_lower = low_all[w] >= LOW_RATIO * cap_all[capitalized(w)]
            if n >= MIN_LOW and len(w) >= 2 and n >= LOW_RATIO * c and mostly_lower:
                found[w] = n / (c + 1)
        for w, n in det.items():
            if n >= MIN_DET and n >= DET_RATIO * cap[w] and w not in listed:
                found.setdefault(w.lower(), 0)
        for w, n in definite.items():
            # German keeps the article for the job (`der Koch`), Luxembourgish also for the person.
            person = lang == "lb" and (w in given or w in family)
            ratio = DEFINITE_RATIO.get(lang, 1)
            if n >= MIN_DEFINITE and n >= ratio * cap[w] and not (person or w in listed or w.isupper()):
                found.setdefault(w.lower(), 0)
        for w, ratio in found.items():
            (plain if ratio >= CLEARLY_COMMON or not name_like(w) else marked).add(w)
        print(f"{lang}: {len(found)} words")
    marked -= plain

    header = [
        "# Everyday words of English, French, German and Luxembourgish, from Wikipedia text (CC BY-SA).",
        "# Generated by core/data/build_common_words.py. '~' marks words that are also frequent names.",
    ]
    lines = sorted(plain | {f"~{w}" for w in marked}, key=lambda w: w.lstrip("~"))
    OUT.write_text("\n".join(header + lines) + "\n", encoding="utf-8", newline="\n")
    print(f"wrote {len(plain)} words and {len(marked)} name-like words to {OUT}")


if __name__ == "__main__":
    main()
