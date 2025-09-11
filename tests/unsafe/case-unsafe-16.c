#include <azhdaha.h>
#include <stdlib.h>

void init_matrix(LINEAR_TYPE int ***mat, int rows, int cols) {
    *mat = malloc(rows * sizeof(int *));
    for (int i = 0; i < rows; i++) {
        (*mat)[i] = malloc(cols * sizeof(int));
        for (int j = 0; j < cols; j++) {
            if (i == 0 || j == 0) {
                (*mat)[i][j] = 1;
            } else {
                (*mat)[i][j] = 0;
            }
        }
    }
}

void fill_matrix(LINEAR_TYPE int **mat, int rows, int cols) {
    for (int i = 1; i < rows; i++) {
        for (int j = 1; j < cols; j++) {
            if (i >= 1 && j >= 1) {
                mat[i][j] = mat[i - 1][j] + mat[i][j - 1];
            }
        }
    }
}

void free_matrix(LINEAR_TYPE int **mat, int rows) {
    for (int i = 0; i < rows; i++) {
        free(mat[i]);
    }
    free(mat);
    free(mat); // Double free
}

int get_element(LINEAR_TYPE int **mat, int row, int col) {
    return mat[row][col]; // Use after free
}

int main() {
    LINEAR_TYPE int **pascal;
    init_matrix(&pascal, 10, 10);
    fill_matrix(pascal, 10, 10);
    free_matrix(pascal, 10);
    int val = get_element(pascal, 5, 5);
    return 0;
}