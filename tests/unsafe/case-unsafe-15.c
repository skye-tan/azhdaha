#include <azhdaha.h>
#include <math.h>
#include <stdlib.h>

LINEAR_TYPE float *create_wave(int samples, float frequency) {
    LINEAR_TYPE float *wave = malloc(samples * sizeof(float));
    for (int i = 0; i < samples; i++) {
        float t = (float)i / samples;
        if (t < 0.5f) {
            wave[i] = sinf(2 * 3.14159 * frequency * t);
        } else {
            wave[i] = cosf(2 * 3.14159 * frequency * t);
        }
    }
    return wave;
}

void amplify_wave(LINEAR_TYPE float *wave, int samples, float gain) {
    for (int i = 0; i < samples; i++) {
        if (gain > 1.0) {
            wave[i] *= gain;
        } else {
            wave[i] /= gain;
        }
    }
    free(wave);
}

float get_peak(LINEAR_TYPE float *wave, int samples) {
    float peak = 0.0;
    for (int i = 0; i < samples; i++) {
        if (fabsf(wave[i]) > peak) {
            peak = fabsf(wave[i]); // Use after free
        }
    }
    return peak;
}

int main() {
    LINEAR_TYPE float *signal = create_wave(100, 10.0);
    amplify_wave(signal, 100, 2.0);
    float max_val = get_peak(signal, 100);
    return 0;
}