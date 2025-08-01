
#include <azhdaha.h>

LINEAR_TYPE int *my_malloc();
void my_free(LINEAR_TYPE int *);

// Variable 'ptr' is defined as a linear variable but does not move its value
// before the function ends.
// In a C program, this action will cause memory leakage.
void foo_1() {
    LINEAR_TYPE int *ptr;

    ptr = my_malloc();

    return;
}

// Variable 'ptr' is defined as a linear variable but its value has been
// overwritten by another allocation.
// In a C program, this action will cause memory leakage.
void foo_2() {
    LINEAR_TYPE int *ptr;

    ptr = my_malloc();

    ptr = my_malloc();

    return;
}

int main() {
    foo_1();

    foo_2();

    return 0;
}