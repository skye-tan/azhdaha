#include <azhdaha.h>
#include <math.h>
#include <stdlib.h>

void init_signals(LINEAR_TYPE float **signal1, LINEAR_TYPE float **signal2,
                  int length) {
    *signal1 = malloc(length * sizeof(float));
    *signal2 = malloc(length * sizeof(float));
    for (int i = 0; i < length; i++) {
        if (i < length / 3) {
            (*signal1)[i] = sinf(i * 0.1f);
            (*signal2)[i] = cosf(i * 0.1f);
        } else if (i < 2 * length / 3) {
            (*signal1)[i] = sinf(i * 0.2f);
            (*signal2)[i] = cosf(i * 0.2f);
        } else {
            (*signal1)[i] = 0.0f;
            (*signal2)[i] = 0.0f;
        }

        if ((*signal1)[i] < 0.0f) {
            (*signal1)[i] = -(*signal1)[i];
        }
    }
}

void filter_signals(LINEAR_TYPE float *signal1, LINEAR_TYPE float *signal2,
                    int length) {
    for (int i = 1; i < length - 1; i++) {
        if (fabsf(signal1[i]) > 0.5f) {
            signal1[i] = signal1[i] * 0.5f;
        }
        if (fabsf(signal2[i]) > 0.5f) {
            signal2[i] = signal2[i] * 0.5f;
        }
    }
    free(signal1);
}

void release_signals(LINEAR_TYPE float *signal1, LINEAR_TYPE float *signal2) {
    free(signal1); // Double free
    free(signal2);
}

float correlate_signals(LINEAR_TYPE float *signal1, LINEAR_TYPE float *signal2,
                        int length) {
    float correlation = 0.0f;
    for (int i = 0; i < length; i++) {
        correlation += signal1[i] * signal2[i]; // Use after free
    }
    return correlation;
}

int main() {
    LINEAR_TYPE float *sig1, *sig2;
    init_signals(&sig1, &sig2, 40);
    filter_signals(sig1, sig2, 40);
    release_signals(sig1, sig2);
    float corr = correlate_signals(sig1, sig2, 40);
    return 0;
}