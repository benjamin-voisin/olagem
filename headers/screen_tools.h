
#include <ncurses.h>

void color_init(void);

void init(void);

void make_cursor(WINDOW*);

void suppr(WINDOW* window, WINDOW* display_text);

bool is_not_first_caracter(WINDOW* window);
