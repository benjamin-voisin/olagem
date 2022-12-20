require "math"

math.randomseed(os.time())

function sentence_generator()
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

