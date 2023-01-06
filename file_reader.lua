_G.table = {}

function slice_lines(ligne, max_size)
	t = {}
	while(#ligne >= max_size) do
		l1 = string.sub(ligne, 1, max_size)
		ligne = string.sub(ligne, max_size, #ligne)
		t[#t + 1] = l1
	end
	return t
end

function init_table(file, max_size)
	i = 0
	file = io.open(file, "r")
	if (file == nil) then
		return nil
	else
		ligne = file:read()
		lignes = slice_lines(ligne, max_size)
		for k = 1, #lignes do
			print(lignes[k])
			_G.table[i] = lignes[k]
			i = i + 1
		end
	end
end

function read_line(i)
		return _G.table[i]
end

function reset_table()
	_G.table = {}
end
