CC = gcc
CFLAGS = -Wall
RUST_LIB_DIR = rust_lib/target/release
RUST_LIB_FILE = $(RUST_LIB_DIR)/librust_lib.so

all: main

main: main.o rustlib
	$(CC) $(CFLAGS) -o main main.o $(RUST_LIB_FILE)

main.o: main.c
	$(CC) $(CFLAGS) -c main.c

rustlib:
	cd rust_lib && cargo build --release

clean:
	rm -f main.o main
	cd rust_lib && cargo clean

