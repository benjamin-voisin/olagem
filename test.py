#Ce programme affiche un adjectif au hasard parmi l’ensemble des adjectifs de wikidata !!!
import requests
import json
import random


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
print (jason["results"]["bindings"][0]["lemma"]["value"])
