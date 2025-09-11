#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE long *create_sequence(int length) {
    LINEAR_TYPE long *seq = malloc(length * sizeof(long));
    for (int i = 0; i < length; i++) {
        if (i == 0) {
            seq[i] = 1L;
        } else if (i == 1) {
            seq[i] = 2L;
        } else {
            seq[i] = seq[i - 1] + seq[i - 2];
        }

        if (seq[i] % 3 == 0) {
            seq[i] /= 3;
        }
    }
    return seq;
}

void modify_sequence(LINEAR_TYPE long *seq, int length) {
    for (int i = 0; i < length; i++) {
        if (seq[i] % 2L == 0L) {
            seq[i] /= 2L;
        } else {
            seq[i] = seq[i] * 3L + 1L;
        }
    }
    free(seq);
}

long find_value(LINEAR_TYPE long *seq, int length, long target) {
    for (int i = 0; i < length; i++) {
        if (seq[i] == target) {
            return i; // Use after free
        }
    }
    return -1L;
}

int main() {
    LINEAR_TYPE long *numbers = create_sequence(18);
    modify_sequence(numbers, 18);
    long pos = find_value(numbers, 18, 10L);
    return 0;
}