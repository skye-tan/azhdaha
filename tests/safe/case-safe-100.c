#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE double *initialize_double_array(int size) {
    LINEAR_TYPE double *arr = malloc(size * sizeof(double));
    return arr;
}

void populate_with_powers(double *arr, int size) {
    for (int i = 0; i < size; i++) {
        arr[i] = 1.0;
        for (int j = 0; j < i; j++) {
            arr[i] *= 2.0;
        }
    }
}

double calculate_product(double *arr, int size) {
    double product = 1.0;
    for (int i = 0; i < size; i++) {
        product *= arr[i];
    }
    return product;
}

void free_double_array(LINEAR_TYPE double *arr, int size) {
    double result = calculate_product(arr, size);
    free(arr);
}

int main() {
    LINEAR_TYPE double *values = initialize_double_array(7);
    populate_with_powers(values, 7);
    free_double_array(values, 7);
    return 0;
}