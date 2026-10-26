import argparse
import csv
import json
import random
import re
import unicodedata
from pathlib import Path

from faker import Faker

ROOT = Path(__file__).parent
RAW = ROOT.parent / "core" / "data" / "raw"
OUT = ROOT / "data" / "eval.jsonl"
DEV_OUT = ROOT / "data" / "dev.jsonl"
LANGS = ["en", "fr", "de", "lb"]
SEED = 13
DEV_SEED = 7
INSTANCES = 2

rng = random.Random(SEED)
Faker.seed(SEED)
fake = Faker(["en_GB", "fr_FR", "de_DE"])

SLOT_TYPES = {
    "p": "person",
    "loc": "location",
    "org": "organization",
    "addr": "address",
    "email": "email",
    "phone": "phone",
    "iban": "iban",
    "card": "credit_card",
    "nid": "national_id",
    "dob": "date_of_birth",
    "url": "url",
    "ip": "ip_address",
}

# Standard subset: common names, well-known organizations and places, every structured type.
STANDARD = {
    "contact": {
        "en": "{p1} can be reached at {email} or on {phone}.",
        "fr": "On peut joindre {p1} à l'adresse {email} ou au {phone}.",
        "de": "{p1} ist unter {email} oder {phone} erreichbar.",
        "lb": "{p1} ass ënner {email} oder um {phone} ze erreechen.",
    },
    "birth": {
        "en": "{p1} was born on {dob} in {loc}.",
        "fr": "{p1} (né(e) le {dob} à {loc}) a signé le contrat.",
        "de": "{p1} wurde am {dob} in {loc} geboren.",
        "lb": "{p1} ass den {dob} zu {loc} gebuer.",
    },
    "transfer": {
        "en": "Please transfer the deposit to {p1}, IBAN {iban}.",
        "fr": "Veuillez virer la caution à {p1}, IBAN {iban}.",
        "de": "Bitte überweisen Sie die Kaution an {p1}, IBAN {iban}.",
        "lb": "Iwwerweist d'Kautioun w.e.g. un {p1}, IBAN {iban}.",
    },
    "card": {
        "en": "{p1} paid the booking with card {card}.",
        "fr": "{p1} a payé la réservation avec la carte {card}.",
        "de": "{p1} hat die Buchung mit der Karte {card} bezahlt.",
        "lb": "{p1} huet d'Reservatioun mat der Kaart {card} bezuelt.",
    },
    "matricule": {
        "en": "Social security number of {p1}: {nid}.",
        "fr": "Matricule de {p1} : {nid}.",
        "de": "Sozialversicherungsnummer von {p1}: {nid}.",
        "lb": "D'Matrikelnummer fir {p1} ass {nid}.",
    },
    "job": {
        "en": "{p1} has worked for {org} in {loc} since 2019.",
        "fr": "{p1} travaille chez {org} à {loc} depuis 2019.",
        "de": "{p1} arbeitet seit 2019 bei {org} in {loc}.",
        "lb": "{p1} schafft zënter 2019 bei {org} zu {loc}.",
    },
    "address": {
        "en": "{p1} lives at {addr}, {loc}.",
        "fr": "{p1} habite {addr} à {loc}.",
        "de": "{p1} wohnt unter der Adresse {addr} in {loc}.",
        "lb": "{p1} wunnt op der Adress {addr} zu {loc}.",
    },
    "profile": {
        "en": "More about {p1} on {url}.",
        "fr": "Plus d'informations sur {p1} : {url}.",
        "de": "Mehr über {p1} unter {url}.",
        "lb": "Méi iwwer {p1} op {url}.",
    },
    "login": {
        "en": "Login to {email} from IP address {ip} was blocked.",
        "fr": "La connexion à {email} depuis l'adresse IP {ip} a été bloquée.",
        "de": "Die Anmeldung bei {email} von der IP-Adresse {ip} wurde blockiert.",
        "lb": "D'Umeldung op {email} vun der IP-Adress {ip} gouf blockéiert.",
    },
    "meeting": {
        "en": "{p1} met {p2} at the {org} offices on Monday.",
        "fr": "{p1} a rencontré {p2} dans les bureaux de {org} lundi.",
        "de": "{p1} hat {p2} am Montag in den Büros von {org} getroffen.",
        "lb": "{p1} huet de Méindeg {p2} an de Büroe vun {org} getraff.",
    },
    "appointment": {
        "en": "Appointment confirmed for {p1} in {loc}, call {phone} to cancel.",
        "fr": "Rendez-vous confirmé pour {p1} à {loc}, appelez le {phone} pour annuler.",
        "de": "Termin für {p1} in {loc} bestätigt, Absage unter {phone}.",
        "lb": "Den Termin fir {p1} zu {loc} ass confirméiert, rufft {phone} un fir ofzesoen.",
    },
    "invoice": {
        "en": "Send the invoice to {email}, accounting department of {org}.",
        "fr": "Envoyez la facture à {email}, service comptabilité de {org}.",
        "de": "Senden Sie die Rechnung an {email}, Buchhaltung von {org}.",
        "lb": "Schéckt d'Rechnung un {email}, Comptabilitéit vun {org}.",
    },
    "form": {
        "en": "Name: {p1}, date of birth: {dob}, place of birth: {loc}.",
        "fr": "Nom : {p1}, date de naissance : {dob}, lieu de naissance : {loc}.",
        "de": "Name: {p1}, Geburtsdatum: {dob}, Geburtsort: {loc}.",
        "lb": "Numm: {p1}, Gebuertsdatum: {dob}, Gebuertsplaz: {loc}.",
    },
}

