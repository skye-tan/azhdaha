#include <azhdaha.h>
#include <stdlib.h>

void init_arrays(LINEAR_TYPE short int **arr1, LINEAR_TYPE short int **arr2,
                 int size) {
    *arr1 = malloc(size * sizeof(short int));
    *arr2 = malloc(size * sizeof(short int));
    for (int i = 0; i < size; i++) {
        if (i % 5 == 0) {
            (*arr1)[i] = i;
            (*arr2)[i] = -i;
        } else if (i % 5 == 1) {
            (*arr1)[i] = i * 10;
            (*arr2)[i] = -i * 10;
        } else if (i % 5 == 2) {
            (*arr1)[i] = 0;
            (*arr2)[i] = 0;
        } else if (i % 5 == 3) {
            (*arr1)[i] = 32767;
            (*arr2)[i] = -32768;
        } else {
            (*arr1)[i] = i * 100;
            (*arr2)[i] = -i * 100;
        }
    }
}

void combine_arrays(LINEAR_TYPE short int *arr1, LINEAR_TYPE short int *arr2,
                    int size) {
    for (int i = 0; i < size; i++) {
        if (arr1[i] > 0 && arr2[i] < 0) {
            arr1[i] += arr2[i];
        } else if (arr1[i] < 0 && arr2[i] > 0) {
            arr2[i] += arr1[i];
        }
    }
    free(arr2);
}

void release_arrays(LINEAR_TYPE short int *arr1, LINEAR_TYPE short int *arr2) {
    free(arr1);
    free(arr2); // Double free
}

short int find_difference(LINEAR_TYPE short int *arr1,
                          LINEAR_TYPE short int *arr2, int size) {
    for (int i = 0; i < size; i++) {
        if (arr1[i] != arr2[i]) {
            return arr1[i] - arr2[i]; // Use after free
        }
    }
    return 0;
}

int main() {
    LINEAR_TYPE short int *a1, *a2;
    init_arrays(&a1, &a2, 25);
    combine_arrays(a1, a2, 25);
    release_arrays(a1, a2);
    short int diff = find_difference(a1, a2, 25);
    return 0;
}