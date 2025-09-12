#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *initialize_array(int size) {
    LINEAR_TYPE int *arr = malloc(size * sizeof(int));
    return arr;
}

void populate_array(int *arr, int size) {
    for (int i = 0; i < size; i++) {
        arr[i] = i * 2;
    }
}

int calculate_sum(int *arr, int size) {
    int sum = 0;
    for (int i = 0; i < size; i++) {
        sum += arr[i];
    }
    return sum;
}

void release_array(LINEAR_TYPE int *arr, int size) {
    int total = calculate_sum(arr, size);
    free(arr);
}

int main() {
    LINEAR_TYPE int *data = initialize_array(10);
    populate_array(data, 10);
    release_array(data, 10);
    return 0;
}