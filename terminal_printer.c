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
	int ch;

	initscr();			/* Start curses mode 		  */
	cbreak(); 		/* supprime le buffer du terminal pour avoir les caractères instant */
	noecho();

	printw(ligne);
	ch = getch();
	if (ch == KEY_F(1)){
		printw("T’as appuyé sur F1 fréro");
	}
	else{
		printw("T'as appuye sur ");
		attron(A_BOLD);
		printw("%c", ch);
		attroff(A_BOLD);
	}


	refresh();			/* Print it on to the real screen */




	getch();			/* Wait for user input */
	endwin();			/* End curses mode		  */

	free(ligne);


	return 0;
}

