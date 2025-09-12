#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE float *allocate_float_array(int size) {
    LINEAR_TYPE float *array = malloc(size * sizeof(float));
    return array;
}

void initialize_float_array(float *array, int size) {
    for (int i = 0; i < size; i++) {
        array[i] = (float)i / 10.0f;
    }
}

float calculate_float_average(float *array, int size) {
    float sum = 0.0f;
    for (int i = 0; i < size; i++) {
        sum += array[i];
    }
    return sum / size;
}

void deallocate_float_array(LINEAR_TYPE float *array, int size) {
    float average = calculate_float_average(array, size);
    free(array);
}

int main() {
    LINEAR_TYPE float *floats = allocate_float_array(5);
    initialize_float_array(floats, 5);
    deallocate_float_array(floats, 5);
    return 0;
}