from tagging.lang import guess


def test_guesses_the_four_languages():
    for text, lang in [
        ("De Jhemp wunnt elo bei der Bomi zu Léiler.", "lb"),
        ("Ech hunn den Här Kieffer gëschter gesinn.", "lb"),
        ("Der Bruder von Charel arbeitet nicht mehr bei der Bank.", "de"),
        ("Le frère de Charel travaille chez Cactus depuis des années.", "fr"),
        ("Charel's brother has worked for the bank since 2019.", "en"),
    ]:
        assert guess(text) == lang, text


def test_no_guess_without_function_words():
    for text in ["", "Jhemp Kieffer", "LU28 0019 4006 4475 0000", "BE07718341704718", "Trier, Metz, Arlon"]:
        assert guess(text) is None, text
