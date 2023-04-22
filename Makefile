CC = gcc
CFLAGS = -Wall
RUST_LIB_DIR = rust_lib/target/release
RUST_LIB_NAME = rust_lib

all: main

main: rustlib
	$(CC) $(CFLAGS) -o main main.c -L$(RUST_LIB_DIR) -l$(RUST_LIB_NAME)

rustlib:
	cd rust_lib && cargo build --release

run: main
	export LD_LIBRARY_PATH=$(RUST_LIB_DIR):$$LD_LIBRARY_PATH; ./main

clean:
	rm -f main
	cd rust_lib && cargo clean

