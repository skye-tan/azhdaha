#include <azhdaha.h>
#include <stdlib.h>

void init_arrays(LINEAR_TYPE float **arr1, LINEAR_TYPE float **arr2, int size) {
    *arr1 = malloc(size * sizeof(float));
    *arr2 = malloc(size * sizeof(float));
    for (int i = 0; i < size; i++) {
        if (i % 3 == 0) {
            (*arr1)[i] = (float)i;
            (*arr2)[i] = (float)(-i);
        } else if (i % 3 == 1) {
            (*arr1)[i] = (float)(i * 1.2);
            (*arr2)[i] = (float)(-i * 1.2);
        } else {
            (*arr1)[i] = 0.0f;
            (*arr2)[i] = 0.0f;
        }
    }
}

void normalize_arrays(LINEAR_TYPE float *arr1, LINEAR_TYPE float *arr2,
                      int size) {
    for (int i = 0; i < size; i++) {
        float sum = arr1[i] + arr2[i];
        if (sum != 0.0f) {
            arr1[i] /= sum;
            arr2[i] /= sum;
        }
    }
    free(arr1);
}

void release_arrays(LINEAR_TYPE float *arr1, LINEAR_TYPE float *arr2) {
    free(arr1); // Double free
    free(arr2);
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
    LINEAR_TYPE float *a1, *a2;
    init_arrays(&a1, &a2, 21);
    normalize_arrays(a1, a2, 21);
    release_arrays(a1, a2);
    float dot = calculate_dot_product(a1, a2, 21);
    return 0;
}