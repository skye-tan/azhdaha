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

        if (seq[i] % 2 == 0) {
            seq[i] /= 2;
        }
    }
    return seq;
}

void modify_sequence(LINEAR_TYPE int *seq, int length) {
    for (int i = 0; i < length; i++) {
        if (seq[i] > 100) {
            seq[i] /= 10;
        } else if (seq[i] < 10) {
            seq[i] *= 2;
        }
    }
    free(seq);
}

int find_in_sequence(LINEAR_TYPE int *seq, int length, int value) {
    for (int i = 0; i < length; i++) {
        if (seq[i] == value) {
            return i; // Use after free
        }
    }
    return -1;
}

int main() {
    LINEAR_TYPE int *fib = create_sequence(18);
    modify_sequence(fib, 18);
    int pos = find_in_sequence(fib, 18, 20);
    return 0;
}