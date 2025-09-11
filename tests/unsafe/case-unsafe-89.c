#include <azhdaha.h>
#include <stdlib.h>

void init_matrices(LINEAR_TYPE double **mat1, LINEAR_TYPE double **mat2,
                   int rows, int cols) {
    *mat1 = malloc(rows * cols * sizeof(double));
    *mat2 = malloc(rows * cols * sizeof(double));
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            int idx = i * cols + j;
            if (i == j) {
                (*mat1)[idx] = 1.0;
                (*mat2)[idx] = 1.0;
            } else if (i < j) {
                (*mat1)[idx] = 0.5;
                (*mat2)[idx] = 0.5;
            } else {
                (*mat1)[idx] = 2.0;
                (*mat2)[idx] = 2.0;
            }
        }
    }
}

void multiply_matrices(LINEAR_TYPE double *mat1, LINEAR_TYPE double *mat2,
                       int rows, int cols) {
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            int idx = i * cols + j;
            if (mat1[idx] > 1.0 && mat2[idx] > 1.0) {
                mat1[idx] *= mat2[idx];
            } else if (mat1[idx] < 1.0 && mat2[idx] < 1.0) {
                mat1[idx] *= mat2[idx];
            } else {
                mat1[idx] = 1.0;
            }
        }
    }
    free(mat2);
}

void release_matrices(LINEAR_TYPE double *mat1, LINEAR_TYPE double *mat2) {
    free(mat1);
    free(mat2); // Double free
}

double matrix_determinant(LINEAR_TYPE double *mat1, LINEAR_TYPE double *mat2,
                          int size) {
    double det = 1.0;
    for (int i = 0; i < size; i++) {
        int idx = i * size + i;
        det *= mat1[idx] * mat2[idx]; // Use after free
    }
    return det;
}

int main() {
    LINEAR_TYPE double *matrix1, *matrix2;
    init_matrices(&matrix1, &matrix2, 4, 4);
    multiply_matrices(matrix1, matrix2, 4, 4);
    release_matrices(matrix1, matrix2);
    double determinant = matrix_determinant(matrix1, matrix2, 4);
    return 0;
}