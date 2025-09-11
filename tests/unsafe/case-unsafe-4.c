#include <azhdaha.h>
#include <stdlib.h>

void init_matrix(LINEAR_TYPE int ***matrix, int rows, int cols) {
    *matrix = malloc(rows * sizeof(int *));
    for (int i = 0; i < rows; i++) {
        (*matrix)[i] = malloc(cols * sizeof(int));
    }

    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            (*matrix)[i][j] = i * cols + j;
        }
    }
}

void free_matrix(LINEAR_TYPE int **matrix, int rows) {
    for (int i = 0; i < rows; i++) {
        free(matrix[i]);
    }
    free(matrix);
}

int access_matrix(LINEAR_TYPE int **matrix, int row, int col) {
    return matrix[row][col]; // Potential use after free if called incorrectly
}

int main() {
    LINEAR_TYPE int **mat;
    init_matrix(&mat, 5, 5);
    free_matrix(mat, 5);
    int val = access_matrix(mat, 2, 3); // Use after free
    return 0;
}