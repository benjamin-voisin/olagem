
t_printer: terminal_printer.c
	gcc -o printer terminal_printer.c -lncurses
	rm *.o


clean:
	rm *.out
