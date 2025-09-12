#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_sequence(int length) {
    LINEAR_TYPE int *seq = malloc(length * sizeof(int));
    return seq;
}

void generate_fibonacci(int *seq, int length) {
    if (length > 0)
        seq[0] = 0;
    if (length > 1)
        seq[1] = 1;
    for (int i = 2; i < length; i++) {
        seq[i] = seq[i - 1] + seq[i - 2];
    }
}

int sum_sequence(int *seq, int length) {
    int sum = 0;
    for (int i = 0; i < length; i++) {
        sum += seq[i];
    }
    return sum;
}

void destroy_sequence(LINEAR_TYPE int *seq, int length) {
    int total = sum_sequence(seq, length);
    free(seq);
}

int main() {
    LINEAR_TYPE int *fib = create_sequence(12);
    generate_fibonacci(fib, 12);
    destroy_sequence(fib, 12);
    return 0;
}