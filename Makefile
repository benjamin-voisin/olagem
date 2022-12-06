main: main.c terminal_printer.o
	gcc -o main -llua -lncurses main.c terminal_printer.o 

terminal_printer.o: terminal_printer.c terminal_printer.h
	cc -c -lncurses terminal_printer.c 


clean:
	rm *.o
