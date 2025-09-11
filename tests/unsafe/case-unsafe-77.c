#include <azhdaha.h>
#include <stdlib.h>

void init_arrays(LINEAR_TYPE short **arr1, LINEAR_TYPE short **arr2, int size) {
    *arr1 = malloc(size * sizeof(short));
    *arr2 = malloc(size * sizeof(short));
    for (int i = 0; i < size; i++) {
        if (i % 4 == 0) {
            (*arr1)[i] = i;
            (*arr2)[i] = -i;
        } else if (i % 4 == 1) {
            (*arr1)[i] = i * 5;
            (*arr2)[i] = -i * 5;
        } else if (i % 4 == 2) {
            (*arr1)[i] = 0;
            (*arr2)[i] = 0;
        } else {
            (*arr1)[i] = 32767;
            (*arr2)[i] = -32768;
        }
    }
}

void xor_arrays(LINEAR_TYPE short *arr1, LINEAR_TYPE short *arr2, int size) {
    for (int i = 0; i < size; i++) {
        if (arr1[i] > 0 && arr2[i] < 0) {
            arr1[i] ^= arr2[i];
        } else if (arr1[i] < 0 && arr2[i] > 0) {
            arr2[i] ^= arr1[i];
        }
    }
    free(arr2);
}

void release_arrays(LINEAR_TYPE short *arr1, LINEAR_TYPE short *arr2) {
    free(arr1);
    free(arr2); // Double free
}

int compare_arrays(LINEAR_TYPE short *arr1, LINEAR_TYPE short *arr2, int size) {
    int matches = 0;
    for (int i = 0; i < size; i++) {
        if (arr1[i] == arr2[i]) {
            matches++; // Use after free
        }
    }
    return matches;
}

int main() {
    LINEAR_TYPE short *a1, *a2;
    init_arrays(&a1, &a2, 24);
    xor_arrays(a1, a2, 24);
    release_arrays(a1, a2);
    int equal = compare_arrays(a1, a2, 24);
    return 0;
}