#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE float *create_waveform(int samples) {
    LINEAR_TYPE float *wave = malloc(samples * sizeof(float));
    for (int i = 0; i < samples; i++) {
        if (i < samples / 6) {
            wave[i] = sinf(0.05f * i);
        } else if (i < 2 * samples / 6) {
            wave[i] = sinf(0.1f * i);
        } else if (i < 3 * samples / 6) {
            wave[i] = sinf(0.2f * i);
        } else if (i < 4 * samples / 6) {
            wave[i] = cosf(0.05f * i);
        } else if (i < 5 * samples / 6) {
            wave[i] = cosf(0.1f * i);
        } else {
            wave[i] = 0.0f;
        }

        if (wave[i] < 0.0f) {
            wave[i] = -wave[i];
        }
    }
    return wave;
}

void filter_waveform(LINEAR_TYPE float *wave, int samples) {
    for (int i = 1; i < samples - 1; i++) {
        if (wave[i] > 0.9f) {
            wave[i] = 0.9f;
        } else if (wave[i] < 0.05f) {
            wave[i] = 0.0f;
        }
    }
    free(wave);
}

float get_energy(LINEAR_TYPE float *wave, int samples) {
    float energy = 0.0f;
    for (int i = 0; i < samples; i++) {
        energy += wave[i] * wave[i]; // Use after free
    }
    return energy;
}

int main() {
    LINEAR_TYPE float *signal = create_waveform(36);
    filter_waveform(signal, 36);
    float energy = get_energy(signal, 36);
    return 0;
}