#include <azhdaha.h>
#include <stdlib.h>

void init_data(LINEAR_TYPE float **data, int size) {
    *data = malloc(size * sizeof(float));
    for (int i = 0; i < size; i++) {
        (*data)[i] = i * 1.5f;
    }
}

void process_data(LINEAR_TYPE float *data) {
    for (int i = 0; i < 20; i++) {
        data[i] = data[i] * 2.0f;
    }
}

void destroy_data(LINEAR_TYPE float *data) { free(data); }

float calculate_average(LINEAR_TYPE float *data, int size) {
    float sum = 0.0f;
    for (int i = 0; i < size; i++) {
        sum += data[i]; // Use after free
    }
    return sum / size;
}

int main() {
    LINEAR_TYPE float *values;
    init_data(&values, 20);
    process_data(values);
    destroy_data(values);
    float avg = calculate_average(values, 20);
    return 0;
}