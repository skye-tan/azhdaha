#include <azhdaha.h>
#include <stdlib.h>

void init_matrices(LINEAR_TYPE float ***mat1, LINEAR_TYPE float ***mat2,
                   int size) {
    *mat1 = malloc(size * sizeof(float *));
    *mat2 = malloc(size * sizeof(float *));
    for (int i = 0; i < size; i++) {
        (*mat1)[i] = malloc(size * sizeof(float));
        (*mat2)[i] = malloc(size * sizeof(float));
        for (int j = 0; j < size; j++) {
            if (i == j) {
                (*mat1)[i][j] = 1.0f;
                (*mat2)[i][j] = 1.0f;
            } else {
                (*mat1)[i][j] = 0.0f;
                (*mat2)[i][j] = 0.0f;
            }
        }
    }
}

void multiply_matrices(LINEAR_TYPE float **mat1, LINEAR_TYPE float **mat2,
                       int size) {
    for (int i = 0; i < size; i++) {
        for (int j = 0; j < size; j++) {
            float sum = 0.0f;
            for (int k = 0; k < size; k++) {
                sum += mat1[i][k] * mat2[k][j];
            }
            mat1[i][j] = sum;
        }
    }
    // Free second matrix
    for (int i = 0; i < size; i++) {
        free(mat2[i]);
    }
    free(mat2);
}

void release_matrices(LINEAR_TYPE float **mat1, LINEAR_TYPE float **mat2,
                      int size) {
    for (int i = 0; i < size; i++) {
        free(mat1[i]);
        free(mat2[i]); // Double free for mat2
    }
    free(mat1);
    free(mat2); // Double free
}

float trace_matrix(LINEAR_TYPE float **mat, int size) {
    float trace = 0.0f;
    for (int i = 0; i < size; i++) {
        trace += mat[i][i]; // Use after free
    }
    return trace;
}

int main() {
    LINEAR_TYPE float **matrix1, **matrix2;
    init_matrices(&matrix1, &matrix2, 5);
    multiply_matrices(matrix1, matrix2, 5);
    release_matrices(matrix1, matrix2, 5);
    float tr = trace_matrix(matrix1, 5);
    return 0;
}