#include <ncurses.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

long int max_time() {
  FILE *fichier;
  fichier = fopen("/usr/share/olagem/settings.txt", "r");
  if (fichier == NULL) {
    perror("fopen");
    endwin();
    exit(1);
  }
  char letter;
  char time_char[] = "ffff";
  int i = 0;
  while ((letter = fgetc(fichier)) != '=')
    ;
  while ((letter = fgetc(fichier)) != EOF) {
    time_char[i] = letter;
    i++;
  }
  fclose(fichier);
  return (atoi(time_char));
}