# Hard subset: Luxembourgish short names, articles and titles, coordinated names, native place
# spellings, local organizations and less common formats. `@dat` and `@poss` pick the
# Luxembourgish dative article or the dative possessive ("dem Mett säi").
HARD = {
    "coordinated": {
        "en": "{p1} and {p2} drove to {loc}.",
        "fr": "{p1} et {p2} sont partis à {loc}.",
        "de": "{p1} und {p2} sind nach {loc} gefahren.",
        "lb": "{p1} an {p2} sinn op {loc} gefuer.",
    },
    "letter": {
        "en": "{t1} from {org} wrote us a letter.",
        "fr": "{t1} de {org} nous a écrit une lettre.",
        "de": "{t1} von {org} hat uns einen Brief geschrieben.",
        "lb": "{t1} vun {org} huet eis e Bréif geschriwwen.",
    },
    "family": {
        "en": "My mother and {p1} are coming to {loc} on Sunday.",
        "fr": "Ma mère et {p1} viennent à {loc} dimanche.",
        "de": "Meine Mutter und {p1} kommen am Sonntag nach {loc}.",
        "lb": "Meng Mamm an {p1} kommen e Sonndeg op {loc}.",
    },
    "workplace": {
        "en": "{p1} has worked at {org} in {loc} for years.",
        "fr": "{p1} travaille depuis des années chez {org} à {loc}.",
        "de": "{p1} arbeitet seit Jahren bei {org} in {loc}.",
        "lb": "{p1} schafft zënter Joren bei {org} zu {loc}.",
    },
    "hotline": {
        "en": "For questions: {t1}, tel. {phone}.",
        "fr": "Pour toute question : {t1}, tél. {phone}.",
        "de": "Bei Fragen: {t1}, Tel. {phone}.",
        "lb": "Fir Froen: {t1}, Tel. {phone}.",
    },
    "chat": {
        "en": "{l1} and {l2} are coming tonight",
        "fr": "{l1} et {l2} viennent ce soir",
        "de": "{l1} und {l2} kommen heute abend",
        "lb": "{l1} an {l2} kommen haut den owend",
    },
    "born": {
        "en": "{p1}, b. {dob} in {loc}.",
        "fr": "{p1}, né(e) le {dob} à {loc}.",
        "de": "{p1}, geb. {dob} in {loc}.",
        "lb": "{p1}, gebuer {dob} zu {loc}.",
    },
    "account": {
        "en": "ID number {nid}, account {iban}.",
        "fr": "Matricule {nid}, compte {iban}.",
        "de": "Matrikel {nid}, Konto {iban}.",
        "lb": "Matricule {nid}, Kont {iban}.",
    },
    "separated": {
        "en": "{t1} and {t2} have separated.",
        "fr": "{t1} et {t2} se sont séparés.",
        "de": "{t1} und {t2} haben sich getrennt.",
        "lb": "{t1} an {t2} hu sech getrennt.",
    },
    "grandma": {
        "en": "{p1} now lives with grandma in {loc}.",
        "fr": "{p1} habite maintenant chez sa grand-mère à {loc}.",
        "de": "{p1} wohnt jetzt bei der Oma in {loc}.",
        "lb": "{p1} wunnt elo bei der Bomi zu {loc}.",
    },
    "text": {
        "en": "Email {p1} at {email} or text {phone}.",
        "fr": "Écris à {p1} sur {email} ou par SMS au {phone}.",
        "de": "Schreib {p1} an {email} oder eine SMS an {phone}.",
        "lb": "Schreif {p1@dat} op {email} oder eng SMS op {phone}.",
    },
    "brother": {
        "en": "{p1}'s brother works at {org}.",
        "fr": "Le frère de {p1} travaille chez {org}.",
        "de": "Der Bruder von {p1} arbeitet bei {org}.",
        "lb": "{p1@poss} Brudder schafft bei {org}.",
    },
    "card": {
        "en": "Card {card} belongs to {t1}.",
        "fr": "La carte {card} appartient à {t1}.",
        "de": "Die Karte {card} gehört {t1@dat}.",
        "lb": "D'Kaart {card} gehéiert {t1@dat}.",
    },
}

