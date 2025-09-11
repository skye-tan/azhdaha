#include <azhdaha.h>
#include <stdlib.h>

void init_arrays(LINEAR_TYPE int **positive, LINEAR_TYPE int **negative,
                 int size) {
    *positive = malloc(size * sizeof(int));
    *negative = malloc(size * sizeof(int));
    for (int i = 0; i < size; i++) {
        if (i % 2 == 0) {
            (*positive)[i] = i;
            (*negative)[i] = -i;
        } else {
            (*positive)[i] = -i;
            (*negative)[i] = i;
        }
    }
}

void process_arrays(LINEAR_TYPE int *positive, LINEAR_TYPE int *negative,
                    int size) {
    for (int i = 0; i < size; i++) {
        if (positive[i] > 0) {
            positive[i] *= 2;
        }
        if (negative[i] < 0) {
            negative[i] *= 2;
        }
    }
    free(positive);
}

void cleanup_arrays(LINEAR_TYPE int *positive, LINEAR_TYPE int *negative) {
    free(positive); // Double free
    free(negative);
}

int compare_elements(LINEAR_TYPE int *positive, LINEAR_TYPE int *negative,
                     int size) {
    int matches = 0;
    for (int i = 0; i < size; i++) {
        if (positive[i] == negative[i]) {
            matches++; // Use after free
        }
    }
    return matches;
}

int main() {
    LINEAR_TYPE int *pos_arr, *neg_arr;
    init_arrays(&pos_arr, &neg_arr, 25);
    process_arrays(pos_arr, neg_arr, 25);
    cleanup_arrays(pos_arr, neg_arr);
    int equal = compare_elements(pos_arr, neg_arr, 25);
    return 0;
}