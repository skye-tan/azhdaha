#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_fibonacci(int length) {
    LINEAR_TYPE int *fib = malloc(length * sizeof(int));
    for (int i = 0; i < length; i++) {
        if (i == 0 || i == 1) {
            fib[i] = i;
        } else {
            fib[i] = fib[i - 1] + fib[i - 2];
        }
    }
    return fib;
}

void modify_fibonacci(LINEAR_TYPE int *fib, int length) {
    for (int i = 0; i < length; i++) {
        if (fib[i] % 2 == 0) {
            fib[i] /= 2;
        } else {
            fib[i] *= 3;
        }
    }
    free(fib);
}

int sum_odd_fibonacci(LINEAR_TYPE int *fib, int length) {
    int sum = 0;
    for (int i = 0; i < length; i++) {
        if (fib[i] % 2 != 0) {
            sum += fib[i]; // Use after free
        }
    }
    return sum;
}

int main() {
    LINEAR_TYPE int *sequence = create_fibonacci(20);
    modify_fibonacci(sequence, 20);
    int odd_sum = sum_odd_fibonacci(sequence, 20);
    return 0;
}