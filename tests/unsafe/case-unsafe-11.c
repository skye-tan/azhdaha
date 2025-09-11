#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_sequence(int length) {
    LINEAR_TYPE int *seq = malloc(length * sizeof(int));
    for (int i = 0; i < length; i++) {
        if (i == 0) {
            seq[i] = 1;
        } else if (i == 1) {
            seq[i] = 1;
        } else {
            seq[i] = seq[i - 1] + seq[i - 2];
        }
    }
    return seq;
}

void scale_sequence(LINEAR_TYPE int *seq, int length, int factor) {
    for (int i = 0; i < length; i++) {
        if (factor > 0) {
            seq[i] *= factor;
        } else {
            seq[i] = 0;
        }
    }
    free(seq);
}

int sum_sequence(LINEAR_TYPE int *seq, int length) {
    int sum = 0;
    for (int i = 0; i < length; i++) {
        if (seq[i] > 0) {
            sum += seq[i]; // Use after free
        }
    }
    return sum;
}

int main() {
    LINEAR_TYPE int *fib = create_sequence(10);
    scale_sequence(fib, 10, 2);
    int total = sum_sequence(fib, 10);
    return 0;
}