#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_multiples(int base, int count) {
    LINEAR_TYPE int *multiples = malloc(count * sizeof(int));
    for (int i = 0; i < count; i++) {
        multiples[i] = base * (i + 1);
    }
    return multiples;
}

void filter_multiples(LINEAR_TYPE int *multiples, int count, int threshold) {
    for (int i = 0; i < count; i++) {
        if (multiples[i] > threshold) {
            multiples[i] = 0;
        } else if (multiples[i] < 0) {
            multiples[i] = -1;
        }
    }
    free(multiples);
}

int count_positive(LINEAR_TYPE int *multiples, int count) {
    int positive = 0;
    for (int i = 0; i < count; i++) {
        if (multiples[i] > 0) {
            positive++; // Use after free
        }
    }
    return positive;
}

int main() {
    LINEAR_TYPE int *numbers = create_multiples(7, 15);
    filter_multiples(numbers, 15, 50);
    int pos_count = count_positive(numbers, 15);
    return 0;
}