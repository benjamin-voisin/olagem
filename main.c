#include <lua.h>
#include <lualib.h>
#include <lauxlib.h>
#include <locale.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <time.h>

#include "headers/terminal_printer.h"
#include "headers/endscreen.h"
#include "headers/startscreen.h"
#include "headers/settings.h"


lua_State * init_lua(char* fichier){

	// Initialize the state
	lua_State *L = luaL_newstate();
	// load the librairies
	luaL_openlibs(L);
	// do file
	luaL_dofile(L,fichier);

	return L;
}

const uint8_t* get_text(lua_State* L, int max_caractere, const char* file){
	lua_getglobal(L, "sentence_generator");
	lua_pushnumber(L, max_caractere);
	lua_pushstring(L, file);
	lua_pcall(L, 2, 1, 0);
	const uint8_t* text = lua_tolstring(L, -1, NULL);

	return text;
}



int main (int argc, char * argv[]){
	initscr();
	setlocale(LC_CTYPE,"");
	lua_State *L = init_lua("generateur.lua");
	int state = 0;
	time_t start_time = time(NULL);
	long int number_of_caractere = 0;
	int max_caractere = COLS / 2;
	if (argc > 1){
		if ((strcmp(argv[1],"-h") == 0) || (strcmp(argv[1], "-help") == 0)){
			system("man ./manpage");
			return 1;
		}
		get_text(L, max_caractere, argv[1]);
		lua_close(L);
		endwin();
		return 1;
	}
	else {

	while(1){
		switch(state){
			case 0 :
				state = startscreen();
				break;

			case 1 :
				clear();
				number_of_caractere = 0;
				start_time = time(NULL);
				long int maximal_time = max_time();
				const uint8_t* first_sentence = get_text(L,max_caractere, "");
				const uint8_t* second_sentence = get_text(L,max_caractere, "");
				time_t actual_time;
				while(((actual_time = time(NULL)) - start_time) < maximal_time){

					number_of_caractere += start_screen(first_sentence, second_sentence, start_time, max_caractere);
					clear();
					first_sentence = second_sentence;
					second_sentence = get_text(L,max_caractere,  "");
				}					
				state = 2;
				break;


			case 2 :
				state = end_screen(number_of_caractere, time(NULL) - start_time);
				break;

			case 3 :
				printf("TODO");
				lua_close(L);
				endwin();
				return 1;
				break;

			case 10 :
				lua_close(L);
				endwin();
				return 1;
				break;

			default :
				printf("Error in the main swith\n");
				lua_close(L);
				endwin();
				break;
	}
	}

	}
	return 1;

}
