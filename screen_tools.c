#include <stdlib.h>
#include <ncurses.h>
#include <locale.h>
#include <stdio.h>

void color_init(){
	start_color();
	if(has_colors() == FALSE){
		endwin();
		printf("Your terminal does not support color\n");
		exit(1);
	}
	init_pair(1, COLOR_BLACK, COLOR_BLACK);
	init_pair(2, COLOR_WHITE, COLOR_BLACK);
	init_pair(3, COLOR_GREEN, COLOR_BLACK);
	init_pair(4, COLOR_RED, COLOR_BLACK);
	init_pair(5, COLOR_RED, COLOR_RED);
	init_pair(6, COLOR_WHITE, COLOR_WHITE);
}

void init(){
	initscr();			/* Start curses mode 		  */
	color_init();
	cbreak(); 		/* supprime le buffer du terminal pour avoir les caractères instant */
	noecho();		// évite d’écrire les caractères qu’on tape (comme ça c’est nous qui gérons
}



void make_cursor(){
	int x, y;
	getyx(stdscr, y, x);
	attron(COLOR_PAIR(6));
	addch(' ');
	attroff(COLOR_PAIR(6));
	move(y,x);
}

void suppr(){
	int x, y;
	getyx(stdscr, y, x);
	attron(COLOR_PAIR(1));
	move(y, x-1);
	addch(' ');
	move(y,x-1);
	attroff(COLOR_PAIR(1));
	make_cursor();
}

bool is_not_first_caracter(){
	int y, x;
	getyx(stdscr, y, x);
	return (x != 0);
}
