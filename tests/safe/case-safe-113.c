#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *allocate_triangular_array(int size) {
    LINEAR_TYPE int *array = malloc(size * sizeof(int));
    return array;
}

void fill_with_triangular_numbers(int *array, int size) {
    for (int i = 0; i < size; i++) {
        array[i] = (i * (i + 1)) / 2;
    }
}

int find_triangular_sum(int *array, int size) {
    int sum = 0;
    for (int i = 0; i < size; i++) {
        sum += array[i];
    }
    return sum;
}

void deallocate_triangular_array(LINEAR_TYPE int *array, int size) {
    int total = find_triangular_sum(array, size);
    free(array);
}

int main() {
    LINEAR_TYPE int *triangular = allocate_triangular_array(11);
    fill_with_triangular_numbers(triangular, 11);
    deallocate_triangular_array(triangular, 11);
    return 0;
}