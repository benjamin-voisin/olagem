#include <stdlib.h>
#include <ncurses.h>
#include <locale.h>
#include <string.h>
#include <stdio.h>
#include <time.h>

#include "screen_tools.h"



void refresh_time(WINDOW* time_case, time_t start_time){
		time_t actual_time = time(NULL);
		wmove(time_case, 1, 1);
		wprintw(time_case, "%ld", actual_time - start_time);
		wrefresh(time_case);
}

void failed(uint8_t ch, WINDOW* window, WINDOW* time_case, time_t start_time){
	waddch(window, ch);
	int i = 1;
	while (i > 0){
		ch = wgetch(window);
		if (ch == 127){
			suppr(window);
			i --;
		}
		else{
			i ++;
			waddch(window, ch);
		}
	refresh_time(time_case, start_time);
	wrefresh(window);
	}

}


int start_screen(const uint8_t* first_sentence, const uint8_t* second_sentence, time_t start_time) {
	clear();

	uint8_t ch;
	int i = 0;

	init();

	int height, width;
	WINDOW* background = newwin(0,0,0,0);
	getmaxyx(background,height,width);
	wprintw(background, "oui");
	wrefresh(background);

	WINDOW* time_case = newwin(3, 5, height/4, width/3 - 6);
	box(time_case, 0, 0);
	refresh_time(time_case, start_time);

	WINDOW* display_text = newwin(5, 70, height/4, width/3);
	box(display_text, 0, 0);
	wmove(display_text, 1, 1);
	wprintw(display_text, first_sentence);
	wmove(display_text, 2, 1);
	wprintw(display_text, second_sentence);
	wrefresh(display_text);
	
	WINDOW* text_input = newwin(3, 70, height/4 +5, width/3);
	box(text_input, 0, 0);
	wmove(text_input, 1, 1);
	wrefresh(text_input);

	

	attron(COLOR_PAIR(2));
	while(*first_sentence != '\0'){
		ch = wgetch(text_input);
		if (ch == 127){
			if (is_not_first_caracter(text_input)) {
			suppr(text_input);
			first_sentence --;
			i --;
			}
		}
		else{
			waddch(text_input, ch);
			wrefresh(text_input);
			if (ch == *first_sentence){ 
				first_sentence ++;
				i ++;
			}
			else {

				suppr(text_input);
				wattron(text_input, COLOR_PAIR(4));
				failed( ch, text_input, time_case, start_time);
				wattroff(text_input, COLOR_PAIR(4));
			}
		}

	refresh_time(time_case, start_time);
	wrefresh(text_input);
	/* refresh();		/1* Print it on to the real screen *1/ */
	}
	delwin(time_case);
	delwin(display_text);
	delwin(text_input);
	delwin(background);
	return i;

}




