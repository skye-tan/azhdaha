#include <azhdaha.h>
#include <stdlib.h>

void init_arrays(LINEAR_TYPE int **arr1, LINEAR_TYPE int **arr2,
                 LINEAR_TYPE int **arr3, int size) {
    *arr1 = malloc(size * sizeof(int));
    *arr2 = malloc(size * sizeof(int));
    *arr3 = malloc(size * sizeof(int));
    for (int i = 0; i < size; i++) {
        if (i % 4 == 0) {
            (*arr1)[i] = i;
            (*arr2)[i] = i * 2;
            (*arr3)[i] = i * 3;
        } else if (i % 4 == 1) {
            (*arr1)[i] = -i;
            (*arr2)[i] = -i * 2;
            (*arr3)[i] = -i * 3;
        } else if (i % 4 == 2) {
            (*arr1)[i] = 0;
            (*arr2)[i] = 0;
            (*arr3)[i] = 0;
        } else {
            (*arr1)[i] = i * i;
            (*arr2)[i] = -i * i;
            (*arr3)[i] = i * i * i;
        }
    }
}

void process_arrays(LINEAR_TYPE int *arr1, LINEAR_TYPE int *arr2,
                    LINEAR_TYPE int *arr3, int size) {
    for (int i = 0; i < size; i++) {
        if (arr1[i] > arr2[i] && arr1[i] > arr3[i]) {
            arr2[i] = arr3[i] = arr1[i];
        } else if (arr2[i] > arr1[i] && arr2[i] > arr3[i]) {
            arr1[i] = arr3[i] = arr2[i];
        }
    }
    free(arr2);
}

void release_arrays(LINEAR_TYPE int *arr1, LINEAR_TYPE int *arr2,
                    LINEAR_TYPE int *arr3) {
    free(arr1);
    free(arr2); // Double free
    free(arr3);
}

int find_minimum(LINEAR_TYPE int *arr1, LINEAR_TYPE int *arr2,
                 LINEAR_TYPE int *arr3, int size) {
    int min = arr1[0];
    for (int i = 1; i < size; i++) {
        if (arr1[i] < min)
            min = arr1[i]; // Use after free
        if (arr2[i] < min)
            min = arr2[i]; // Use after free
        if (arr3[i] < min)
            min = arr3[i]; // Use after free
    }
    return min;
}

int main() {
    LINEAR_TYPE int *a1, *a2, *a3;
    init_arrays(&a1, &a2, &a3, 20);
    process_arrays(a1, a2, a3, 20);
    release_arrays(a1, a2, a3);
    int minimum = find_minimum(a1, a2, a3, 20);
    return 0;
}