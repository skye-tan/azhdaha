#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_stack_array(int size) {
    LINEAR_TYPE int *stack = malloc(size * sizeof(int));
    return stack;
}

int push_elements(int *stack, int size) {
    for (int i = 0; i < size; i++) {
        stack[i] = i * i;
    }
    return size;
}

int pop_elements(int *stack, int count) {
    int sum = 0;
    for (int i = count - 1; i >= 0; i--) {
        sum += stack[i];
    }
    return sum;
}

void release_stack(LINEAR_TYPE int *stack, int size) {
    int popped_sum = pop_elements(stack, size);
    free(stack);
}

int main() {
    LINEAR_TYPE int *stack = create_stack_array(6);
    int pushed = push_elements(stack, 6);
    release_stack(stack, 6);
    return 0;
}