#include <azhdaha.h>
#include <stdlib.h>

void init_arrays(LINEAR_TYPE long **arr1, LINEAR_TYPE long **arr2, int size) {
    *arr1 = malloc(size * sizeof(long));
    *arr2 = malloc(size * sizeof(long));
    for (int i = 0; i < size; i++) {
        if (i % 4 == 0) {
            (*arr1)[i] = i * 1000L;
            (*arr2)[i] = -i * 1000L;
        } else if (i % 4 == 1) {
            (*arr1)[i] = i * 100L;
            (*arr2)[i] = -i * 100L;
        } else if (i % 4 == 2) {
            (*arr1)[i] = 0L;
            (*arr2)[i] = 0L;
        } else {
            (*arr1)[i] = i * 10L;
            (*arr2)[i] = -i * 10L;
        }
    }
}

void combine_arrays(LINEAR_TYPE long *arr1, LINEAR_TYPE long *arr2, int size) {
    for (int i = 0; i < size; i++) {
        if (arr1[i] > 0L && arr2[i] < 0L) {
            long temp = arr1[i];
            arr1[i] = arr2[i];
            arr2[i] = temp;
        } else if (arr1[i] == 0L) {
            arr1[i] = 1L;
        } else if (arr2[i] == 0L) {
            arr2[i] = -1L;
        }
    }
    free(arr1);
}

void release_arrays(LINEAR_TYPE long *arr1, LINEAR_TYPE long *arr2) {
    free(arr1); // Double free
    free(arr2);
}

long find_sum(LINEAR_TYPE long *arr1, LINEAR_TYPE long *arr2, int size) {
    long sum = 0L;
    for (int i = 0; i < size; i++) {
        sum += arr1[i] + arr2[i]; // Use after free
    }
    return sum;
}

int main() {
    LINEAR_TYPE long *a1, *a2;
    init_arrays(&a1, &a2, 21);
    combine_arrays(a1, a2, 21);
    release_arrays(a1, a2);
    long total = find_sum(a1, a2, 21);
    return 0;
}