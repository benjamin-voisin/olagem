_G.table = {}

function slice_lines(ligne, max_size)
	t = {}
	if (#ligne < max_size) then
		return {ligne}
	else
		while(#ligne >= max_size) do
			l1 = string.sub(ligne, 1, max_size)
			ligne = string.sub(ligne, max_size + 1, #ligne)
			t[#t + 1] = l1
		end
		t[#t + 1] = ligne
		return t
	end
end

function init_table(file, max_size)
	i = 0
	file = io.open(file, "r")
	if (file == nil) then
		return nil
	else
		ligne = file:read()
		while (ligne ~= nil) do
			lignes = slice_lines(ligne, max_size)
			for k = 1, #lignes do
				_G.table[i] = lignes[k]
				i = i + 1
			end
			ligne = file:read()
		end
	end
end

function read_line(i)
		return _G.table[i]
end

function reset_table()
	_G.table = {}
end
