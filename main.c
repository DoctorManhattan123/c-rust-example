#include <stdio.h>
#include "rust_lib.h"

int main() {
    printf("Hello, World (from C)!\n");
    hello_from_rust();
    return 0;
}

