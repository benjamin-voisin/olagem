
#include <stdlib.h>
#include <ncurses.h>
#include <locale.h>
#include <string.h>
#include <stdio.h>

#include "screen_tools.h"

void startscreen(void){
	init();
	clear();
	printw("oui");
	char ch = getch();
	endwin();
}