# Entity-free subset: capitalized nouns, kinship and role words, decoy numbers and place-like words.
ENTITY_FREE = [
    {
        "en": "Mother and Grandpa are going to the cinema today.",
        "fr": "La mère et le grand-père vont au cinéma aujourd'hui.",
        "de": "Die Mutter und der Opa gehen heute ins Kino.",
        "lb": "D'Mamm an de Bopa ginn haut an de Kino.",
    },
    {
        "en": "The Neighbour took his car to the Garage.",
        "fr": "Le voisin a amené sa voiture au garage.",
        "de": "Der Nachbar hat sein Auto in die Garage gebracht.",
        "lb": "Den Noper huet säin Auto an d'Garage bruecht.",
    },
    {
        "en": "The Director and the Teacher are talking to the parents.",
        "fr": "Le directeur et le professeur parlent avec les parents.",
        "de": "Der Direktor und der Lehrer sprechen mit den Eltern.",
        "lb": "De Direkter an de Proff schwätzen mat den Elteren.",
    },
    {
        "en": "A mosquito stung Grandma on the arm.",
        "fr": "Un moustique a piqué la grand-mère au bras.",
        "de": "Eine Mücke hat die Oma in den Arm gestochen.",
        "lb": "Eng Méck huet d'Bomi an den Aarm gestach.",
    },
    {
        "en": "We climbed the Mountain on Sunday.",
        "fr": "Nous sommes montés sur la Montagne dimanche.",
        "de": "Wir sind am Sonntag auf den Berg gestiegen.",
        "lb": "Mir sinn e Sonndeg op de Bierg gaang.",
    },
    {
        "en": "The letter is at the Post Office, the parcel at the Station.",
        "fr": "La lettre est à la Poste, le colis à la Gare.",
        "de": "Der Brief liegt auf der Post, das Paket am Bahnhof.",
        "lb": "De Bréif läit op der Post, de Pak an der Gare.",
    },
    {
        "en": "Auntie and Uncle are coming at Christmas.",
        "fr": "La Tante et l'Oncle viennent à Noël.",
        "de": "Die Tante und der Onkel kommen an Weihnachten.",
        "lb": "D'Tatta an de Monni kommen op Chrëschtdag.",
    },
    {
        "en": "My Sister is a Nurse at the Hospital.",
        "fr": "Ma Sœur est infirmière à l'Hôpital.",
        "de": "Meine Schwester ist Krankenschwester im Krankenhaus.",
        "lb": "Meng Schwëster ass Infirmière am Spidol.",
    },
    {
        "en": "Order 2023-000123 costs €1,234.56 and ships on Monday.",
        "fr": "La commande 2023-000123 coûte 1 234,56 € et sera livrée lundi.",
        "de": "Bestellung 2023-000123 kostet 1 234,56 € und wird am Montag geliefert.",
        "lb": "D'Bestellung 2023-000123 kascht 1 234,56 € a gëtt e Méindeg geliwwert.",
    },
    {
        "en": "Software Version 4.2.1.7 runs on the Server in the Office.",
        "fr": "Le Logiciel version 4.2.1.7 tourne sur le Serveur du Bureau.",
        "de": "Die Software Version 4.2.1.7 läuft auf dem Server im Büro.",
        "lb": "D'Software Versioun 4.2.1.7 leeft um Server am Büro.",
    },
    {
        "en": "Father and Son play Football in the Garden.",
        "fr": "Le Père et le Fils jouent au football dans le Jardin.",
        "de": "Der Vater und der Sohn spielen im Garten Fußball.",
        "lb": "De Papp an de Jong spillen am Gaart Fussball.",
    },
    {
        "en": "The Council repaired the Streets after the Winter.",
        "fr": "La Commune a refait les Rues après l'hiver.",
        "de": "Die Gemeinde hat die Straßen nach dem Winter erneuert.",
        "lb": "D'Gemeng huet d'Stroossen nom Wanter nei gemaach.",
    },
    {
        "en": "The Nurse and the Doctor were on duty all day.",
        "fr": "L'Infirmière et le Médecin ont été de service toute la journée.",
        "de": "Die Krankenschwester und der Arzt waren den ganzen Tag im Dienst.",
        "lb": "D'Infirmière an den Dokter waren de ganzen Dag am Déngscht.",
    },
    {
        "en": "The Meeting is on 14/11/2024 at 10 o'clock in Room 3.",
        "fr": "La Réunion a lieu le 14/11/2024 à 10 heures en Salle 3.",
        "de": "Die Sitzung ist am 14.11.2024 um 10 Uhr in Saal 3.",
        "lb": "D'Reunioun ass den 14.11.2024 um 10 Auer am Sall 3.",
    },
    {
        "en": "The Boss and the Secretary are at a Conference.",
        "fr": "Le Chef et la Secrétaire sont en Conférence.",
        "de": "Der Chef und die Sekretärin sind auf einer Konferenz.",
        "lb": "De Chef an d'Sekretärin sinn op enger Konferenz.",
    },
    {
        "en": "My Cousin and my Niece live in the Countryside.",
        "fr": "Mon Cousin et ma Nièce habitent à la Campagne.",
        "de": "Mein Cousin und meine Nichte wohnen auf dem Land.",
        "lb": "Mäi Cousin an d'Niess wunnen um Land.",
    },
    {
        "en": "The Farmer sold his Wine at the Winery.",
        "fr": "Le Paysan a vendu son Vin à la Cave.",
        "de": "Der Bauer hat seinen Wein in der Kellerei verkauft.",
        "lb": "De Bauer huet säi Wäin an der Kellerei verkaaft.",
    },
    {
        "en": "Breakfast was Mett with onions.",
        "fr": "Au petit-déjeuner, il y avait du Mett aux oignons.",
        "de": "Zum Frühstück gab es Mett mit Zwiebeln.",
        "lb": "Mir hunn Moies Mett mat Ënnen giess.",
    },
    {
        "en": "The Train into Town was twenty minutes late.",
        "fr": "Le Train pour la Ville avait vingt minutes de retard.",
        "de": "Der Zug in die Stadt hatte zwanzig Minuten Verspätung.",
        "lb": "Den Zuch op d'Stad hat zwanzeg Minutte Verspéidung.",
    },
    {
        "en": "The Children sang with the Teacher at Nursery.",
        "fr": "Les Enfants ont chanté avec la Maîtresse à la Maternelle.",
        "de": "Die Kinder haben im Kindergarten mit der Lehrerin gesungen.",
        "lb": "D'Kanner hunn am Kannergaart mat der Joffer gesongen.",
    },
]

