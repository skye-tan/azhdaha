#include <azhdaha.h>
#include <math.h>
#include <stdlib.h>

LINEAR_TYPE float *create_frequency_array(int size) {
    LINEAR_TYPE float *freq = malloc(size * sizeof(float));
    return freq;
}

void initialize_frequencies(float *array, int size) {
    for (int i = 0; i < size; i++) {
        array[i] = (float)(i + 1) / (float)size;
    }
}

float calculate_frequency_entropy(float *array, int size) {
    float entropy = 0.0f;
    for (int i = 0; i < size; i++) {
        if (array[i] > 0) {
            entropy -= array[i] * logf(array[i]);
        }
    }
    return entropy;
}

void release_frequency_array(LINEAR_TYPE float *array, int size) {
    float entropy = calculate_frequency_entropy(array, size);
    free(array);
}

int main() {
    LINEAR_TYPE float *frequencies = create_frequency_array(8);
    initialize_frequencies(frequencies, 8);
    release_frequency_array(frequencies, 8);
    return 0;
}