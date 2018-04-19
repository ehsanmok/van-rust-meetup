#include <stdint.h>
#include <stdio.h>

extern int32_t rust_abs(int32_t input);

int main() {
    printf("C calculates abs of -1 using Rust %d\n", rust_abs(-1));
    return 0;
}
