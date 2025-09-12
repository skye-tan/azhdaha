#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *allocate_matrix(int rows, int cols) {
    LINEAR_TYPE int *matrix = malloc(rows * cols * sizeof(int));
    return matrix;
}

void initialize_matrix(int *matrix, int rows, int cols) {
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            matrix[i * cols + j] = i + j;
        }
    }
}

int sum_matrix(LINEAR_TYPE int *matrix, int rows, int cols) {
    int sum = 0;
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            sum += matrix[i * cols + j];
        }
    }
    free(matrix);
    return sum;
}

int main() {
    LINEAR_TYPE int *grid = allocate_matrix(3, 4);
    initialize_matrix(grid, 3, 4);
    int result = sum_matrix(grid, 3, 4);
    return 0;
}