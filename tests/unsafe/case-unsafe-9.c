#include <azhdaha.h>
#include <stdlib.h>

void init_values(LINEAR_TYPE int **arr, int size) {
    *arr = malloc(size * sizeof(int));
    for (int i = 0; i < size; i++) {
        if (i % 2 == 0) {
            (*arr)[i] = i * 2;
        } else {
            (*arr)[i] = i * 3;
        }
    }
}

void process_values(LINEAR_TYPE int *arr, int size) {
    for (int i = 0; i < size; i++) {
        if (arr[i] > 10) {
            arr[i] /= 2;
        } else {
            arr[i] *= 2;
        }
    }
    free(arr);
}

int find_max(LINEAR_TYPE int *arr, int size) {
    int max = arr[0];
    for (int i = 1; i < size; i++) {
        if (arr[i] > max) {
            max = arr[i]; // Use after free
        }
    }
    return max;
}

int main() {
    LINEAR_TYPE int *data;
    init_values(&data, 15);
    process_values(data, 15);
    int maximum = find_max(data, 15);
    return 0;
}