from functools import lru_cache

from tagging.lang import guess

# spaCy has no Luxembourgish pipeline; its multilingual model was trained on Wikipedia in many
# languages, Luxembourgish among them, and also serves texts whose language is unclear.
MULTILINGUAL = "xx_ent_wiki_sm"
# The small pipelines: on the development data the medium and large ones found no more names once
# the rules and filters of this package had run, and they are ten to forty times the download.
MODELS = {"en": "en_core_web_sm", "fr": "fr_core_news_sm", "de": "de_core_news_sm", "lb": MULTILINGUAL}
TYPES = {"PER": "person", "PERSON": "person", "ORG": "organization", "LOC": "location", "GPE": "location"}
# spaCy gives no confidence per entity.
SCORE = 0.7


@lru_cache(maxsize=8)
def load(name):
    import spacy

    nlp = spacy.load(name)
    # Tagger, parser and lemmatizer are not needed for entities and cost most of the time.
    nlp.select_pipes(enable=[p for p in nlp.pipe_names if p in ("tok2vec", "transformer", "ner")])
    return nlp


# Character offsets, like `ner.predict_many`. Each text goes to the model of its language, guessed
# from its function words unless `language` is given.
def predict_many(texts, models=None, language=None, batch_size=64):
    models = MODELS | (models or {})
    by_model = {}
    for i, text in enumerate(texts):
        name = models.get(language or guess(text)) or models.get(None) or MULTILINGUAL
        by_model.setdefault(name, []).append(i)
    spans = [[] for _ in texts]
    for name, ids in by_model.items():
        for i, doc in zip(ids, load(name).pipe((texts[i] for i in ids), batch_size=batch_size)):
            spans[i] = [
                {
                    "type": TYPES[e.label_],
                    "start": e.start_char,
                    "end": e.end_char,
                    "text": e.text,
                    "score": SCORE,
                    "source": "ner",
                }
                for e in doc.ents
                if e.label_ in TYPES and e.text.strip()
            ]
    return spans
