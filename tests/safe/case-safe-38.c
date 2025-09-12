#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE float *create_float_matrix(int rows, int cols) {
    LINEAR_TYPE float *matrix = malloc(rows * cols * sizeof(float));
    return matrix;
}

void initialize_matrix_values(float *matrix, int rows, int cols) {
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            matrix[i * cols + j] = (float)(i + j) / 2.0f;
        }
    }
}

float calculate_matrix_trace(float *matrix, int size) {
    float trace = 0.0f;
    for (int i = 0; i < size; i++) {
        trace += matrix[i * size + i];
    }
    return trace;
}

void deallocate_float_matrix(LINEAR_TYPE float *matrix, int rows, int cols) {
    float trace = calculate_matrix_trace(matrix, (rows < cols) ? rows : cols);
    free(matrix);
}

int main() {
    LINEAR_TYPE float *grid = create_float_matrix(3, 4);
    initialize_matrix_values(grid, 3, 4);
    deallocate_float_matrix(grid, 3, 4);
    return 0;
}