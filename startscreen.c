#include <ncurses.h>

#include "screen_tools.h"

int startscreen(void){
	init();
	clear();
	printw("Press s to go to the settings page. Press r to start the game. Press q to quit.");
	char ch = getch();
	while(ch != 's' && ch != 'r' && ch != 'q'){
		ch = getch();
	}
	switch(ch){
		case 's' :
			return 3;
		case 'r' :
			return 1;
		case 'q' :
			return 10;
		default :
			printf("Error in the startscreen function \n");
			return 10;
	}
}
