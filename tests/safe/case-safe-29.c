#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *allocate_array(int size) {
    LINEAR_TYPE int *arr = malloc(size * sizeof(int));
    return arr;
}

void initialize_array(int *arr, int size) {
    for (int i = 0; i < size; i++) {
        arr[i] = i * 2;
    }
}

int sum_array(LINEAR_TYPE int *arr, int size) {
    int sum = 0;
    for (int i = 0; i < size; i++) {
        sum += arr[i];
    }
    free(arr);
    return sum;
}

int main() {
    LINEAR_TYPE int *numbers = allocate_array(10);
    initialize_array(numbers, 10);
    int result = sum_array(numbers, 10);
    return 0;
}