#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_game_board_state(int width, int height) {
    LINEAR_TYPE int *board = malloc(width * height * sizeof(int));
    return board;
}

void initialize_game_board(int *board, int width, int height) {
    for (int i = 0; i < width * height; i++) {
        board[i] = 0;
    }
}

int calculate_board_score(int *board, int width, int height) {
    int score = 0;
    for (int y = 0; y < height; y++) {
        int row_score = 0;
        for (int x = 0; x < width; x++) {
            row_score += board[y * width + x];
        }
        if (row_score == width) {
            score += 100;
        }
    }
    return score;
}

void update_game_board(int *board, int width, int height, int x, int y,
                       int value) {
    if (x >= 0 && x < width && y >= 0 && y < height) {
        board[y * width + x] = value;
    }
}

void release_game_board(LINEAR_TYPE int *board, int width, int height) {
    int score = calculate_board_score(board, width, height);
    free(board);
}

int main() {
    LINEAR_TYPE int *game_board = create_game_board_state(10, 10);
    initialize_game_board(game_board, 10, 10);
    update_game_board(game_board, 10, 10, 5, 5, 1);
    release_game_board(game_board, 10, 10);
    return 0;
}