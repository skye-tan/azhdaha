#include <azhdaha.h>
#include <stdlib.h>

void init_grid(LINEAR_TYPE int ***grid, int rows, int cols) {
    *grid = malloc(rows * sizeof(int *));
    for (int i = 0; i < rows; i++) {
        (*grid)[i] = malloc(cols * sizeof(int));
        for (int j = 0; j < cols; j++) {
            if (i == j) {
                (*grid)[i][j] = 1;
            } else {
                (*grid)[i][j] = 0;
            }
        }
    }
}

void multiply_grid(LINEAR_TYPE int **grid, int rows, int cols, int factor) {
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            if (factor != 0) {
                grid[i][j] *= factor;
            }
        }
    }
}

void release_grid(LINEAR_TYPE int **grid, int rows) {
    for (int i = 0; i < rows; i++) {
        free(grid[i]);
    }
    free(grid);
    free(grid); // Double free
}

int trace_grid(LINEAR_TYPE int **grid, int size) {
    int trace = 0;
    for (int i = 0; i < size; i++) {
        if (grid[i][i] > 0) {
            trace += grid[i][i]; // Use after free
        }
    }
    return trace;
}

int main() {
    LINEAR_TYPE int **matrix;
    init_grid(&matrix, 5, 5);
    multiply_grid(matrix, 5, 5, 3);
    release_grid(matrix, 5);
    int diag_sum = trace_grid(matrix, 5);
    return 0;
}