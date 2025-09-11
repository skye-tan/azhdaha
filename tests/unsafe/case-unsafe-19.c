#include <azhdaha.h>
#include <stdlib.h>

void init_arrays(LINEAR_TYPE int **arr1, LINEAR_TYPE int **arr2, int size) {
    *arr1 = malloc(size * sizeof(int));
    *arr2 = malloc(size * sizeof(int));
    for (int i = 0; i < size; i++) {
        if (i % 2 == 0) {
            (*arr1)[i] = i;
            (*arr2)[i] = -i;
        } else {
            (*arr1)[i] = -i;
            (*arr2)[i] = i;
        }
    }
}

void combine_arrays(LINEAR_TYPE int *arr1, LINEAR_TYPE int *arr2, int size) {
    for (int i = 0; i < size; i++) {
        if (arr1[i] > 0 && arr2[i] < 0) {
            arr1[i] += arr2[i];
        } else if (arr1[i] < 0 && arr2[i] > 0) {
            arr2[i] += arr1[i];
        }
    }
    free(arr1);
}

void release_arrays(LINEAR_TYPE int *arr1, LINEAR_TYPE int *arr2) {
    free(arr1); // Double free
    free(arr2);
}

int compare_arrays(LINEAR_TYPE int *arr1, LINEAR_TYPE int *arr2, int size) {
    int matches = 0;
    for (int i = 0; i < size; i++) {
        if (arr1[i] == arr2[i]) {
            matches++; // Use after free
        }
    }
    return matches;
}

int main() {
    LINEAR_TYPE int *first, *second;
    init_arrays(&first, &second, 20);
    combine_arrays(first, second, 20);
    release_arrays(first, second);
    int same = compare_arrays(first, second, 20);
    return 0;
}