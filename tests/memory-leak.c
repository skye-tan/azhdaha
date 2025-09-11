
#include <azhdaha.h>
#include <stdlib.h>

// Variable 'ptr' is defined as a linear variable but does not move its value
// before the function ends.
// In a C program, this action will cause memory leakage.
void foo_1() {
    LINEAR_TYPE int *ptr;

    ptr = malloc(100);

    return;
}

// Variable 'ptr' is defined as a linear variable but its value has been
// overwritten by another allocation.
// In a C program, this action will cause memory leakage.
void foo_2() {
    LINEAR_TYPE int *ptr;

    ptr = malloc(100);

    ptr = malloc(100);

    return;
}

int main() {
    foo_1();

    foo_2();

    return 0;
}