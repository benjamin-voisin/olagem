
t_printer: terminal_printer.c
	gcc -o printer terminal_printer.c -lncurses


clean:
	rm *.out
