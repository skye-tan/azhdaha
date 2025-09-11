
#include <azhdaha.h>
#include <stdlib.h>

// Variable 'ptr' is defined as a linear variable adn has been accessed after
// being freed.
// In a C program, this action will make the program prone to various
// exploitations.
void foo() {
    LINEAR_TYPE int *ptr;

    ptr = malloc(100);

    free(ptr);

    int temp = *ptr;

    return;
}

int main() {
    foo();

    return 0;
}