
#include <azhdaha.h>

LINEAR_TYPE int *my_malloc();
void my_free(LINEAR_TYPE int *);

// Variable 'ptr' is defined as a linear variable adn has been accessed after
// being freed.
// In a C program, this action will make the program prone to various
// exploitations.
void foo() {
    LINEAR_TYPE int *ptr;

    ptr = my_malloc();

    my_free(ptr);

    int temp = *ptr;

    return;
}

int main() {
    foo();

    return 0;
}