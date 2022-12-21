#include <ncurses.h>

#include "screen_tools.h"

void startscreen(void){
	init();
	clear();
	printw("oui");
	char ch = getch();
	endwin();
}
