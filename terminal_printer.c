#include <stdlib.h>
#include <ncurses.h>

char* lire_ligne (const char* nom) {
	FILE * fichier;
	fichier = fopen("text.txt", "r");
	if (fichier == NULL) {
		perror("fopen");
		exit(1);
	}
	size_t taille_tampon = 32;
	char * ligne = malloc(taille_tampon * sizeof(char));
	if (ligne == NULL){
		perror("malloc");
		exit(1);
	}
	size_t taille_ligne;
	taille_ligne = getline(&ligne, &taille_tampon, fichier);
	fclose(fichier);
	return ligne;

}


int main() {

	char* ligne = lire_ligne("text.txt");
	char ch;
	bool running = true;
	int y, x;

	initscr();			/* Start curses mode 		  */
	start_color();
	if(has_colors() == FALSE){
		endwin();
		printf("Your terminal does not support color\n");
		exit(1);
	}
	init_pair(1, COLOR_BLACK, COLOR_BLACK);
	init_pair(2, COLOR_BLACK, COLOR_CYAN);
	cbreak(); 		/* supprime le buffer du terminal pour avoir les caractères instant */
	noecho();

	printw(ligne);
	refresh();
	attron(COLOR_PAIR(2));
	while(running){
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




	getch();			/* Wait for user input */
	endwin();			/* End curses mode		  */

	free(ligne);


	return 0;
}

