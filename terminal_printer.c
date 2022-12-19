#include <stdlib.h>
#include <ncurses.h>
#include <locale.h>
#include <string.h>
#include <stdio.h>

#include "screen_tools.h"

void suppr(){
	int x, y;
	getyx(stdscr, y, x);
	attron(COLOR_PAIR(1));
	move(y, x-1);
	addch(' ');
	move(y,x-1);
	attrset(COLOR_PAIR(2));
	make_cursor();
}


void failed(uint8_t ch){
	attron(COLOR_PAIR(4));
	addch(ch);
	int i = 1;
	while (i > 0){
		ch = getch();
		if (ch == 127){
			suppr();
			i --;
		}
		else{
			i ++;
			addch(ch);
		}
	}

}

void start_screen(const uint8_t* text) {



	uint8_t ch;

	init();

	printw(text);
	move(2,0);
	refresh();
	attron(COLOR_PAIR(2));
	while(*text != '\n'){
		ch = getch();
		if (ch == 127){
			suppr();
			text --;
		}
		else{
			addch(ch);
			if (ch == *text){ 
				text ++;
			}
			else {
				suppr();
				attron(COLOR_PAIR(4));
				failed(ch);
				attroff(COLOR_PAIR(4));
			}
		}

	refresh();		/* Print it on to the real screen */
	}
	endwin();

}


