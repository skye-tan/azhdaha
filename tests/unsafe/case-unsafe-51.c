#include <azhdaha.h>
#include <stdlib.h>

void init_arrays(LINEAR_TYPE float **arr1, LINEAR_TYPE float **arr2,
                 LINEAR_TYPE float **arr3, int size) {
    *arr1 = malloc(size * sizeof(float));
    *arr2 = malloc(size * sizeof(float));
    *arr3 = malloc(size * sizeof(float));
    for (int i = 0; i < size; i++) {
        if (i % 3 == 0) {
            (*arr1)[i] = (float)i;
            (*arr2)[i] = (float)(i * 2);
            (*arr3)[i] = (float)(i * 3);
        } else if (i % 3 == 1) {
            (*arr1)[i] = (float)(-i);
            (*arr2)[i] = (float)(-i * 2);
            (*arr3)[i] = (float)(-i * 3);
        } else {
            (*arr1)[i] = 0.0f;
            (*arr2)[i] = 0.0f;
            (*arr3)[i] = 0.0f;
        }
    }
}

void normalize_arrays(LINEAR_TYPE float *arr1, LINEAR_TYPE float *arr2,
                      LINEAR_TYPE float *arr3, int size) {
    for (int i = 0; i < size; i++) {
        float sum = arr1[i] + arr2[i] + arr3[i];
        if (sum != 0.0f) {
            arr1[i] /= sum;
            arr2[i] /= sum;
            arr3[i] /= sum;
        }
    }
    free(arr2);
}

void release_arrays(LINEAR_TYPE float *arr1, LINEAR_TYPE float *arr2,
                    LINEAR_TYPE float *arr3) {
    free(arr1);
    free(arr2); // Double free
    free(arr3);
}

float calculate_dot_product(LINEAR_TYPE float *arr1, LINEAR_TYPE float *arr2,
                            int size) {
    float product = 0.0f;
    for (int i = 0; i < size; i++) {
        product += arr1[i] * arr2[i]; // Use after free
    }
    return product;
}

int main() {
    LINEAR_TYPE float *a1, *a2, *a3;
    init_arrays(&a1, &a2, &a3, 20);
    normalize_arrays(a1, a2, a3, 20);
    release_arrays(a1, a2, a3);
    float dot = calculate_dot_product(a1, a2, 20);
    return 0;
}