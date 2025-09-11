#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_pattern(int length) {
    LINEAR_TYPE int *pattern = malloc(length * sizeof(int));
    for (int i = 0; i < length; i++) {
        if (i == 0 || i == 1) {
            pattern[i] = i;
        } else {
            pattern[i] = pattern[i - 1] + pattern[i - 2];
        }

        if (pattern[i] % 2 == 0) {
            pattern[i] /= 2;
        } else {
            pattern[i] = pattern[i] * 3 + 1;
        }
    }
    return pattern;
}

void reverse_pattern(LINEAR_TYPE int *pattern, int length) {
    for (int i = 0; i < length / 2; i++) {
        int temp = pattern[i];
        pattern[i] = pattern[length - 1 - i];
        pattern[length - 1 - i] = temp;
    }
    free(pattern);
}

int find_pattern(LINEAR_TYPE int *pattern, int length, int value) {
    for (int i = 0; i < length; i++) {
        if (pattern[i] == value) {
            return i; // Use after free
        }
    }
    return -1;
}

int main() {
    LINEAR_TYPE int *sequence = create_pattern(20);
    reverse_pattern(sequence, 20);
    int pos = find_pattern(sequence, 20, 10);
    return 0;
}