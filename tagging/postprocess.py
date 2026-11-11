import re
from functools import lru_cache
from pathlib import Path

from tagging.lang import guess

ARTICLES = set("d' d’ l' l’ de den dem der die das des le la les the".split())
TITLES = set(
    "här madame monsieur herr herrn frau dokter doktesch docteur docteure dr dr. mr mr. mrs mrs. ms ms. mme mme. "
    "mlle m. prof prof. professor sir miss".split()
)
DETERMINERS = ARTICLES | set(
    "d l a an e en eng engem enger ein eine einen einem einer un une my your his her our their this that these "
    "those meng mengem menger mäi mäin däin deng dengem säin säi seng sengem senger hir hire hiren hirem hirer eis "
    "eise eisen eisem eiser äre ären ärem dësen dës dëst dësem dëser mein meine meinem meiner meinen dein deine "
    "sein seine seinem seiner seinen ihr ihre ihrem ihrer ihren unser unsere unserem unserer dieser diese dieses "
    "diesem diesen ma mon mes ta ton sa son ses notre nos votre vos leur leurs ce cet cette ces du au aux".split()
)
# Words that may sit between role nouns without making the span a name: "the Baker and the Butcher".
FILLERS = set(
    "and und et an a & of von vum vun du new old big small nei neie al grouss kleng neu neue neuen alt alte groß "
    "große klein nouveau nouvelle vieux vieille grand grande petit petite".split()
)
ROLES = set(
    # Luxembourgish
    "mamm mamma papp pappa bopa bomi boma monni tatta noper nopesch direkter direktesch proff joffer schwëster "
    "brudder jong meedchen kand kanner fra mann frënd frëndin chef cheffin dokter doktesch infirmière infirmier "
    "elteren niess neveu cousin kusinn buergermeeschter apdikter bäcker metzler bauer enkel schwoer schwéiesch "
    "nonn spiller trainer patient patientin "
    # German
    "mutter mama vater papa opa oma großvater großmutter grossvater grossmutter onkel tante nachbar nachbarin "
    "direktor direktorin lehrer lehrerin chefin bruder schwester sohn tochter kind kinder eltern cousine nichte "
    "neffe arzt ärztin doktor freund freundin sekretärin sekretär apotheker metzger bürgermeister "
    "krankenschwester kollege kollegin junge mädchen spieler spielerin patienten "
    # French
    "mère maman père grand-mère grand-père mémé pépé oncle voisin voisine directeur directrice professeur "
    "maîtresse maître frère sœur soeur fils fille enfant enfants parents médecin docteur patron patronne "
    "secrétaire maire nièce collègue ami amie mari femme boulanger boucher pharmacien entraîneur garçon joueur "
    "joueurs patiente "
    # English
    "mother mom mum father dad grandma grandpa grandmother grandfather uncle aunt auntie neighbour neighbor "
    "teacher boss brother sister son daughter child children niece nephew doctor nurse secretary farmer mayor "
    "colleague friend husband wife baker butcher pharmacist coach boy girl man woman player players".split()
)
PRONOUNS = set(
    "i me we us you he him she her they them ech mir mer du hien hatt si dir ich wir er sie es je nous vous il "
    "elle ils elles on moi toi lui eux".split()
)
# Generic places and institutions; descriptive labels still tag them as organizations when they are
# capitalized ("the new School", "la Cuisine", "Garage", "Kino").
GENERIC = set(
    "school schoul schule école ecole kitchen kichen küche cuisine barn scheier scheune grange garage kino cinema "
    "cinéma hospital spidol krankenhaus hôpital station gare bahnhof poste church kierch kirche église castle "
    "schlass burg château park parc office büro bureau room changing sall saal salle town hall gemeng gemeinde "
    "commune bakery bäckerei boulangerie pharmacy apdikt apotheke pharmacie restaurant café cafe shop geschäft "
    "magasin river baach fluss rivière garden gaart garten jardin library bibliothéik bibliothek bibliothèque "
    "university universitéit universität université police polizei court gericht tribunal".split()
)
NOT_NAMES = ROLES | PRONOUNS | GENERIC | {t.rstrip(".") for t in TITLES}
LEADING = re.compile(r"([dDlL]['’])|(\S+)\s+")
CONJUNCTION = re.compile(r"\s+(?:an|a|und|et|and|&)\s+", re.IGNORECASE)
WORDS = re.compile(r"[^\s'’]+")
# What follows the owner in a possessive: "Charel's brother", "dem Charel säi Brudder".
POSSESSIVE = re.compile(r"['’]s\b|\s+(?:säin?|seng(?:em|er)?|hir(?:en?|em|er)?)\s", re.IGNORECASE)
# A place joined to a name by one of these is not part of the name: "ADEM zu Welschent".
LOCATIVE = re.compile(r"\s+(?:zu|à|in|at|bei|near|no|nach|op)\s+$", re.IGNORECASE)
COMMON_WORDS = Path(__file__).with_name("common_words.txt")
NUMBER = re.compile(r"[\d.,:/\-]+")
# Lowercase words that can stand inside a name: "Banque de Luxembourg", "Esch an der Alzette".
PARTICLES = set(
    "de du des d la le les l et en sur sous aux au à of the and for in on von vom van der den dem zu zur und am im "
    "an op vun a y e da do dos di del della".split()
)


