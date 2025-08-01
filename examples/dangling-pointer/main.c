
#include <azhdaha.h>

LINEAR_TYPE int *my_malloc();
void my_free(LINEAR_TYPE int *);

// Variable 'ptr' is defined as a non-linear variable and has been freed.
// In a C program, this action will cause 'ptr' to become a dangling pointer in
// 'main' function.
void foo(int *ptr) {
    my_free(ptr);

    return;
}

int main() {
    LINEAR_TYPE int *ptr = my_malloc();

    foo(ptr);

    return 0;
}