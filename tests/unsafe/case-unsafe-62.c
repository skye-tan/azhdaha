#include <azhdaha.h>
#include <math.h>
#include <stdlib.h>

LINEAR_TYPE double *create_signal(int length) {
    LINEAR_TYPE double *signal = malloc(length * sizeof(double));
    for (int i = 0; i < length; i++) {
        if (i < length / 3) {
            signal[i] = sin(0.1 * i);
        } else if (i < 2 * length / 3) {
            signal[i] = cos(0.1 * i);
        } else {
            signal[i] = 0.0;
        }

        if (signal[i] < 0.0) {
            signal[i] = -signal[i];
        }
    }
    return signal;
}

void filter_signal(LINEAR_TYPE double *signal, int length) {
    for (int i = 1; i < length - 1; i++) {
        if (signal[i] > 0.5) {
            signal[i] = 0.5;
        } else if (signal[i] < 0.1) {
            signal[i] = 0.0;
        }
    }
    free(signal);
}

double get_energy(LINEAR_TYPE double *signal, int length) {
    double energy = 0.0;
    for (int i = 0; i < length; i++) {
        energy += signal[i] * signal[i]; // Use after free
    }
    return energy;
}

int main() {
    LINEAR_TYPE double *wave = create_signal(35);
    filter_signal(wave, 35);
    double energy = get_energy(wave, 35);
    return 0;
}