def strip_titles(text, start, end):
    while m := LEADING.match(text, start, end):
        word = (m.group(1) or m.group(2)).lower()
        if word not in ARTICLES and word not in TITLES:
            break
        start = m.end()
    return start, end


def not_a_name(span):
    words = [w.strip(".,;:!?") for w in WORDS.findall(span.lower())]
    return all(w in NOT_NAMES for w in words if w not in DETERMINERS and w not in FILLERS)


def split_coordinated(text, start, end):
    cuts = [(m.start() + start, m.end() + start) for m in CONJUNCTION.finditer(text[start:end])]
    bounds = [start] + [x for c in cuts for x in c] + [end]
    return list(zip(bounds[::2], bounds[1::2]))


# The owner of a possessive is the name; what is owned is not: "der Gréit hire Brudder".
def cut_possessive(text, start, end):
    m = POSSESSIVE.search(text, start, end)
    return (start, m.start()) if m and m.start() > start else (start, end)


def trim(text, start, end):
    while start < end and not text[start].isalnum():
        start += 1
    while end > start and not text[end - 1].isalnum():
        end -= 1
    return start, end


# A model sometimes runs on into the sentence: "Musée mécht op Feierdeeg", "Schéifer huet d'Schof".
# The span is cut before the first lowercase everyday word that is no name particle.
def cut_at_sentence_word(text, start, end):
    plain, _ = common_words()
    for m in WORDS.finditer(text, start, end):
        w = m.group().strip('.,;:!?()"«»')
        if m.start() > start and w.islower() and w in plain and w not in PARTICLES and w not in FILLERS:
            return start, m.start()
    return start, end


# Spans here use character offsets; they are converted to bytes after post-processing.
def postprocess(text, spans):
    out = []
    for s in spans:
        parts = split_coordinated(text, s["start"], s["end"]) if s["type"] == "person" else [(s["start"], s["end"])]
        for a, b in parts:
            if s["type"] == "person":
                a, b = cut_possessive(text, a, b)
                a, b = strip_titles(text, a, b)
            if s["type"] != "address":
                a, b = cut_at_sentence_word(text, a, b)
            a, b = trim(text, a, b)
            if a < b and any(c.isalpha() for c in text[a:b]) and not not_a_name(text[a:b]):
                out.append({**s, "start": a, "end": b, "text": text[a:b]})
    return out


# Everyday words from Wikipedia text; "~" marks those that are also frequent names ("rose", "baker").
@lru_cache(maxsize=1)
def common_words():
    plain, marked = set(), set()
    for line in COMMON_WORDS.read_text(encoding="utf-8").splitlines():
        if not line.startswith("#"):
            (marked if line.startswith("~") else plain).add(line.lstrip("~"))
    return frozenset(plain), frozenset(marked)


def content_words(span):
    words = [w.strip('.,;:!?()"«»') for w in WORDS.findall(span.lower())]
    # Numbers say nothing either way: "Invoice 4471" is as much an everyday phrase as "Invoice".
    return [w for w in words if w and w not in DETERMINERS and w not in FILLERS and not NUMBER.fullmatch(w)]


# A span made of everyday words only ("le Plombier", "die Feuerwehr", "the Post Office") is not a
# name. Two things speak for a name all the same, and both only count without a determiner in front:
# a word that is also a frequent name ("Baker", "Berger"), and several capitalized words in a row
# ("National Water Well Association", "Motor Coach Industries").
def everyday(text, start, end):
    plain, marked = common_words()
    span = text[start:end]
    words = content_words(span)
    known = [w in plain or w in marked or all(p in plain for p in w.split("-")) for w in words]
    if not words or not all(known):
        return False
    before = WORDS.findall(text[:start].lower())[-1:]
    first = WORDS.findall(span.lower())[:1]
    if set(before + first) & DETERMINERS:
        return True
    # German and Luxembourgish capitalize every noun, so a row of capitals proves nothing there.
    title_case = [w[:1].isupper() for w in WORDS.findall(span) if w.lower() not in FILLERS]
    several = len(title_case) >= 2 and all(title_case) and guess(text) not in ("de", "lb")
    return not (all(w in marked for w in words) or several)


# Byte offsets, like the list hits in `support`, which are all hits whatever their score. A model
# span survives if it is not made of everyday words, or if a word list knows a name in it. A name
# that is itself an everyday word (score below 0.45, `Romaine`) only vouches for a span it covers.
def drop_everyday(text, spans, support):
    raw = text.encode()
    out = []
    for s in spans:
        supported = any(
            s["start"] < k["end"] and k["start"] < s["end"] and k["source"] == "gazetteer"
            for k in support
            if k["score"] > 0.45 or (k["start"] <= s["start"] and s["end"] <= k["end"])
        )
        start, end = len(raw[: s["start"]].decode()), len(raw[: s["end"]].decode())
        if s["type"] == "address" or supported or not everyday(text, start, end):
            out.append(s)
    return out
