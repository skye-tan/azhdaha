#include <azhdaha.h>
#include <stdlib.h>

void init_arrays(LINEAR_TYPE char **arr1, LINEAR_TYPE char **arr2, int size) {
    *arr1 = malloc(size);
    *arr2 = malloc(size);
    for (int i = 0; i < size - 1; i++) {
        if (i % 4 == 0) {
            (*arr1)[i] = 'A' + (i % 26);
            (*arr2)[i] = 'a' + (i % 26);
        } else if (i % 4 == 1) {
            (*arr1)[i] = '0' + (i % 10);
            (*arr2)[i] = ' ';
        } else if (i % 4 == 2) {
            (*arr1)[i] = '+';
            (*arr2)[i] = '-';
        } else {
            (*arr1)[i] = '*';
            (*arr2)[i] = '/';
        }
    }
    (*arr1)[size - 1] = '\0';
    (*arr2)[size - 1] = '\0';
}

void merge_arrays(LINEAR_TYPE char *arr1, LINEAR_TYPE char *arr2, int size) {
    for (int i = 0; i < size; i++) {
        if (arr1[i] >= 'A' && arr1[i] <= 'Z' && arr2[i] >= 'a' &&
            arr2[i] <= 'z') {
            arr1[i] = arr1[i] + 32;
            arr2[i] = arr2[i] - 32;
        } else if (arr1[i] == ' ' || arr2[i] == ' ') {
            arr1[i] = '_';
            arr2[i] = '_';
        }
    }
    free(arr2);
}

void release_arrays(LINEAR_TYPE char *arr1, LINEAR_TYPE char *arr2) {
    free(arr1);
    free(arr2); // Double free
}

int compare_arrays(LINEAR_TYPE char *arr1, LINEAR_TYPE char *arr2) {
    int i = 0;
    while (arr1[i] != '\0' && arr2[i] != '\0') {
        if (arr1[i] != arr2[i]) {
            return i; // Use after free
        }
        i++;
    }
    return -1;
}

int main() {
    LINEAR_TYPE char *a1, *a2;
    init_arrays(&a1, &a2, 25);
    merge_arrays(a1, a2, 25);
    release_arrays(a1, a2);
    int diff_pos = compare_arrays(a1, a2);
    return 0;
}