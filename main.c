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
#include "startscreen.h"
#include "settings.h"


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

int old_main(int argc, char * argv[]){
	setlocale(LC_CTYPE,"");
	lua_State *L = init_lua("generateur.lua");

	bool restart = true;

	long int maximal_time = max_time();

	startscreen();

	while (restart){
		const uint8_t* first_sentence = get_text(L);
		const uint8_t* second_sentence = get_text(L);
		time_t start_time = time(NULL);
		time_t actual_time;
		long int number_of_caractere = 0;
		while(((actual_time = time(NULL)) - start_time) < maximal_time){

			number_of_caractere += start_screen(first_sentence, second_sentence);
			clear();
			first_sentence = second_sentence;
			second_sentence = get_text(L);
		}
		restart = end_screen(number_of_caractere, time(NULL) - start_time);
	}
	lua_close(L);
	endwin();
	return 1;
}

int main (int argc, char * argv[]){
	setlocale(LC_CTYPE,"");
	lua_State *L = init_lua("generateur.lua");
	int state = 0;
	time_t start_time = time(NULL);
	long int number_of_caractere = 0;

	while(1){
		switch(state){
			case 0 :
				state = startscreen();
				break;

			case 1 :
				number_of_caractere = 0;
				start_time = time(NULL);
				long int maximal_time = max_time();
				const uint8_t* first_sentence = get_text(L);
				const uint8_t* second_sentence = get_text(L);
				time_t actual_time;
				while(((actual_time = time(NULL)) - start_time) < maximal_time){

					number_of_caractere += start_screen(first_sentence, second_sentence);
					clear();
					first_sentence = second_sentence;
					second_sentence = get_text(L);
				}					
				state = 2;
				break;


			case 2 :
				state = end_screen(number_of_caractere, time(NULL) - start_time);
				break;

			case 3 :
				printf("TODO");
				return 1;
				break;

			case 10 :
				lua_close(L);
				endwin();
				return 1;
				break;

			default :
				printf("Error in the main swith\n");
				break;
	}
	}

	return 1;

}
