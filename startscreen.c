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
	int result;
	switch(ch){
		case 's' :
			result = 3;
			break;

		case 'r' :
			result = 1;
			break;

		case 'q' :
			result = 10;
			break;

		default :
			printf("Error in the startscreen function \n");
			result = 10;
			break;
	}
	return result;
}
