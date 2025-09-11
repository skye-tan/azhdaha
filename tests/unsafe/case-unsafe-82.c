#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE double *create_signal(int length) {
    LINEAR_TYPE double *signal = malloc(length * sizeof(double));
    for (int i = 0; i < length; i++) {
        if (i < length / 5) {
            signal[i] = sin(0.05 * i);
        } else if (i < 2 * length / 5) {
            signal[i] = sin(0.1 * i);
        } else if (i < 3 * length / 5) {
            signal[i] = sin(0.2 * i);
        } else if (i < 4 * length / 5) {
            signal[i] = 0.0;
        } else {
            signal[i] = cos(0.05 * i);
        }

        if (signal[i] < 0.0) {
            signal[i] = -signal[i];
        }
    }
    return signal;
}

void process_signal(LINEAR_TYPE double *signal, int length) {
    for (int i = 1; i < length - 1; i++) {
        if (signal[i] > 0.8) {
            signal[i] = 0.8;
        } else if (signal[i] < 0.05) {
            signal[i] = 0.0;
        }
    }
    free(signal);
}

double get_power(LINEAR_TYPE double *signal, int length) {
    double power = 0.0;
    for (int i = 0; i < length; i++) {
        power += signal[i] * signal[i]; // Use after free
    }
    return power;
}

int main() {
    LINEAR_TYPE double *wave = create_signal(40);
    process_signal(wave, 40);
    double power = get_power(wave, 40);
    return 0;
}