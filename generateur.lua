require "math"

math.randomseed(os.time())

function sentence_generator()
	n = math.random(5)
	tableau = {
		"Première phrase\n",
		"Deuxième phrase\n",
		"Troisième phrase\n",
		"Quatrième phrase\n",
		"Cinquième phrase\n"
	}
	return tableau[n]
end

