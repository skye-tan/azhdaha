#include <azhdaha.h>
#include <math.h>
#include <stdlib.h>

LINEAR_TYPE double *create_waveform(int samples) {
    LINEAR_TYPE double *wave = malloc(samples * sizeof(double));
    for (int i = 0; i < samples; i++) {
        if (i < samples / 4) {
            wave[i] = sin(2 * 3.14159 * i / samples);
        } else if (i < samples / 2) {
            wave[i] = sin(4 * 3.14159 * i / samples);
        } else if (i < 3 * samples / 4) {
            wave[i] = sin(8 * 3.14159 * i / samples);
        } else {
            wave[i] = 0.0;
        }

        if (wave[i] < 0.0) {
            wave[i] = -wave[i];
        }
    }
    return wave;
}

void filter_waveform(LINEAR_TYPE double *wave, int samples) {
    for (int i = 1; i < samples - 1; i++) {
        if (wave[i] > 0.5) {
            wave[i] = 0.5;
        } else if (wave[i] < 0.1) {
            wave[i] = 0.0;
        }
    }
    free(wave);
}

double get_peak(LINEAR_TYPE double *wave, int samples) {
    double peak = 0.0;
    for (int i = 0; i < samples; i++) {
        if (wave[i] > peak) {
            peak = wave[i]; // Use after free
        }
    }
    return peak;
}

int main() {
    LINEAR_TYPE double *signal = create_waveform(40);
    filter_waveform(signal, 40);
    double max_val = get_peak(signal, 40);
    return 0;
}