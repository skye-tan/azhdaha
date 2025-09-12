#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE char *allocate_character_grid(int rows, int cols) {
    LINEAR_TYPE char *grid = malloc(rows * cols * sizeof(char));
    return grid;
}

void initialize_character_grid(char *grid, int rows, int cols) {
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            grid[i * cols + j] = 'A' + ((i + j) % 26);
        }
    }
}

int count_vowels_in_grid(char *grid, int rows, int cols) {
    int count = 0;
    for (int i = 0; i < rows * cols; i++) {
        if (grid[i] == 'A' || grid[i] == 'E' || grid[i] == 'I' ||
            grid[i] == 'O' || grid[i] == 'U') {
            count++;
        }
    }
    return count;
}

void release_character_grid(LINEAR_TYPE char *grid, int rows, int cols) {
    int vowel_count = count_vowels_in_grid(grid, rows, cols);
    free(grid);
}

int main() {
    LINEAR_TYPE char *char_grid = allocate_character_grid(4, 6);
    initialize_character_grid(char_grid, 4, 6);
    release_character_grid(char_grid, 4, 6);
    return 0;
}