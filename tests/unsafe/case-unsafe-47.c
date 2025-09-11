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
            (*arr1)[i] = i * 2;
            (*arr2)[i] = -i * 2;
        } else if (i % 5 == 2) {
            (*arr1)[i] = i * 3;
            (*arr2)[i] = -i * 3;
        } else if (i % 5 == 3) {
            (*arr1)[i] = 0;
            (*arr2)[i] = 0;
        } else {
            (*arr1)[i] = 32767;
            (*arr2)[i] = -32768;
        }
    }
}

void process_arrays(LINEAR_TYPE short int *arr1, LINEAR_TYPE short int *arr2,
                    int size) {
    for (int i = 0; i < size; i++) {
        if (arr1[i] > 0 && arr2[i] < 0) {
            short int temp = arr1[i];
            arr1[i] = arr2[i];
            arr2[i] = temp;
        } else if (arr1[i] == 0) {
            arr1[i] = 1;
        } else if (arr2[i] == 0) {
            arr2[i] = -1;
        }
    }
    // Free arr2
    free(arr2);
}

void release_arrays(LINEAR_TYPE short int *arr1, LINEAR_TYPE short int *arr2) {
    free(arr1);
    free(arr2); // Double free
}

short int find_maximum(LINEAR_TYPE short int *arr1, LINEAR_TYPE short int *arr2,
                       int size) {
    short int max = arr1[0];
    for (int i = 0; i < size; i++) {
        if (arr1[i] > max) {
            max = arr1[i]; // Use after free
        }
        if (arr2[i] > max) {
            max = arr2[i]; // Use after free
        }
    }
    return max;
}

int main() {
    LINEAR_TYPE short int *a1, *a2;
    init_arrays(&a1, &a2, 25);
    process_arrays(a1, a2, 25);
    release_arrays(a1, a2);
    short int maximum = find_maximum(a1, a2, 25);
    return 0;
}