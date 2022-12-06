main: main.c terminal_printer.o
	gcc -o aclue -llua -lncurses main.c terminal_printer.o 

terminal_printer.o: terminal_printer.c terminal_printer.h
	gcc -c -lncurses terminal_printer.c 


clean:
	rm *.o