# Separate entity-free sentences for the development split, so false positives can be tuned
# without touching the evaluation sentences.
ENTITY_FREE_DEV = [
    {
        "en": "Grandma and the Neighbour drink coffee in the Kitchen.",
        "fr": "La Grand-mère et la Voisine boivent un café dans la Cuisine.",
        "de": "Die Oma und die Nachbarin trinken Kaffee in der Küche.",
        "lb": "D'Bomi an d'Nopesch drénken e Kaffi an der Kichen.",
    },
    {
        "en": "The Mayor opened the new School next to the Church.",
        "fr": "Le Maire a inauguré la nouvelle École à côté de l'Église.",
        "de": "Der Bürgermeister hat die neue Schule neben der Kirche eröffnet.",
        "lb": "De Buergermeeschter huet déi nei Schoul nieft der Kierch opgemaach.",
    },
    {
        "en": "My Father and my Uncle repaired the roof of the Barn.",
        "fr": "Mon Père et mon Oncle ont réparé le toit de la Grange.",
        "de": "Mein Vater und mein Onkel haben das Dach der Scheune repariert.",
        "lb": "Mäi Papp an de Monni hunn den Daach vun der Scheier gefléckt.",
    },
    {
        "en": "Invoice 4471 for 89,90 € is due within 30 days.",
        "fr": "La facture 4471 de 89,90 € est payable sous 30 jours.",
        "de": "Die Rechnung 4471 über 89,90 € ist innerhalb von 30 Tagen fällig.",
        "lb": "D'Rechnung 4471 iwwer 89,90 € ass bannent 30 Deeg ze bezuelen.",
    },
    {
        "en": "The Boy and the Girl are playing in the Park.",
        "fr": "Le Garçon et la Fille jouent dans le Parc.",
        "de": "Der Junge und das Mädchen spielen im Park.",
        "lb": "De Jong an d'Meedchen spillen am Park.",
    },
    {
        "en": "The Pharmacist gave the Patient new tablets.",
        "fr": "Le Pharmacien a donné de nouveaux comprimés au Patient.",
        "de": "Der Apotheker hat dem Patienten neue Tabletten gegeben.",
        "lb": "Den Apdikter huet dem Patient nei Pëllen ginn.",
    },
    {
        "en": "We walked along the River up to the Castle.",
        "fr": "Nous avons marché le long de la Rivière jusqu'au Château.",
        "de": "Wir sind am Fluss entlang bis zur Burg gelaufen.",
        "lb": "Mir sinn d'Baach laanscht bis bei d'Schlass gaang.",
    },
    {
        "en": "The Coach and the Players celebrated in the Changing Room.",
        "fr": "L'Entraîneur et les Joueurs ont fêté dans le Vestiaire.",
        "de": "Der Trainer und die Spieler haben in der Kabine gefeiert.",
        "lb": "Den Trainer an d'Spiller hunn an der Kabinn gefeiert.",
    },
    {
        "en": "Update 2.3.0.1 fixes the Printer driver on the Laptop.",
        "fr": "La mise à jour 2.3.0.1 corrige le pilote de l'Imprimante.",
        "de": "Das Update 2.3.0.1 behebt den Druckertreiber auf dem Laptop.",
        "lb": "Den Update 2.3.0.1 flécken den Drécker um Laptop.",
    },
    {
        "en": "The Baker and the Butcher close early on Saturday.",
        "fr": "Le Boulanger et le Boucher ferment tôt le samedi.",
        "de": "Der Bäcker und der Metzger schließen am Samstag früh.",
        "lb": "De Bäcker an de Metzler maachen e Samschdeg fréi zou.",
    },
]

