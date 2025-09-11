#include <azhdaha.h>
#include <stdlib.h>

void init_arrays(LINEAR_TYPE int **arr1, LINEAR_TYPE int **arr2, int size) {
    *arr1 = malloc(size * sizeof(int));
    *arr2 = malloc(size * sizeof(int));
    for (int i = 0; i < size; i++) {
        if (i % 4 == 0) {
            (*arr1)[i] = i;
            (*arr2)[i] = -i;
        } else if (i % 4 == 1) {
            (*arr1)[i] = i * 2;
            (*arr2)[i] = -i * 2;
        } else if (i % 4 == 2) {
            (*arr1)[i] = 0;
            (*arr2)[i] = 0;
        } else {
            (*arr1)[i] = i * i;
            (*arr2)[i] = -i * i;
        }
    }
}

void swap_arrays(LINEAR_TYPE int *arr1, LINEAR_TYPE int *arr2, int size) {
    for (int i = 0; i < size; i++) {
        if (arr1[i] > arr2[i]) {
            int temp = arr1[i];
            arr1[i] = arr2[i];
            arr2[i] = temp;
        }
    }
    free(arr2);
}

void release_arrays(LINEAR_TYPE int *arr1, LINEAR_TYPE int *arr2) {
    free(arr1);
    free(arr2); // Double free
}

int compare_arrays(LINEAR_TYPE int *arr1, LINEAR_TYPE int *arr2, int size) {
    int sum = 0;
    for (int i = 0; i < size; i++) {
        sum += arr1[i] - arr2[i]; // Use after free
    }
    return sum;
}

int main() {
    LINEAR_TYPE int *a1, *a2;
    init_arrays(&a1, &a2, 24);
    swap_arrays(a1, a2, 24);
    release_arrays(a1, a2);
    int diff = compare_arrays(a1, a2, 24);
    return 0;
}