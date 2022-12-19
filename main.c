#include <lua.h>
#include <lualib.h>
#include <lauxlib.h>
#include <locale.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

#include "terminal_printer.h"

// Dans ce fichier, je dois :
// récupérer le texte généré par notre truc lua, formatté en un tableau de char*. Dans chaque case, une phase.
// envoyer ce text dans notre programme de terminal printing qui va afficher le bouzin, et totu comparer.


const char* get_text(char* fichier){

	// Initialize the state
	lua_State *L = luaL_newstate();
	// load the librairies
	luaL_openlibs(L);
	// do file
	luaL_dofile(L,fichier);

	lua_getglobal(L, "sentence_generator");

	if (lua_isnil(L,-1)){
		printf("Cette variable n’est pas définie dans le code lua !\n");
	}
	else{
		lua_pcall(L, 0, 1, 0);
		const uint8_t* text = lua_tolstring(L, -1, NULL);
		return text;
	}
}

int main(int argc, char * argv[]){
	setlocale(LC_CTYPE,"");
	while(1){

		const uint8_t* text = get_text("generateur.lua");
		start_screen(text);
	}

	return 1;
}
