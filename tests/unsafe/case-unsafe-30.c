#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *generate_sequence(int length) {
    LINEAR_TYPE int *seq = malloc(length * sizeof(int));
    for (int i = 0; i < length; i++) {
        if (i == 0) {
            seq[i] = 0;
        } else if (i == 1) {
            seq[i] = 1;
        } else {
            seq[i] = seq[i - 1] + seq[i - 2] + seq[i - 3];
        }
    }
    return seq;
}

void filter_sequence(LINEAR_TYPE int *seq, int length) {
    for (int i = 0; i < length; i++) {
        if (seq[i] % 3 == 0) {
            seq[i] = 0;
        } else if (seq[i] % 3 == 1) {
            seq[i] = 1;
        } else {
            seq[i] = -1;
        }
    }
    free(seq);
}

int sum_filtered(LINEAR_TYPE int *seq, int length) {
    int sum = 0;
    for (int i = 0; i < length; i++) {
        sum += seq[i]; // Use after free
    }
    return sum;
}

int main() {
    LINEAR_TYPE int *series = generate_sequence(15);
    filter_sequence(series, 15);
    int total = sum_filtered(series, 15);
    return 0;
}