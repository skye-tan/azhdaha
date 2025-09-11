#include <azhdaha.h>
#include <stdlib.h>

void init_matrices(LINEAR_TYPE int **matrix1, LINEAR_TYPE int **matrix2,
                   int rows, int cols) {
    *matrix1 = malloc(rows * cols * sizeof(int));
    *matrix2 = malloc(rows * cols * sizeof(int));
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            if ((i + j) % 2 == 0) {
                (*matrix1)[i * cols + j] = i + j;
                (*matrix2)[i * cols + j] = i - j;
            } else {
                (*matrix1)[i * cols + j] = i * j;
                (*matrix2)[i * cols + j] = j - i;
            }
        }
    }
}

void add_matrices(LINEAR_TYPE int *matrix1, LINEAR_TYPE int *matrix2, int rows,
                  int cols) {
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            int idx = i * cols + j;
            if (matrix1[idx] > 0 && matrix2[idx] > 0) {
                matrix1[idx] += matrix2[idx];
            } else if (matrix1[idx] < 0 && matrix2[idx] < 0) {
                matrix1[idx] += matrix2[idx];
            } else {
                matrix1[idx] = 0;
            }
        }
    }
    free(matrix2);
}

void release_matrices(LINEAR_TYPE int *matrix1, LINEAR_TYPE int *matrix2) {
    free(matrix1);
    free(matrix2); // Double free
}

int matrix_sum(LINEAR_TYPE int *matrix1, LINEAR_TYPE int *matrix2, int rows,
               int cols) {
    int sum = 0;
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            int idx = i * cols + j;
            sum += matrix1[idx] + matrix2[idx]; // Use after free
        }
    }
    return sum;
}

int main() {
    LINEAR_TYPE int *mat1, *mat2;
    init_matrices(&mat1, &mat2, 5, 5);
    add_matrices(mat1, mat2, 5, 5);
    release_matrices(mat1, mat2);
    int total = matrix_sum(mat1, mat2, 5, 5);
    return 0;
}