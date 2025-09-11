#include <azhdaha.h>
#include <math.h>
#include <stdlib.h>

LINEAR_TYPE double *create_signal(int length) {
    LINEAR_TYPE double *signal = malloc(length * sizeof(double));
    for (int i = 0; i < length; i++) {
        if (i < length / 2) {
            signal[i] = sin(0.1 * i);
        } else {
            signal[i] = cos(0.1 * i);
        }
    }
    return signal;
}

void filter_signal(LINEAR_TYPE double *signal, int length) {
    for (int i = 1; i < length - 1; i++) {
        if (signal[i] > 0.5) {
            signal[i] = 0.5;
        } else if (signal[i] < -0.5) {
            signal[i] = -0.5;
        }
    }
    free(signal);
}

double get_average(LINEAR_TYPE double *signal, int length) {
    double sum = 0.0;
    for (int i = 0; i < length; i++) {
        if (signal[i] != 0.0) {
            sum += signal[i]; // Use after free
        }
    }
    return sum / length;
}

int main() {
    LINEAR_TYPE double *wave = create_signal(50);
    filter_signal(wave, 50);
    double avg = get_average(wave, 50);
    return 0;
}