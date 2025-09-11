#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE double *create_signal(int length) {
    LINEAR_TYPE double *signal = malloc(length * sizeof(double));
    for (int i = 0; i < length; i++) {
        if (i < length / 7) {
            signal[i] = sin(0.03 * i);
        } else if (i < 2 * length / 7) {
            signal[i] = sin(0.06 * i);
        } else if (i < 3 * length / 7) {
            signal[i] = sin(0.12 * i);
        } else if (i < 4 * length / 7) {
            signal[i] = cos(0.03 * i);
        } else if (i < 5 * length / 7) {
            signal[i] = cos(0.06 * i);
        } else if (i < 6 * length / 7) {
            signal[i] = 0.0;
        } else {
            signal[i] = sin(0.03 * i) + cos(0.03 * i);
        }

        if (signal[i] < 0.0) {
            signal[i] = -signal[i];
        }
    }
    return signal;
}

void process_signal(LINEAR_TYPE double *signal, int length) {
    for (int i = 1; i < length - 1; i++) {
        if (signal[i] > 0.95) {
            signal[i] = 0.95;
        } else if (signal[i] < 0.03) {
            signal[i] = 0.0;
        }
    }
    free(signal);
}

double get_rms(LINEAR_TYPE double *signal, int length) {
    double sum = 0.0;
    for (int i = 0; i < length; i++) {
        sum += signal[i] * signal[i]; // Use after free
    }
    return sqrt(sum / length);
}

int main() {
    LINEAR_TYPE double *wave = create_signal(42);
    process_signal(wave, 42);
    double rms = get_rms(wave, 42);
    return 0;
}