
#include <azhdaha.h>
#include <stdlib.h>

// Variable 'ptr' is defined as a non-linear variable and has been freed.
// In a C program, this action will cause 'ptr' to become a dangling pointer in
// 'main' function.
void foo(int *ptr) {
    free(ptr);

    return;
}

int main() {
    LINEAR_TYPE int *ptr = malloc(100);

    foo(ptr);

    return 0;
}