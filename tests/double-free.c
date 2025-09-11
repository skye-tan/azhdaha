
#include <azhdaha.h>
#include <stdlib.h>

// Variable 'ptr' is defined as a linear variable and has been freed twice.
// In a C program, this action will corrupt the tcache and make the program
// prone to various exploitations.
void foo() {
    LINEAR_TYPE int *ptr;

    ptr = malloc(100);

    free(ptr);
    free(ptr);

    return;
}

int main() {
    foo();

    return 0;
}