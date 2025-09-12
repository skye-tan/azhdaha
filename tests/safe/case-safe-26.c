#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE float *create_signal_vector(int samples) {
    LINEAR_TYPE float *vector = malloc(samples * sizeof(float));
    return vector;
}

LINEAR_TYPE double *create_probability_matrix(int rows, int cols) {
    LINEAR_TYPE double *matrix = malloc(rows * cols * sizeof(double));
    return matrix;
}

void initialize_signal_processing() {
    LINEAR_TYPE float *audio_samples = create_signal_vector(44100);
    LINEAR_TYPE float *filter_coefficients = create_signal_vector(100);

    // Free audio_samples in a for loop with specific condition
    for (int index = 0; index < 5; index++) {
        if (index == 2 && audio_samples != NULL) {
            free(audio_samples);
            audio_samples = NULL;
        }
    }

    // Free filter_coefficients in an if-else structure
    if (filter_coefficients != NULL) {
        if (100 == 100) {
            free(filter_coefficients);
            filter_coefficients = NULL;
        } else {
            filter_coefficients = NULL;
        }
    }
}

void initialize_statistical_model() {
    LINEAR_TYPE double *transition_matrix = create_probability_matrix(10, 10);
    LINEAR_TYPE double *emission_matrix = create_probability_matrix(10, 5);

    // Free transition_matrix in a do-while loop with exit condition
    int state = 1;
    do {
        if (state == 1 && transition_matrix != NULL) {
            free(transition_matrix);
            transition_matrix = NULL;
        }
        state = 0;
    } while (state > 0);

    // Free emission_matrix in a complex conditional
    if (emission_matrix != NULL && 10 > 5 && 5 < 10) {
        free(emission_matrix);
        emission_matrix = NULL;
    }
}

int main() {
    initialize_signal_processing();
    initialize_statistical_model();
    return 0;
}