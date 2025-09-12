#include <azhdaha.h>
#include <math.h>
#include <stdlib.h>

LINEAR_TYPE float *allocate_signal_array(int length) {
    LINEAR_TYPE float *signal = malloc(length * sizeof(float));
    return signal;
}

void generate_sine_wave(float *signal, int length, float frequency) {
    for (int i = 0; i < length; i++) {
        signal[i] =
            sinf(2.0f * 3.14159f * frequency * (float)i / (float)length);
    }
}

float calculate_signal_power(float *signal, int length) {
    float power = 0.0f;
    for (int i = 0; i < length; i++) {
        power += signal[i] * signal[i];
    }
    return power / length;
}

void release_signal_array(LINEAR_TYPE float *signal, int length) {
    float power = calculate_signal_power(signal, length);
    free(signal);
}

int main() {
    LINEAR_TYPE float *wave = allocate_signal_array(100);
    generate_sine_wave(wave, 100, 5.0f);
    release_signal_array(wave, 100);
    return 0;
}