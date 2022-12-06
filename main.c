#include "lua.h"
#include "lualib.h"
#include "lauxlib.h"

#include "terminal_printer.h"

// Dans ce fichier, je dois :
// récupérer le texte généré par notre truc lua, formatté en un tableau de char*. Dans chaque case, une phase.
// envoyer ce text dans notre programme de terminal printing qui va afficher le bouzin, et totu comparer.

int main(){
	
	// Initialize the state
	lua_State *L = luaL_newstate();
	// load the librairies
	luaL_openlibs(L);
	// do file
	luaL_dofile(L,"generateur.lua");

	lua_getglobal(L, "text");
	if (lua_isnil(L,-1)){
		printf("Cette variable n’est pas définie dans le code lua!\n");
	}
	else{
		const char* text = lua_tolstring(L, -1,NULL);
		start_screen(text);
	}

	return 1;
}
