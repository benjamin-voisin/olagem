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

	initscr();			/* Start curses mode 		  */
	cbreak(); 		/* supprime le buffer du terminal pour avoir les caractères instant */
	noecho();
	printw(ligne);	/* Print Hello World		  */
	refresh();			/* Print it on to the real screen */
	getch();			/* Wait for user input */
	endwin();			/* End curses mode		  */

	free(ligne);


	return 0;
}

