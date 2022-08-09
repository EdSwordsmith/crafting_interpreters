CC=clang

.PHONY: all clean

all: hello

hello: main.c list.o
	$(CC) main.c list.o -o hello

list.o: list.c
	$(CC) -c list.c

clean:
	rm -f hello list.o
