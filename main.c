#include <stdio.h>
#include "rust_lib.h"

// C function to be called from Rust
void hello_from_c() {
    printf("Extern Hello from C!\n");
}

int main() {
    printf("Hello, World (from C)!\n");
    hello_from_rust();
    return 0;
}

