#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_pattern(int length) {
    LINEAR_TYPE int *pattern = malloc(length * sizeof(int));
    for (int i = 0; i < length; i++) {
        if (i % 3 == 0) {
            pattern[i] = i * 2;
        } else if (i % 3 == 1) {
            pattern[i] = i * 3;
        } else {
            pattern[i] = i * 4;
        }
    }
    return pattern;
}

void transform_pattern(LINEAR_TYPE int *pattern, int length) {
    for (int i = 0; i < length; i++) {
        if (pattern[i] % 2 == 0) {
            pattern[i] /= 2;
        } else {
            pattern[i] = pattern[i] * 2 + 1;
        }
    }
    free(pattern);
}

int find_pattern_value(LINEAR_TYPE int *pattern, int length, int target) {
    for (int i = 0; i < length; i++) {
        if (pattern[i] == target) {
            return i; // Use after free
        }
    }
    return -1;
}

int main() {
    LINEAR_TYPE int *seq = create_pattern(30);
    transform_pattern(seq, 30);
    int pos = find_pattern_value(seq, 30, 15);
    return 0;
}