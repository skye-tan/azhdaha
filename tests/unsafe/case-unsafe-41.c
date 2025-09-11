#include <azhdaha.h>
#include <stdlib.h>

void init_arrays(LINEAR_TYPE double **arr1, LINEAR_TYPE double **arr2,
                 LINEAR_TYPE double **arr3, int size) {
    *arr1 = malloc(size * sizeof(double));
    *arr2 = malloc(size * sizeof(double));
    *arr3 = malloc(size * sizeof(double));
    for (int i = 0; i < size; i++) {
        if (i % 3 == 0) {
            (*arr1)[i] = i * 1.5;
            (*arr2)[i] = i * 2.5;
            (*arr3)[i] = i * 3.5;
        } else if (i % 3 == 1) {
            (*arr1)[i] = -i * 1.5;
            (*arr2)[i] = -i * 2.5;
            (*arr3)[i] = -i * 3.5;
        } else {
            (*arr1)[i] = 0.0;
            (*arr2)[i] = 0.0;
            (*arr3)[i] = 0.0;
        }
    }
}

void process_arrays(LINEAR_TYPE double *arr1, LINEAR_TYPE double *arr2,
                    LINEAR_TYPE double *arr3, int size) {
    for (int i = 0; i < size; i++) {
        if (arr1[i] > arr2[i] && arr1[i] > arr3[i]) {
            arr2[i] = arr3[i] = arr1[i];
        } else if (arr2[i] > arr1[i] && arr2[i] > arr3[i]) {
            arr1[i] = arr3[i] = arr2[i];
        } else if (arr3[i] > arr1[i] && arr3[i] > arr2[i]) {
            arr1[i] = arr2[i] = arr3[i];
        }
    }
    free(arr2);
}

void release_arrays(LINEAR_TYPE double *arr1, LINEAR_TYPE double *arr2,
                    LINEAR_TYPE double *arr3) {
    free(arr1);
    free(arr2); // Double free
    free(arr3);
}

double calculate_variance(LINEAR_TYPE double *arr1, LINEAR_TYPE double *arr2,
                          LINEAR_TYPE double *arr3, int size) {
    double sum = 0.0;
    for (int i = 0; i < size; i++) {
        double mean = (arr1[i] + arr2[i] + arr3[i]) / 3.0;
        double diff = (arr1[i] - mean) * (arr1[i] - mean);
        sum += diff; // Use after free
    }
    return sum / size;
}

int main() {
    LINEAR_TYPE double *a1, *a2, *a3;
    init_arrays(&a1, &a2, &a3, 15);
    process_arrays(a1, a2, a3, 15);
    release_arrays(a1, a2, a3);
    double variance = calculate_variance(a1, a2, a3, 15);
    return 0;
}