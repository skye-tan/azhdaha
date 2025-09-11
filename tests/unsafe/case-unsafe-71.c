#include <azhdaha.h>
#include <stdlib.h>

void init_data(LINEAR_TYPE float **data1, LINEAR_TYPE float **data2, int size) {
    *data1 = malloc(size * sizeof(float));
    *data2 = malloc(size * sizeof(float));
    for (int i = 0; i < size; i++) {
        if (i % 4 == 0) {
            (*data1)[i] = (float)i;
            (*data2)[i] = (float)(-i);
        } else if (i % 4 == 1) {
            (*data1)[i] = (float)(i * 1.5);
            (*data2)[i] = (float)(-i * 1.5);
        } else if (i % 4 == 2) {
            (*data1)[i] = 0.0f;
            (*data2)[i] = 0.0f;
        } else {
            (*data1)[i] = 1.0f;
            (*data2)[i] = -1.0f;
        }
    }
}

void process_data(LINEAR_TYPE float *data1, LINEAR_TYPE float *data2,
                  int size) {
    for (int i = 0; i < size; i++) {
        if (data1[i] > 0.0f && data2[i] < 0.0f) {
            float temp = data1[i];
            data1[i] = data2[i];
            data2[i] = temp;
        } else if (data1[i] == 0.0f) {
            data1[i] = 1.0f;
        } else if (data2[i] == 0.0f) {
            data2[i] = -1.0f;
        }
    }
    free(data1);
}

void release_data(LINEAR_TYPE float *data1, LINEAR_TYPE float *data2) {
    free(data1); // Double free
    free(data2);
}

float calculate_correlation(LINEAR_TYPE float *data1, LINEAR_TYPE float *data2,
                            int size) {
    float correlation = 0.0f;
    for (int i = 0; i < size; i++) {
        correlation += data1[i] * data2[i]; // Use after free
    }
    return correlation;
}

int main() {
    LINEAR_TYPE float *d1, *d2;
    init_data(&d1, &d2, 20);
    process_data(d1, d2, 20);
    release_data(d1, d2);
    float corr = calculate_correlation(d1, d2, 20);
    return 0;
}