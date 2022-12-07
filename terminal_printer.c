#include <stdlib.h>
#include <ncurses.h>
#include <locale.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

void color_init(){
	init_pair(1, COLOR_BLACK, COLOR_BLACK);
	init_pair(2, COLOR_WHITE, COLOR_BLACK);
	init_pair(3, COLOR_GREEN, COLOR_BLACK);
	init_pair(4, COLOR_RED, COLOR_BLACK);
}

void start_screen(const char* text) {

	char * phrase;


	uint8_t ch;
	int y, x;

	initscr();			/* Start curses mode 		  */
	start_color();
	if(has_colors() == FALSE){
		endwin();
		printf("Your terminal does not support color\n");
		exit(1);
	}
	color_init();
	cbreak(); 		/* supprime le buffer du terminal pour avoir les caractères instant */
	noecho();		// évite d’écrire les caractères qu’on tape (comme ça c’est nous qui gérons

	printw(text);
	refresh();
	attron(COLOR_PAIR(2));
	while(1){
		ch = getch();
		if (ch == 127){
			getyx(stdscr, y, x);
			attron(COLOR_PAIR(1));
			move(y, x-1);
			addch(' ');
			move(y,x-1);
			attrset(COLOR_PAIR(2));
		}
		else{
			addch(ch);
		}

	refresh();		/* Print it on to the real screen */
	}

}


