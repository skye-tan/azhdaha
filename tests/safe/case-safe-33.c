#include <azhdaha.h>
#include <math.h>
#include <stdlib.h>

LINEAR_TYPE float *create_filter_coefficients(int taps) {
    LINEAR_TYPE float *coeffs = malloc(taps * sizeof(float));
    return coeffs;
}

void design_lowpass_filter(float *coeffs, int taps, float cutoff) {
    for (int i = 0; i < taps; i++) {
        float x = (float)(i - taps / 2);
        if (x == 0) {
            coeffs[i] = 2.0f * cutoff;
        } else {
            coeffs[i] = sinf(2.0f * 3.14159f * cutoff * x) / (3.14159f * x);
        }
    }
}

float apply_filter(float *coeffs, int taps, float *signal, int signal_length) {
    float output = 0.0f;
    for (int i = 0; i < taps && i < signal_length; i++) {
        output += coeffs[i] * signal[i];
    }
    return output;
}

void release_filter_coefficients(LINEAR_TYPE float *coeffs, int taps) {
    float filtered_value = apply_filter(coeffs, taps, NULL, 0);
    free(coeffs);
}

int main() {
    LINEAR_TYPE float *filter_coeffs = create_filter_coefficients(11);
    design_lowpass_filter(filter_coeffs, 11, 0.2f);
    release_filter_coefficients(filter_coeffs, 11);
    return 0;
}