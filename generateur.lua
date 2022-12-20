require "math"

math.randomseed(os.time())

function sentence_generator()
	nb_words_per_sentences = 10
	words = {"le", "de", "un", "être", "et", "à", "il", "avoir", "ne", "je", "son", "que", "se", "qui", "ce", "dans", "en", "du", "elle", "au", "de", "ce", "le", "pour", "pas", "que", "vous", "par", "sur", "faire", "plus", "dire", "me", "on", "mon", "lui", "nous", "comme", "mais", "pouvoir", "avec", "tout", "y", "aller", "voir", "en", "bien", "où", "sans", "tu", "ou", "leur", "homme", "si", "deux", "mari", "moi", "vouloir", "te", "femme", "venir", "quand", "grand", "celui", "si", "notre", "devoir", "là", "jour", "prendre", "même", "votre", "tout", "rien", "petit", "encore", "aussi", "quelque", "dont", "tout", "donner", "temps", "ça", "peu", "même", "falloir", "sous", "parler", "alors", "main", "chose", "ton", "mettre", "vie", "savoir", "yeux", "passer", "autre", "après", "regarder", "toujours", "puis", "jamais", "cela", "aimer", "non", "heure", "croire", "cent"}
	text = words[math.random(#words)]
	for k = 1, nb_words_per_sentences do
		text = text.." "..words[math.random(#words)]
	end
	return text

end

function cadavres_exquis()
	noms = {"Le cadavre", "Le chat", "Le chien", "La grenouille"}
	noms.taille = 4
	adjectifs1 = {"exquis", "noir", "pouilleux", "gluant"}
	adjectifs1.taille = 4
	verbes = {"boira", "mange", "aime", "capture"}
	verbes.taille = 4
	complements = {"le vin", "un rat", "un os", "un moustique"}
	complements.taille = 4
	adjectifs2 = {"nouveau", "gris", "moelleux", "agressif"}
	adjectifs2.taille = 4
	phrase = noms[math.random(noms.taille)].." "..adjectifs1[math.random(adjectifs1.taille)].." "..verbes[math.random(verbes.taille)].." "..complements[math.random(complements.taille)].." "..adjectifs2[math.random(adjectifs2.taille)].."."
	return phrase
end
