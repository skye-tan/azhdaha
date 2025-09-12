#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *allocate_game_board(int rows, int cols) {
    LINEAR_TYPE int *board = malloc(rows * cols * sizeof(int));
    return board;
}

void initialize_game_board(int *board, int rows, int cols) {
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            board[i * cols + j] = (i + j) % 2;
        }
    }
}

int count_live_cells(int *board, int rows, int cols) {
    int count = 0;
    for (int i = 0; i < rows * cols; i++) {
        if (board[i] == 1) {
            count++;
        }
    }
    return count;
}

void release_game_board(LINEAR_TYPE int *board, int rows, int cols) {
    int live_cells = count_live_cells(board, rows, cols);
    free(board);
}

int main() {
    LINEAR_TYPE int *game_board = allocate_game_board(8, 8);
    initialize_game_board(game_board, 8, 8);
    release_game_board(game_board, 8, 8);
    return 0;
}