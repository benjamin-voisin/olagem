
#include <stdlib.h>
#include <ncurses.h>
#include <locale.h>
#include <string.h>
#include <stdio.h>

#include "screen_tools.h"

bool end_screen(int number_of_words, long int time){
	clear();
	init();
	uint8_t ch;
	printw("You have typed %d caracteres in %ld seconds\n\n", number_of_words, time); 
	printw("Press any key to restart the game. Press q to quit");
	ch = getch();
	clear();
	endwin();
	if (ch == 'q'){
		return false;
	}
	else {
		return true;
	}
}	
