TITLE=tictactoe
CC=clang
CFLAGS=-Wall -Wextra -pedantic -g -fsanitize=address -std=c11
LIBS=-lncursesw
OUTPUT=bin
.PHONY: run clean
.DEFAULT_GOAL:=all

all: bin ${OUTPUT}/${TITLE}
bin:
	@mkdir bin

${OUTPUT}/${TITLE}: ${OUTPUT}/main.o
	@${CC} $< ${LIBS} ${CFLAGS} -o $@

${OUTPUT}/main.o: main.c global.h errors.h
	@${CC} $< ${CFLAGS} -c -o $@

run: all
	@./${OUTPUT}/${TITLE}

clean:
	@rm -rf bin
