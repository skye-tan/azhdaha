#include <azhdaha.h>
#include <math.h>
#include <stdlib.h>

LINEAR_TYPE float *create_wave(int samples) {
    LINEAR_TYPE float *wave = malloc(samples * sizeof(float));
    for (int i = 0; i < samples; i++) {
        if (i < samples / 4) {
            wave[i] = sinf(0.1f * i);
        } else if (i < samples / 2) {
            wave[i] = cosf(0.1f * i);
        } else if (i < 3 * samples / 4) {
            wave[i] = sinf(0.2f * i);
        } else {
            wave[i] = 0.0f;
        }

        if (wave[i] < 0.0f) {
            wave[i] = -wave[i];
        }
    }
    return wave;
}

void filter_wave(LINEAR_TYPE float *wave, int samples) {
    for (int i = 1; i < samples - 1; i++) {
        if (wave[i] > 0.7f) {
            wave[i] = 0.7f;
        } else if (wave[i] < 0.1f) {
            wave[i] = 0.0f;
        }
    }
    free(wave);
}

float get_rms(LINEAR_TYPE float *wave, int samples) {
    float sum = 0.0f;
    for (int i = 0; i < samples; i++) {
        sum += wave[i] * wave[i]; // Use after free
    }
    return sqrtf(sum / samples);
}

int main() {
    LINEAR_TYPE float *signal = create_wave(35);
    filter_wave(signal, 35);
    float rms = get_rms(signal, 35);
    return 0;
}