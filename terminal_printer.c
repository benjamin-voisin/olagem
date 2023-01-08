#include <stdlib.h>
#include <ncurses.h>
#include <locale.h>
#include <string.h>
#include <stdio.h>
#include <time.h>

#include "headers/screen_tools.h"



void refresh_time(WINDOW* time_case, time_t start_time){
		time_t actual_time = time(NULL);
		wmove(time_case, 1, 1);
		wprintw(time_case, "%ld", actual_time - start_time);
		wrefresh(time_case);
}

bool is_last(int max_caractere,WINDOW* window){
	int x, y;
	getyx(window,y,x);
	return (x == max_caractere + 1);
}

void failed(uint8_t ch, WINDOW* window, WINDOW* time_case,WINDOW* display_text, time_t start_time, int max_caractere, const uint8_t* text){
	wattron(window, COLOR_PAIR(4));
	waddch(window, ch);
	wattron(display_text, COLOR_PAIR(5));
	int x, y;
	getyx(display_text,y,x);
	wmove(display_text,y,x+1);
	waddch(display_text, *text);
	wrefresh(display_text);
	int i = 1;
	while (i > 0){
		ch = wgetch(window);
		wattron(window, COLOR_PAIR(4));
		if (ch == 127){
			suppr(window, display_text);
			getyx(display_text,y,x);
			wattron(display_text, COLOR_PAIR(2));
			waddch(display_text,*text);
			wattroff(display_text, COLOR_PAIR(2));
			wmove(display_text,y,x);
			wrefresh(display_text);
			text --;
			i --;
		}
		else{
			if (!is_last(max_caractere, window)){
				i ++;
				text ++;
				waddch(window, ch);
				wattron(display_text, COLOR_PAIR(5));
				waddch(display_text, *text);
				wrefresh(display_text);
			}
		}
	wattroff(display_text, COLOR_PAIR(4));
	wattron(display_text, COLOR_PAIR(3));
	refresh_time(time_case, start_time);
	wrefresh(window);
	}

}


int start_screen(const uint8_t* first_sentence, const uint8_t* second_sentence, time_t start_time, int max_caractere) {
	clear();

	uint8_t ch;
	int i = 0;

	init();

	int height, width;
	WINDOW* background = newwin(0,0,0,0);
	getmaxyx(background,height,width);
	wprintw(background, "oui");
	wrefresh(background);

	WINDOW* time_case = newwin(3, 5, height/4, ((width - max_caractere - 4) / 2) - 6);
	box(time_case, 0, 0);
	refresh_time(time_case, start_time);

	WINDOW* display_text = newwin(4, max_caractere + 2, height/4, (width - max_caractere - 4) / 2);
	box(display_text, 0, 0);
	wmove(display_text, 1, 1);
	wprintw(display_text, first_sentence);
	wmove(display_text, 2, 1);
	wprintw(display_text, second_sentence);
	wmove(display_text, 1, 1);
	wattron(display_text,COLOR_PAIR(3));
	wrefresh(display_text);
	
	WINDOW* text_input = newwin(3, max_caractere + 2, height/4 +5, (width - max_caractere - 4) /2);
	box(text_input, 0, 0);
	wmove(text_input, 1, 1);
	wrefresh(text_input);

	

	attron(COLOR_PAIR(2));
	while(*first_sentence != '\0'){
		ch = wgetch(text_input);
		if (ch == 127){
			if (is_not_first_caracter(text_input)) {
			int y, x;
			getyx(display_text,y,x);
			suppr(text_input, display_text);
			first_sentence --;
			wattron(display_text, COLOR_PAIR(2));
			waddch(display_text, *first_sentence);
			wattron(display_text, COLOR_PAIR(3));
			wmove(display_text, y, x - 1);
			i --;
			}
		}
		else{
			if (ch != 10 && ch != 9 ){
				waddch(text_input, ch);
				wrefresh(text_input);
				if (ch == *first_sentence){ 
					waddch(display_text, ch);
					first_sentence ++;
					i ++;
				}
				else {

					suppr(text_input, display_text);
					failed( ch, text_input, time_case,display_text, start_time, max_caractere, first_sentence);
					wattroff(text_input, COLOR_PAIR(4));
				}
			}
		}

	wrefresh(display_text);
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

int file_input_game(char ** text){
	return 1;
}




