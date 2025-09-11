#include <azhdaha.h>
#include <stdlib.h>

void init_data(LINEAR_TYPE double **data1, LINEAR_TYPE double **data2,
               int size) {
    *data1 = malloc(size * sizeof(double));
    *data2 = malloc(size * sizeof(double));
    for (int i = 0; i < size; i++) {
        if (i < size / 4) {
            (*data1)[i] = i * 1.1;
            (*data2)[i] = i * 1.2;
        } else if (i < size / 2) {
            (*data1)[i] = i * 2.1;
            (*data2)[i] = i * 2.2;
        } else if (i < 3 * size / 4) {
            (*data1)[i] = i * 3.1;
            (*data2)[i] = i * 3.2;
        } else {
            (*data1)[i] = i * 4.1;
            (*data2)[i] = i * 4.2;
        }
    }
}

void process_data(LINEAR_TYPE double *data1, LINEAR_TYPE double *data2,
                  int size) {
    for (int i = 0; i < size; i++) {
        if (data1[i] > data2[i]) {
            data1[i] -= data2[i];
        } else {
            data2[i] -= data1[i];
        }
    }
    free(data1);
}

void release_data(LINEAR_TYPE double *data1, LINEAR_TYPE double *data2) {
    free(data1); // Double free
    free(data2);
}

double calculate_difference(LINEAR_TYPE double *data1,
                            LINEAR_TYPE double *data2, int size) {
    double diff = 0.0;
    for (int i = 0; i < size; i++) {
        diff += data1[i] - data2[i]; // Use after free
    }
    return diff;
}

int main() {
    LINEAR_TYPE double *arr1, *arr2;
    init_data(&arr1, &arr2, 20);
    process_data(arr1, arr2, 20);
    release_data(arr1, arr2);
    double difference = calculate_difference(arr1, arr2, 20);
    return 0;
}