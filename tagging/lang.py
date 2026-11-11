import re

# Frequent function words that set the four languages apart. Words that two of them share
# (`de`, `an`, `in`, `die`, `der`, `bei`, `un`, `en`) carry no weight and are left out.
WORDS = {
    "en": "the and is are of for with that this was has have from not you they his her will would there their what "
    "which when your my our been were",
    "fr": "le la les et est une pour dans que qui pas je nous vous il elle sur avec au aux ce cette ne se son sa ont "
    "sont par plus mais comme chez très être fait leur ses mon ma votre",
    "de": "das und ist nicht ich wir sie mit für auf von ein eine einen auch hat haben wird sind wie wenn dass sich "
    "zur wurde werden nur über unter ihr sein seine",
    "lb": "ass sinn huet hunn fir mat op vun och net ech hien hatt wéi wann dat déi eng awer ginn gëtt wëll haut muer "
    "vum sech säin seng hir eis dës dësen nëmmen iwwer ënner gouf zënter gëschter schafft wunnt kënnt elo",
}
WORDS = {lang: frozenset(words.split()) for lang, words in WORDS.items()}
# Letters and letter groups that one language uses far more than the others.
MARKS = {"lb": "ë|éi|äi|ue|d'[a-zäëé]", "de": "ß|ü|ö|sch", "fr": "ç|è|ê|à|œ|[ljnsc]'[aeiouyéh]"}
MARKS = {lang: re.compile(pattern) for lang, pattern in MARKS.items()}
# Letters only, so the `BE` of an IBAN is no word.
TOKEN = re.compile(r"\b[^\W\d_]+\b")


# Returns "en", "fr", "de" or "lb", or None when the text gives no evidence, as a list of names or
# numbers does. A function word counts as much as three letter groups.
def guess(text):
    lowered = text.lower()
    tokens = TOKEN.findall(lowered)
    scores = {
        lang: 3 * sum(t in WORDS[lang] for t in tokens) + (len(MARKS[lang].findall(lowered)) if lang in MARKS else 0)
        for lang in WORDS
    }
    best = max(scores, key=scores.get)
    return best if scores[best] >= 3 else None
