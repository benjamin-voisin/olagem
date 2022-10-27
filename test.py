#Ce programme affiche un adjectif au hasard parmi l’ensemble des adjectifs de wikidata !!!
import requests
import json
import random

def random_adj():
    x = str(random.random())
    comment = "#" + x + "\n"

    url = 'https://query.wikidata.org/sparql'
    query = '''SELECT DISTINCT ?lemma WHERE {
      ?lexemeId dct:language wd:Q150;
                wikibase:lexicalCategory wd:Q34698;
                wikibase:lemma ?lemma.
       SERVICE wikibase:label { bd:serviceParam wikibase:language "[AUTO_LANGUAGE],en". }
    }
    ORDER BY UUID()
    LIMIT 1'''

    query = comment + query

    resp = str(requests.get(url, params={'query': query, 'format':'json'}).json())
    jason = json.loads(str(resp.replace("'",'"')))
    return (jason["results"]["bindings"][0]["lemma"]["value"])


def random_nom():
    x = random.random()
    comment = "#" + str(x) + "\n"

    url = 'https://query.wikidata.org/sparql'
    query = '''SELECT DISTINCT ?lemma ?genre WHERE {
      ?lexemeId dct:language wd:Q150;
                wikibase:lexicalCategory wd:Q1084.
     ?lexemeId wdt:P5185 ?genre;
                wikibase:lemma ?lemma.
       SERVICE wikibase:label { bd:serviceParam wikibase:language "[AUTO_LANGUAGE],en". }
    }
    ORDER BY UUID()
    LIMIT 1'''

    query = comment + query

    resp = str(requests.get(url, params={'query': query, 'format':'json'}).json())
    jason = json.loads(str(resp.replace("'",'"')))
    genre = jason["results"]["bindings"][0]["genre"]["value"]
    mot = jason["results"]["bindings"][0]["lemma"]["value"]
    if genre == "http://www.wikidata.org/entity/Q499327":
        if x > 0.5:
            pre = "le"
        else:
            pre = "un"
    else:
        if x > 0.5:
            pre = "la"
        else:
            pre = "une"
    return(pre + " " + mot)


def random_verbe():
    x = str(random.random())
    comment = "#" + x + "\n"

    url = 'https://query.wikidata.org/sparql'
    query = '''SELECT DISTINCT ?lemma WHERE {
      ?lexemeId dct:language wd:Q150;
                wikibase:lexicalCategory wd:Q24905;
                wikibase:lemma ?lemma.
       SERVICE wikibase:label { bd:serviceParam wikibase:language "[AUTO_LANGUAGE],en". }
    }
    ORDER BY UUID()
    LIMIT 1'''

    query = comment + query

    resp = str(requests.get(url, params={'query': query, 'format':'json'}).json())
    jason = json.loads(str(resp.replace("'",'"')))
    return (jason["results"]["bindings"][0]["lemma"]["value"])


def gener_phrase():
    phrase = random_nom() + " " + random_adj() + " " + random_verbe() + " " + random_nom() + " " + random_adj() + "."
    return phrase.capitalize()

print(gener_phrase())
