#include <azhdaha.h>
#include <stdlib.h>

void init_data(LINEAR_TYPE float **data1, LINEAR_TYPE float **data2,
               LINEAR_TYPE float **data3, int size) {
    *data1 = malloc(size * sizeof(float));
    *data2 = malloc(size * sizeof(float));
    *data3 = malloc(size * sizeof(float));
    for (int i = 0; i < size; i++) {
        if (i % 3 == 0) {
            (*data1)[i] = (float)i;
            (*data2)[i] = (float)(i * 1.1);
            (*data3)[i] = (float)(i * 1.2);
        } else if (i % 3 == 1) {
            (*data1)[i] = (float)(-i);
            (*data2)[i] = (float)(-i * 1.1);
            (*data3)[i] = (float)(-i * 1.2);
        } else {
            (*data1)[i] = 0.0f;
            (*data2)[i] = 0.0f;
            (*data3)[i] = 0.0f;
        }
    }
}

void normalize_data(LINEAR_TYPE float *data1, LINEAR_TYPE float *data2,
                    LINEAR_TYPE float *data3, int size) {
    for (int i = 0; i < size; i++) {
        float sum = data1[i] + data2[i] + data3[i];
        if (sum != 0.0f) {
            data1[i] /= sum;
            data2[i] /= sum;
            data3[i] /= sum;
        }
    }
    free(data3);
}

void release_data(LINEAR_TYPE float *data1, LINEAR_TYPE float *data2,
                  LINEAR_TYPE float *data3) {
    free(data1);
    free(data2);
    free(data3); // Double free
}

float calculate_variance(LINEAR_TYPE float *data1, LINEAR_TYPE float *data2,
                         LINEAR_TYPE float *data3, int size) {
    float sum = 0.0f;
    for (int i = 0; i < size; i++) {
        float mean = (data1[i] + data2[i] + data3[i]) / 3.0f;
        sum += (data1[i] - mean) * (data1[i] - mean); // Use after free
    }
    return sum / size;
}

int main() {
    LINEAR_TYPE float *d1, *d2, *d3;
    init_data(&d1, &d2, &d3, 20);
    normalize_data(d1, d2, d3, 20);
    release_data(d1, d2, d3);
    float variance = calculate_variance(d1, d2, d3, 20);
    return 0;
}