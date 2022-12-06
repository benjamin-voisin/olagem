#include "lua.h"
#include "lualib.h"
#include "lauxlib.h"

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
		printf("%s\n", text);
	}

	return 1;
}
