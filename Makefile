default: olagem

olagem: main.c terminal_printer.o screen_tools.o
	gcc -o olagem -llua -lncursesw main.c terminal_printer.o screen_tools.o


terminal_printer.o: terminal_printer.c terminal_printer.h
	gcc -c -lncursesw terminal_printer.c 

screen_tools.o: screen_tools.c screen_tools.h
	gcc -c -lncursesw screen_tools.c

clean:
	rm *.o
	rm olagem
