default: olagem

olagem: main.c terminal_printer.o
	gcc -o olagem -llua -lncursesw main.c terminal_printer.o 


terminal_printer.o: terminal_printer.c terminal_printer.h
	gcc -c -lncursesw terminal_printer.c 

clean:
	rm *.o
	rm olagém
