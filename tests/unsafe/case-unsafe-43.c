#include <azhdaha.h>
#include <stdlib.h>

void init_matrices(LINEAR_TYPE int ***mat_a, LINEAR_TYPE int ***mat_b, int rows,
                   int cols) {
    *mat_a = malloc(rows * sizeof(int *));
    *mat_b = malloc(rows * sizeof(int *));
    for (int i = 0; i < rows; i++) {
        (*mat_a)[i] = malloc(cols * sizeof(int));
        (*mat_b)[i] = malloc(cols * sizeof(int));
        for (int j = 0; j < cols; j++) {
            if ((i + j) % 2 == 0) {
                (*mat_a)[i][j] = i + j;
                (*mat_b)[i][j] = i - j;
            } else {
                (*mat_a)[i][j] = i * j;
                (*mat_b)[i][j] = i / (j + 1);
            }
        }
    }
}

void transpose_matrix(LINEAR_TYPE int **matrix, int rows, int cols) {
    for (int i = 0; i < rows; i++) {
        for (int j = i + 1; j < cols; j++) {
            if (i < cols && j < rows) {
                int temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
        }
    }
    // Free the matrix
    for (int i = 0; i < rows; i++) {
        free(matrix[i]);
    }
    free(matrix);
}

void release_matrices(LINEAR_TYPE int **mat_a, LINEAR_TYPE int **mat_b,
                      int rows) {
    for (int i = 0; i < rows; i++) {
        free(mat_a[i]);
        free(mat_b[i]);
    }
    free(mat_a);
    free(mat_b); // Double free for mat_b
}

int matrix_trace(LINEAR_TYPE int **matrix, int size) {
    int trace = 0;
    for (int i = 0; i < size; i++) {
        trace += matrix[i][i]; // Use after free
    }
    return trace;
}

int main() {
    LINEAR_TYPE int **matrix_a, **matrix_b;
    init_matrices(&matrix_a, &matrix_b, 6, 6);
    transpose_matrix(matrix_b, 6, 6);
    release_matrices(matrix_a, matrix_b, 6);
    int trace = matrix_trace(matrix_b, 6);
    return 0;
}