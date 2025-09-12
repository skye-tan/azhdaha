#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE float *create_float_array(int count) {
    LINEAR_TYPE float *array = malloc(count * sizeof(float));
    return array;
}

void fill_float_array(float *array, int count) {
    for (int i = 0; i < count; i++) {
        array[i] = i * 1.5f;
    }
}

float find_average(float *array, int count) {
    float sum = 0.0f;
    for (int i = 0; i < count; i++) {
        sum += array[i];
    }
    return sum / count;
}

void dispose_float_array(LINEAR_TYPE float *array, int count) {
    float avg = find_average(array, count);
    free(array);
}

int main() {
    LINEAR_TYPE float *numbers = create_float_array(8);
    fill_float_array(numbers, 8);
    dispose_float_array(numbers, 8);
    return 0;
}