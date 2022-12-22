default: olagem

olagem: main.c terminal_printer.o screen_tools.o endscreen.o startscreen.o settings.o
	gcc -g -o olagem -llua -lncursesw main.c terminal_printer.o screen_tools.o endscreen.o startscreen.o settings.o


terminal_printer.o: terminal_printer.c terminal_printer.h
	gcc -c -lncursesw terminal_printer.c 

screen_tools.o: screen_tools.c screen_tools.h
	gcc -c -lncursesw screen_tools.c

endscreen.o: endscreen.c endscreen.h
	gcc -c -lncursesw endscreen.c

startscreen.o: startscreen.c startscreen.h
	gcc -c -lncursesw startscreen.c

settings.o: settings.c settings.h
	gcc -c -lncursesw settings.c

clean:
	rm *.o
