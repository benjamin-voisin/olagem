default: olagem

install: olagem clean man

man: 
	cp manpage /usr/share/man/man6/olagem.6
	gzip /usr/share/man/man6/olagem.6

olagem: main.o terminal_printer.o screen_tools.o endscreen.o startscreen.o settings.o
	gcc -g -llua -lncursesw main.o terminal_printer.o screen_tools.o endscreen.o startscreen.o settings.o -o olagem 

main.o: main.c
	gcc -c -lncursesw main.c


terminal_printer.o: terminal_printer.c
	gcc -c -lncursesw terminal_printer.c 

screen_tools.o: screen_tools.c
	gcc -c -lncursesw screen_tools.c

endscreen.o: endscreen.c
	gcc -c -lncursesw endscreen.c

startscreen.o: startscreen.c
	gcc -c -lncursesw startscreen.c

settings.o: settings.c
	gcc -c -lncursesw settings.c

clean:
	rm *.o
