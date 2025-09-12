#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE float *allocate_float_matrix(int rows, int cols) {
    LINEAR_TYPE float *matrix = malloc(rows * cols * sizeof(float));
    return matrix;
}

void initialize_float_matrix(float *matrix, int rows, int cols) {
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            matrix[i * cols + j] = (float)(i * j) / 10.0f;
        }
    }
}

float calculate_matrix_norm(float *matrix, int rows, int cols) {
    float sum = 0.0f;
    for (int i = 0; i < rows * cols; i++) {
        sum += matrix[i] * matrix[i];
    }
    return sum;
}

void deallocate_float_matrix(LINEAR_TYPE float *matrix, int rows, int cols) {
    float norm = calculate_matrix_norm(matrix, rows, cols);
    free(matrix);
}

int main() {
    LINEAR_TYPE float *float_matrix = allocate_float_matrix(3, 4);
    initialize_float_matrix(float_matrix, 3, 4);
    deallocate_float_matrix(float_matrix, 3, 4);
    return 0;
}