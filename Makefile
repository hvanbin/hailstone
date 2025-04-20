.POSIX:
.PHONY: all clean

CC = cc
CFLAGS = -I include -std=c11
LDFLAGS =

BIN = hailstone
OBJ = hailstone.o
SRC = hailstone.c

$(BIN): $(OBJ)
	$(CC) -o $@ $^ $(LDFLAGS)

hailstone.o: hailstone.c

clean:
	$(RM) $(BIN) $(OBJ)

lint:
	-splint $(SRC)
