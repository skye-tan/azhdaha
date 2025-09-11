#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE long long int *create_sequence(int length) {
    LINEAR_TYPE long long int *seq = malloc(length * sizeof(long long int));
    for (int i = 0; i < length; i++) {
        if (i == 0) {
            seq[i] = 1LL;
        } else if (i == 1) {
            seq[i] = 2LL;
        } else {
            seq[i] = seq[i - 1] * seq[i - 2];
        }

        if (seq[i] > 1000000LL) {
            seq[i] /= 10LL;
        }
    }
    return seq;
}

void modify_sequence(LINEAR_TYPE long long int *seq, int length) {
    for (int i = 0; i < length; i++) {
        if (seq[i] % 2LL == 0LL) {
            seq[i] /= 2LL;
        } else {
            seq[i] = seq[i] * 3LL + 1LL;
        }
    }
    free(seq);
}

long long int find_value(LINEAR_TYPE long long int *seq, int length,
                         long long int target) {
    for (int i = 0; i < length; i++) {
        if (seq[i] == target) {
            return i; // Use after free
        }
    }
    return -1LL;
}

int main() {
    LINEAR_TYPE long long int *numbers = create_sequence(15);
    modify_sequence(numbers, 15);
    long long int pos = find_value(numbers, 15, 100LL);
    return 0;
}