STANDARD_PLACES = [
    {"en": "Luxembourg", "fr": "Luxembourg", "de": "Luxemburg", "lb": "Lëtzebuerg"},
    {"en": "Esch-sur-Alzette", "fr": "Esch-sur-Alzette", "de": "Esch an der Alzette", "lb": "Esch-Uelzecht"},
    {"en": "Differdange", "fr": "Differdange", "de": "Differdingen", "lb": "Déifferdeng"},
    {"en": "Dudelange", "fr": "Dudelange", "de": "Düdelingen", "lb": "Diddeleng"},
    {"en": "Brussels", "fr": "Bruxelles", "de": "Brüssel", "lb": "Bréissel"},
    {"en": "Paris", "fr": "Paris", "de": "Paris", "lb": "Paräis"},
    {"en": "Trier", "fr": "Trèves", "de": "Trier", "lb": "Tréier"},
    {"en": "Metz", "fr": "Metz", "de": "Metz", "lb": "Metz"},
    {"en": "Berlin", "fr": "Berlin", "de": "Berlin", "lb": "Berlin"},
    {"en": "London", "fr": "Londres", "de": "London", "lb": "London"},
]
STANDARD_ORGS = [
    "ArcelorMittal",
    "Amazon",
    "BGL BNP Paribas",
    "Deutsche Bank",
    "Google",
    "Cargolux",
    "Luxair",
    "KPMG",
    "Deloitte",
    "Banque Internationale à Luxembourg",
    "RTL",
    "Goodyear",
]
LOCAL_ORGS = [
    "CFL",
    "Spuerkeess",
    "POST",
    "CHL",
    "CNS",
    "ADEM",
    "Garage Weis",
    "Brasserie Schuman",
    "Fiduciaire Kremer",
    "Café Majerus",
    "Cactus",
    "Bäckerei Jos Hoffmann",
]
# Luxembourgish short forms and first names; the second group is deliberately absent from the
# gazetteer so rules-only recall is not inflated by list overlap alone.
SHORT_NAMES = {
    "m": [
        "Jhemp",
        "Mett",
        "Méck",
        "Fränz",
        "Gusty",
        "Jang",
        "Pol",
        "Tun",
        "Charel",
        "Néckel",
        "Jos",
        "Gilbert",
        "René",
        "Norbert",
        "Raoul",
        "Ernest",
        "Edmond",
        "Arsène",
    ],
    "f": [
        "Lis",
        "Josée",
        "Mady",
        "Tessy",
        "Maisy",
        "Kätti",
        "Gréit",
        "Ginette",
        "Marthe",
        "Aline",
        "Romaine",
        "Nelly",
        "Fernande",
        "Léontine",
    ],
}
SURNAMES = [
    "Kieffer",
    "Schmit",
    "Weber",
    "Wagner",
    "Hoffmann",
    "Thill",
    "Majerus",
    "Reuter",
    "Kremer",
    "Hansen",
    "Welter",
    "Zeimet",
    "Ries",
    "Schiltz",
    "Weis",
    "Mangen",
    "Feltgen",
    "Gloden",
    "Wirtz",
    "Kayser",
    "Muller",
    "Steichen",
    "Goedert",
    "Bettendorf",
]
TITLES = {
    "m": [
        {"en": "Mr ", "fr": "Monsieur ", "de": "Herr ", "lb": ("den Här ", "dem Här ")},
        {"en": "Dr ", "fr": "le docteur ", "de": "Dr. ", "lb": ("den Dokter ", "dem Dokter ")},
    ],
    "f": [
        {"en": "Mrs ", "fr": "Madame ", "de": "Frau ", "lb": ("d'Madame ", "der Madame ")},
        {"en": "Dr ", "fr": "la docteure ", "de": "Dr. ", "lb": ("d'Doktesch ", "der Doktesch ")},
    ],
}
DE_DATIVE = {"Herr ": "Herrn "}
STREETS = [
    "rue de la Gare",
    "route d'Arlon",
    "avenue de la Liberté",
    "rue Principale",
    "Grand-Rue",
    "rue de l'Église",
    "boulevard Royal",
    "rue du Moulin",
    "Haaptstrooss",
    "rue des Champs",
]
MONTHS = {
    "en": [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ],
    "fr": [
        "janvier",
        "février",
        "mars",
        "avril",
        "mai",
        "juin",
        "juillet",
        "août",
        "septembre",
        "octobre",
        "novembre",
        "décembre",
    ],
    "de": [
        "Januar",
        "Februar",
        "März",
        "April",
        "Mai",
        "Juni",
        "Juli",
        "August",
        "September",
        "Oktober",
        "November",
        "Dezember",
    ],
    "lb": [
        "Januar",
        "Februar",
        "Mäerz",
        "Abrëll",
        "Mee",
        "Juni",
        "Juli",
        "August",
        "September",
        "Oktober",
        "November",
        "Dezember",
    ],
}
IBAN_FORMATS = {"LU": "3n13n", "FR": "10n11n2n", "DE": "18n", "BE": "12n", "NL": "4a10n"}


