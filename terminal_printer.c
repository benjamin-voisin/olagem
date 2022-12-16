#include <stdlib.h>
#include <ncurses.h>
#include <locale.h>
#include <stdlib.h>
#include <string.h>
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
}

void init(){
	initscr();			/* Start curses mode 		  */
	color_init();
	cbreak(); 		/* supprime le buffer du terminal pour avoir les caractères instant */
	noecho();		// évite d’écrire les caractères qu’on tape (comme ça c’est nous qui gérons
}

void suppr(){
	int x, y;
	getyx(stdscr, y, x);
	attron(COLOR_PAIR(1));
	move(y, x-1);
	addch(' ');
	move(y,x-1);
	attrset(COLOR_PAIR(2));
}

void start_screen(const uint8_t* text) {


	uint8_t phrase[strlen(text)];
	int indice = 0;

	uint8_t ch;
	int y, x;

	init();
	x = 0;
	y = 2;

	printw(text);
	refresh();
	attron(COLOR_PAIR(2));
	while(1){
		ch = getch();
		if (ch == 127){
			suppr();
			indice --;
			phrase[indice] = '\0';
		}
		else{
			addch(ch);
			indice ++;
			phrase[indice] = ch;
			if (ch == *text){ 
				text ++;
			}
			else {
				attron(COLOR_PAIR(4));
			}
		}

	refresh();		/* Print it on to the real screen */
	}

}


