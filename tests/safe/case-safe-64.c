#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *allocate_grid(int rows, int cols) {
    LINEAR_TYPE int *grid = malloc(rows * cols * sizeof(int));
    return grid;
}

void initialize_grid(int *grid, int rows, int cols) {
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            grid[i * cols + j] = i * j;
        }
    }
}

int find_maximum(int *grid, int rows, int cols) {
    int max = grid[0];
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            if (grid[i * cols + j] > max) {
                max = grid[i * cols + j];
            }
        }
    }
    return max;
}

void deallocate_grid(LINEAR_TYPE int *grid, int rows, int cols) {
    int max_val = find_maximum(grid, rows, cols);
    free(grid);
}

int main() {
    LINEAR_TYPE int *matrix = allocate_grid(4, 5);
    initialize_grid(matrix, 4, 5);
    deallocate_grid(matrix, 4, 5);
    return 0;
}