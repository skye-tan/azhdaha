#include <azhdaha.h>
#include <stdlib.h>

void init_matrices(LINEAR_TYPE double **mat1, LINEAR_TYPE double **mat2,
                   int rows, int cols) {
    *mat1 = malloc(rows * cols * sizeof(double));
    *mat2 = malloc(rows * cols * sizeof(double));
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            int idx = i * cols + j;
            if ((i + j) % 2 == 0) {
                (*mat1)[idx] = (double)(i + j);
                (*mat2)[idx] = (double)(i - j);
            } else {
                (*mat1)[idx] = (double)(i * j);
                (*mat2)[idx] = (double)(j - i);
            }
        }
    }
}

void multiply_matrices(LINEAR_TYPE double *mat1, LINEAR_TYPE double *mat2,
                       int rows, int cols) {
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            int idx = i * cols + j;
            if (mat1[idx] > 0.0 && mat2[idx] > 0.0) {
                mat1[idx] *= mat2[idx];
            } else if (mat1[idx] < 0.0 && mat2[idx] < 0.0) {
                mat1[idx] *= mat2[idx];
            } else {
                mat1[idx] = 0.0;
            }
        }
    }
    free(mat2);
}

void release_matrices(LINEAR_TYPE double *mat1, LINEAR_TYPE double *mat2) {
    free(mat1);
    free(mat2); // Double free
}

double matrix_trace(LINEAR_TYPE double *mat1, LINEAR_TYPE double *mat2,
                    int size) {
    double trace = 0.0;
    for (int i = 0; i < size; i++) {
        int idx = i * size + i;
        trace += mat1[idx] + mat2[idx]; // Use after free
    }
    return trace;
}

int main() {
    LINEAR_TYPE double *matrix1, *matrix2;
    init_matrices(&matrix1, &matrix2, 5, 5);
    multiply_matrices(matrix1, matrix2, 5, 5);
    release_matrices(matrix1, matrix2);
    double tr = matrix_trace(matrix1, matrix2, 5);
    return 0;
}