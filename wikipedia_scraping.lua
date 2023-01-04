local http = require("socket.http")
local ltn12 = require 'ltn12'

function get_content(title)
	local body = {}
	url_request = "https://fr.wikipedia.org/w/api.php?action=query&format=json&prop=revisions&titles="..title.."&formatversion=2&rvprop=content&rvslots=*"
	local res, code, headers, status = http.request{
	  url = url_request,
	  sink = ltn12.sink.table(body)
	}

	local response = table.concat(body)

	print(response)
end

get_content("Taverny")
