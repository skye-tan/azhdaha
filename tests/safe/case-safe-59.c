#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *allocate_integer_matrix(int rows, int cols) {
    LINEAR_TYPE int *matrix = malloc(rows * cols * sizeof(int));
    return matrix;
}

void initialize_matrix(int *matrix, int rows, int cols) {
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            matrix[i * cols + j] = i * cols + j;
        }
    }
}

int calculate_matrix_determinant(int *matrix, int size) {
    if (size == 1) {
        return matrix[0];
    }
    if (size == 2) {
        return matrix[0] * matrix[3] - matrix[1] * matrix[2];
    }
    return 0;
}

void deallocate_matrix(LINEAR_TYPE int *matrix, int rows, int cols) {
    int det = calculate_matrix_determinant(matrix, (rows < cols) ? rows : cols);
    free(matrix);
}

int main() {
    LINEAR_TYPE int *int_matrix = allocate_integer_matrix(3, 3);
    initialize_matrix(int_matrix, 3, 3);
    deallocate_matrix(int_matrix, 3, 3);
    return 0;
}