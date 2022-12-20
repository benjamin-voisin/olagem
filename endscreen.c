
#include <stdlib.h>
#include <ncurses.h>
#include <locale.h>
#include <string.h>
#include <stdio.h>

void end_screen(int number_of_words, long int time){
	printf("Vous avez tapé %d caractères en %ld secondes", number_of_words, time); 
}	