def same(x):
    return {lang: ("", x, "") for lang in LANGS}


def lb_article(name, gender, case):
    if gender == "f":
        return "d'" if case == "nom" else "der "
    if case == "dat":
        return "dem "
    # Eifeler Regel: the final n is kept before vowels and n, d, t, z, h.
    return "den " if name[0].lower() in "aeiouäéëndtzh" else "de "


def full_name():
    loc = rng.choice(["en_GB", "fr_FR", "de_DE"])
    if rng.random() < 0.3:
        return same(f"{rng.choice(SHORT_NAMES['m'] + SHORT_NAMES['f'])} {rng.choice(SURNAMES)}")
    return same(f"{fake[loc].first_name()} {fake[loc].last_name()}")


def short_name():
    gender = rng.choice("mf")
    name = rng.choice(SHORT_NAMES[gender])
    value = {lang: ("", name, "") for lang in ["en", "fr", "de"]}
    value["lb"] = {
        "nom": (lb_article(name, gender, "nom"), name, ""),
        "dat": (lb_article(name, gender, "dat"), name, ""),
        "poss": (lb_article(name, gender, "dat"), name, " säi" if gender == "m" else " hire"),
    }
    return value


def titled_name():
    gender = rng.choice("mf")
    title = rng.choice(TITLES[gender])
    surname = rng.choice(SURNAMES)
    value = {lang: (title[lang], surname, "") for lang in ["en", "fr"]}
    value["de"] = {"nom": (title["de"], surname, ""), "dat": (DE_DATIVE.get(title["de"], title["de"]), surname, "")}
    value["lb"] = {"nom": (title["lb"][0], surname, ""), "dat": (title["lb"][1], surname, "")}
    return value


def lower_name():
    return same(rng.choice(SHORT_NAMES["m"] + SHORT_NAMES["f"]).lower())


def ascii_slug(x):
    return unicodedata.normalize("NFKD", x).encode("ascii", "ignore").decode().lower().replace(" ", "")


def email(person):
    first, *last = person["en"][1].split()
    domain = rng.choice(["pt.lu", "post.lu", "gmail.com", "education.lu", "outlook.fr", "web.de", "example.com"])
    user = ascii_slug(first) + ("." + ascii_slug(last[0]) if last else str(rng.randint(10, 99)))
    return same(f"{user}@{domain}")


def digits(n):
    return "".join(str(rng.randint(0, 9)) for _ in range(n))


def standard_phone():
    return same(
        rng.choice(
            [
                f"+352 621 {digits(3)} {digits(3)}",
                f"+352 26 {digits(2)} {digits(2)} {digits(2)}",
                f"00352 691 {digits(3)} {digits(3)}",
                f"+33 6 {digits(2)} {digits(2)} {digits(2)} {digits(2)}",
                f"06 {digits(2)} {digits(2)} {digits(2)} {digits(2)}",
                f"+49 171 {digits(7)}",
                f"0171 {digits(7)}",
                f"+32 470 {digits(2)} {digits(2)} {digits(2)}",
                f"0470 {digits(2)} {digits(2)} {digits(2)}",
                f"+31 6 {digits(8)}",
                f"06-{digits(8)}",
            ]
        )
    )


