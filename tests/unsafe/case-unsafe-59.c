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
            (*data2)[i] = (float)(i * 1.5);
            (*data3)[i] = (float)(i * 2.0);
        } else if (i % 3 == 1) {
            (*data1)[i] = (float)(-i);
            (*data2)[i] = (float)(-i * 1.5);
            (*data3)[i] = (float)(-i * 2.0);
        } else {
            (*data1)[i] = 0.0f;
            (*data2)[i] = 0.0f;
            (*data3)[i] = 0.0f;
        }
    }
}

void process_data(LINEAR_TYPE float *data1, LINEAR_TYPE float *data2,
                  LINEAR_TYPE float *data3, int size) {
    for (int i = 0; i < size; i++) {
        if (data1[i] > data2[i] && data1[i] > data3[i]) {
            data2[i] = data3[i] = data1[i];
        } else if (data2[i] > data1[i] && data2[i] > data3[i]) {
            data1[i] = data3[i] = data2[i];
        } else if (data3[i] > data1[i] && data3[i] > data2[i]) {
            data1[i] = data2[i] = data3[i];
        }
    }
    free(data1);
}

void release_data(LINEAR_TYPE float *data1, LINEAR_TYPE float *data2,
                  LINEAR_TYPE float *data3) {
    free(data1); // Double free
    free(data2);
    free(data3);
}

float calculate_average(LINEAR_TYPE float *data1, LINEAR_TYPE float *data2,
                        LINEAR_TYPE float *data3, int size) {
    float sum = 0.0f;
    for (int i = 0; i < size; i++) {
        sum += (data1[i] + data2[i] + data3[i]) / 3.0f; // Use after free
    }
    return sum / size;
}

int main() {
    LINEAR_TYPE float *d1, *d2, *d3;
    init_data(&d1, &d2, &d3, 18);
    process_data(d1, d2, d3, 18);
    release_data(d1, d2, d3);
    float avg = calculate_average(d1, d2, d3, 18);
    return 0;
}