#include <stdlib.h>
#include <ncurses.h>
#include <locale.h>
#include <string.h>
#include <stdio.h>

#include "screen_tools.h"



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
			attron(COLOR_PAIR(4));
			addch(ch);
		}
	refresh();
	}

}

int start_screen(const uint8_t* first_sentence, const uint8_t* second_sentence) {
	clear();

	uint8_t ch;
	int i = 0;

	init();

	move(0,0);
	printw(first_sentence);
	printw("\n");
	printw(second_sentence);
	move(3,0);
	refresh();
	attron(COLOR_PAIR(2));
	while(*first_sentence != '\0'){
		ch = getch();
		if (ch == 127){
			if (is_not_first_caracter()) {
			suppr();
			first_sentence --;
			i --;
			}
		}
		else{
			addch(ch);
			if (ch == *first_sentence){ 
				first_sentence ++;
				i ++;
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
	return i;

}


