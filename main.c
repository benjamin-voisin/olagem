#include <lua.h>
#include <lualib.h>
#include <lauxlib.h>
#include <locale.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <time.h>

#include "terminal_printer.h"
#include "endscreen.h"

// Dans ce fichier, je dois :
// récupérer le texte généré par notre truc lua, formatté en un tableau de char*. Dans chaque case, une phase.
// envoyer ce text dans notre programme de terminal printing qui va afficher le bouzin, et totu comparer.


lua_State * init_lua(char* fichier){

	// Initialize the state
	lua_State *L = luaL_newstate();
	// load the librairies
	luaL_openlibs(L);
	// do file
	luaL_dofile(L,fichier);

	return L;
}

const uint8_t* get_text(lua_State* L){
	lua_getglobal(L, "sentence_generator");
	lua_pcall(L, 0, 1, 0);
	const uint8_t* text = lua_tolstring(L, -1, NULL);

	return text;
}

int main(int argc, char * argv[]){
	setlocale(LC_CTYPE,"");
	lua_State *L = init_lua("generateur.lua");

	bool restart = true;

	while (restart){
		const uint8_t* first_sentence = get_text(L);
		const uint8_t* second_sentence = get_text(L);
		time_t start_time = time(NULL);
		time_t actual_time;
		int number_of_words = 0;
		while(((actual_time = time(NULL)) - start_time) < 20){

			number_of_words += start_screen(first_sentence, second_sentence);
			clear();
			first_sentence = second_sentence;
			second_sentence = get_text(L);
		}
		restart = end_screen(number_of_words, time(NULL) - start_time);
	}
	lua_close(L);

	return 1;
}
