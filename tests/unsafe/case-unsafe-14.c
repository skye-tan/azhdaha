#include <azhdaha.h>
#include <stdlib.h>

void init_array(LINEAR_TYPE int **arr, int size) {
    *arr = malloc(size * sizeof(int));
    for (int i = 0; i < size; i++) {
        if (i < size / 3) {
            (*arr)[i] = i;
        } else if (i < 2 * size / 3) {
            (*arr)[i] = i * 2;
        } else {
            (*arr)[i] = i * 3;
        }
    }
}

void filter_array(LINEAR_TYPE int *arr, int size) {
    for (int i = 0; i < size; i++) {
        if (arr[i] % 2 != 0) {
            arr[i] = 0;
        }
    }
    free(arr);
}

int count_nonzero(LINEAR_TYPE int *arr, int size) {
    int count = 0;
    for (int i = 0; i < size; i++) {
        if (arr[i] != 0) {
            count++; // Use after free
        }
    }
    return count;
}

int main() {
    LINEAR_TYPE int *data;
    init_array(&data, 30);
    filter_array(data, 30);
    int nonzero = count_nonzero(data, 30);
    return 0;
}