def hard_phone():
    return same(
        rng.choice(
            [
                f"26 {digits(2)} {digits(2)} {digits(2)}",
                f"691{digits(6)}",
                f"661 {digits(3)} {digits(3)}",
                f"+352 2{digits(3)} {digits(4)}",
                f"(+352) 621 {digits(3)} {digits(3)}",
                f"0352 621 {digits(6)}",
                f"+49 (0)30 {digits(8)}",
                f"0170/{digits(7)}",
            ]
        )
    )


def mod97(s):
    return int("".join(str(int(c, 36)) for c in s)) % 97


def iban(compact=False):
    country = rng.choice(list(IBAN_FORMATS))
    bban = ""
    for n, kind in re.findall(r"(\d+)([na])", IBAN_FORMATS[country]):
        bban += "".join(
            rng.choice("ABCDEFGHIJKLMNOPQRSTUVWXYZ") if kind == "a" else str(rng.randint(0, 9)) for _ in range(int(n))
        )
    check = 98 - mod97(bban + country + "00")
    x = f"{country}{check:02d}{bban}"
    return same(
        x.lower()
        if compact and rng.random() < 0.5
        else x
        if compact
        else " ".join(x[i : i + 4] for i in range(0, len(x), 4))
    )


def card():
    kind = rng.choice(["visa16", "mastercard", "amex"])
    number = fake.credit_card_number(card_type=kind)
    groups = [4, 6, 5] if kind == "amex" else [4, 4, 4, 4]
    out, i = [], 0
    for g in groups:
        out.append(number[i : i + g])
        i += g
    return same(" ".join(out))


def luhn_digit(s):
    total = 0
    for i, ch in enumerate(reversed(s)):
        d = int(ch) * (2 if i % 2 == 0 else 1)
        total += d - 9 if d > 9 else d
    return (10 - total % 10) % 10


VERHOEFF_D = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    [1, 2, 3, 4, 0, 6, 7, 8, 9, 5],
    [2, 3, 4, 0, 1, 7, 8, 9, 5, 6],
    [3, 4, 0, 1, 2, 8, 9, 5, 6, 7],
    [4, 0, 1, 2, 3, 9, 5, 6, 7, 8],
    [5, 9, 8, 7, 6, 0, 4, 3, 2, 1],
    [6, 5, 9, 8, 7, 1, 0, 4, 3, 2],
    [7, 6, 5, 9, 8, 2, 1, 0, 4, 3],
    [8, 7, 6, 5, 9, 3, 2, 1, 0, 4],
    [9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
]
# Each permutation row is the previous one composed with row 1.
VERHOEFF_P = [[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], [1, 5, 7, 6, 2, 8, 3, 0, 9, 4]]
for _ in range(6):
    VERHOEFF_P.append([VERHOEFF_P[1][x] for x in VERHOEFF_P[-1]])
VERHOEFF_INV = [0, 4, 3, 2, 1, 5, 6, 7, 8, 9]


def verhoeff_digit(s):
    c = 0
    for i, ch in enumerate(reversed(s)):
        c = VERHOEFF_D[c][VERHOEFF_P[(i + 1) % 8][int(ch)]]
    return VERHOEFF_INV[c]


def birth_date():
    return fake.date_of_birth(minimum_age=18, maximum_age=90)


def nid(spaced=False):
    base = birth_date().strftime("%Y%m%d") + digits(3)
    x = f"{base}{luhn_digit(base)}{verhoeff_digit(base)}"
    return same(f"{x[:4]} {x[4:8]} {x[8:11]} {x[11:]}" if spaced else x)


def dob():
    d = birth_date()
    numeric = rng.random() < 0.5
    out = {}
    for lang in LANGS:
        month = MONTHS[lang][d.month - 1]
        if numeric:
            text = d.strftime("%d/%m/%Y" if lang in ("en", "fr") else "%d.%m.%Y")
        elif lang in ("de", "lb"):
            text = f"{d.day}. {month} {d.year}"
        else:
            text = f"{d.day} {month} {d.year}"
        out[lang] = ("", text, "")
    return out


def url(person):
    first, *last = person["en"][1].split()
    slug = ascii_slug(first) + ("-" + ascii_slug(last[0]) if last else "")
    return same(
        rng.choice([f"https://www.linkedin.com/in/{slug}", f"https://{slug}.lu", f"www.{slug.replace('-', '')}.com"])
    )


def ip():
    if rng.random() < 0.25:
        return same(f"2001:db8:{rng.randint(1, 0xFFFF):x}::{rng.randint(1, 0xFFFF):x}")
    return same(".".join(str(rng.randint(1, 254)) for _ in range(4)))


