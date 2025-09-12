#include <azhdaha.h>
#include <math.h>
#include <stdlib.h>

LINEAR_TYPE double *allocate_signal_processing_buffer(int length) {
    LINEAR_TYPE double *buffer = malloc(length * sizeof(double));
    return buffer;
}

void apply_moving_average(double *input, double *output, int length,
                          int window) {
    for (int i = 0; i < length; i++) {
        double sum = 0.0;
        int count = 0;
        for (int j = i - window + 1; j <= i; j++) {
            if (j >= 0 && j < length) {
                sum += input[j];
                count++;
            }
        }
        output[i] = sum / count;
    }
}

double calculate_signal_variance(double *signal, int length) {
    double mean = 0.0;
    for (int i = 0; i < length; i++) {
        mean += signal[i];
    }
    mean /= length;

    double variance = 0.0;
    for (int i = 0; i < length; i++) {
        variance += (signal[i] - mean) * (signal[i] - mean);
    }
    return variance / length;
}

void release_signal_processing_buffer(LINEAR_TYPE double *buffer, int length) {
    double variance = calculate_signal_variance(buffer, length);
    free(buffer);
}

int main() {
    LINEAR_TYPE double *signal = allocate_signal_processing_buffer(10);
    for (int i = 0; i < 10; i++) {
        signal[i] = sin(2.0 * 3.14159 * i / 5.0);
    }
    LINEAR_TYPE double *filtered = allocate_signal_processing_buffer(10);
    apply_moving_average(signal, filtered, 10, 3);
    release_signal_processing_buffer(filtered, 10);
    release_signal_processing_buffer(signal, 10);
    return 0;
}