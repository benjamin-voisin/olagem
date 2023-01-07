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

const uint8_t* get_text(lua_State* L, int max_caractere){
	lua_getglobal(L, "sentence_generator");
	lua_pushnumber(L, max_caractere);
	lua_pcall(L, 1, 1, 0);
	const uint8_t* text = lua_tolstring(L, -1, NULL);
	return text;
}

uint8_t ** get_text_file(lua_State* L, int max_caractere, const char* file){
	/* lua_getglobal(L, "file_reader"); */
	/* lua_pushnumber(L, max_caractere); */
	/* lua_pushstring(L, file); */
	/* lua_pcall(L, 2, 2, 0); */
	/* int number_of_sentences = lua_tonumber(L, -2); */
	/* uint8_t text[number_of_sentences][max_caractere]; */
	/* printf("%d\n", number_of_sentences); */
	/* for (int k = 1; k <= number_of_sentences; k++){ */
	/* 	lua_rawgeti(L, -1, k); */ 
	/* 	uint8_t*  sentence = lua_tolstring(L, -1, NULL); */
	/* 	lua_settop(L, -2); */
	/* 	printf("%s\n", sentence); */
	/* 	strcpy(text[k-1], sentence); */
	/* } */
	/* return text; */
	return NULL;


}


int main (int argc, char * argv[]){
	initscr();
	setlocale(LC_CTYPE,"");
	lua_State *L = init_lua("lua/generateur.lua");
	int state = 0;
	time_t start_time = time(NULL);
	long int number_of_caractere = 0;
	int max_caractere = COLS / 1.5;
	if (argc > 1){
		state = 1;
		if ((strcmp(argv[1],"-h") == 0) || (strcmp(argv[1], "-help") == 0)){
			system("man ./manpage");
			return 1;
		}
	}

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
				time_t actual_time;
				if (argc > 1){

					lua_State *file_reader = init_lua("lua/file_reader.lua");

					lua_getglobal(file_reader, "init_table");
					lua_pushstring(file_reader, argv[1]);
					lua_pushnumber(file_reader, max_caractere);
					lua_pcall(file_reader, 2, 1, 0);
					if (lua_isnil(file_reader, -1)){
						printf("The file %s does not exist", argv[1]);
						lua_close(file_reader);
						endwin();
						return 1;
					}

					const uint8_t* first_line;
					const uint8_t* second_line;
					int i = 0;
					lua_getglobal(file_reader, "read_line");
					lua_pushnumber(file_reader, 0);
					lua_pcall(file_reader, 1, 1, 0);
					while (!lua_isnil(file_reader, -1)) {
						first_line = lua_tolstring(file_reader, -1, NULL);
						lua_getglobal(file_reader, "read_line");
						lua_pushnumber(file_reader, i + 1);
						lua_pcall(file_reader, 1, 1, 0);
						second_line = lua_tolstring(file_reader, -1, NULL);

						number_of_caractere += start_screen(first_line, second_line, start_time, max_caractere);
						clear();
						i ++;
						lua_getglobal(file_reader, "read_line");
						lua_pushnumber(file_reader, i);
						lua_pcall(file_reader, 1, 1, 0);



						}




					
					lua_close(file_reader);
					state = 2;
				}
				else{
				const uint8_t* first_sentence = get_text(L,max_caractere);
				const uint8_t* second_sentence = get_text(L,max_caractere);
				while(((actual_time = time(NULL)) - start_time) < maximal_time){

					number_of_caractere += start_screen(first_sentence, second_sentence, start_time, max_caractere);
					clear();
					first_sentence = second_sentence;
					second_sentence = get_text(L,max_caractere);
				}					
				state = 2;
				}
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
	return 1;

}