def address():
    return same(f"{rng.randint(1, 120)}, {rng.choice(STREETS)}")


def local_places():
    rows = []
    for name in ("communes.csv", "settlements.csv"):
        with open(RAW / name, encoding="utf-8") as f:
            rows += list(csv.DictReader(f))
    keep = {"commune", "village", "town", "municipality seat", "locality", "human settlement"}
    places = {}
    for r in rows:
        labels = [r["lb"], r["fr"], r["de"]]
        if r["type"] in keep and all(labels) and r["lb"] != r["fr"] and not any("(" in x for x in labels):
            places[r["item"]] = {"en": r["en"] or r["fr"], "fr": r["fr"], "de": r["de"], "lb": r["lb"]}
    return sorted(places.values(), key=lambda x: x["fr"])


LOCAL_PLACES = local_places()


def values(slots, hard):
    out = {}
    person = None
    for slot in slots:
        kind = re.sub(r"\d", "", slot)
        if kind == "p":
            out[slot] = short_name() if hard else full_name()
            person = person or out[slot]
        elif kind == "t":
            out[slot] = titled_name()
        elif kind == "l":
            out[slot] = lower_name()
    person = person or full_name()
    makers = {
        "loc": lambda: {k: ("", v, "") for k, v in rng.choice(LOCAL_PLACES if hard else STANDARD_PLACES).items()},
        "org": lambda: same(rng.choice(LOCAL_ORGS if hard else STANDARD_ORGS)),
        "addr": address,
        "email": lambda: email(person),
        "phone": hard_phone if hard else standard_phone,
        "iban": lambda: iban(compact=hard),
        "card": card,
        "nid": lambda: nid(spaced=hard),
        "dob": dob,
        "url": lambda: url(person),
        "ip": ip,
    }
    for slot in slots:
        if slot not in out:
            out[slot] = makers[slot]()
    return out


def render(template, vals, lang, capitalize=True):
    text, spans = "", []
    for i, part in enumerate(re.split(r"\{([\w@]+)\}", template)):
        if i % 2 == 0:
            text += part
            continue
        slot, _, case = part.partition("@")
        value = vals[slot][lang]
        prefix, entity, suffix = value[case or "nom"] if isinstance(value, dict) else value
        if not text and capitalize:
            prefix, entity = (
                (prefix[:1].upper() + prefix[1:], entity) if prefix else ("", entity[:1].upper() + entity[1:])
            )
        text += prefix
        start = len(text.encode())
        text += entity
        kind = re.sub(r"\d", "", slot)
        spans.append(
            {"type": SLOT_TYPES.get(kind, "person"), "start": start, "end": len(text.encode()), "text": entity}
        )
        text += suffix
    return text, spans


def main():
    ap = argparse.ArgumentParser(description="Generate the synthetic labeled dataset")
    ap.add_argument("--split", choices=["eval", "dev"], default="eval")
    args = ap.parse_args()
    seed, entity_free, out = (SEED, ENTITY_FREE, OUT) if args.split == "eval" else (DEV_SEED, ENTITY_FREE_DEV, DEV_OUT)
    rng.seed(seed)
    Faker.seed(seed)
    rows = []
    for subset, templates in [("standard", STANDARD), ("hard", HARD)]:
        for tid, tpl in templates.items():
            slots = list(dict.fromkeys(s.partition("@")[0] for s in re.findall(r"\{([\w@]+)\}", tpl["en"] + tpl["lb"])))
            for k in range(INSTANCES):
                vals = values(slots, subset == "hard")
                for lang in LANGS:
                    text, spans = render(tpl[lang], vals, lang, capitalize=tid != "chat")
                    group = f"{subset}-{tid}-{k}"
                    rows.append(
                        {
                            "id": f"{lang}-{group}",
                            "group": group,
                            "lang": lang,
                            "subset": subset,
                            "text": text,
                            "spans": spans,
                        }
                    )
    for k, sentence in enumerate(entity_free):
        for lang in LANGS:
            group = f"entity_free-{k:02d}"
            rows.append(
                {
                    "id": f"{lang}-{group}",
                    "group": group,
                    "lang": lang,
                    "subset": "entity_free",
                    "text": sentence[lang],
                    "spans": [],
                }
            )
    for r in rows:
        raw = r["text"].encode()
        assert all(raw[s["start"] : s["end"]].decode() == s["text"] for s in r["spans"]), r
    out.parent.mkdir(exist_ok=True)
    out.write_text("".join(json.dumps(r, ensure_ascii=False) + "\n" for r in rows), encoding="utf-8", newline="\n")
    print(len(rows), "sentences written to", out)


if __name__ == "__main__":
    main()
