#include <azhdaha.h>
#include <math.h>
#include <stdlib.h>

LINEAR_TYPE float *allocate_dsp_buffer(int length) {
    LINEAR_TYPE float *buffer = malloc(length * sizeof(float));
    return buffer;
}

void apply_window_function(float *buffer, int length) {
    for (int i = 0; i < length; i++) {
        float window = 0.5f * (1.0f - cosf(2.0f * 3.14159f * i / (length - 1)));
        buffer[i] *= window;
    }
}

float calculate_rms(float *buffer, int length) {
    float sum = 0.0f;
    for (int i = 0; i < length; i++) {
        sum += buffer[i] * buffer[i];
    }
    return sqrtf(sum / length);
}

void release_dsp_buffer(LINEAR_TYPE float *buffer, int length) {
    float rms = calculate_rms(buffer, length);
    free(buffer);
}

int main() {
    LINEAR_TYPE float *signal = allocate_dsp_buffer(100);
    for (int i = 0; i < 100; i++) {
        signal[i] = sinf(2.0f * 3.14159f * i / 10.0f);
    }
    apply_window_function(signal, 100);
    release_dsp_buffer(signal, 100);
    return 0;
}