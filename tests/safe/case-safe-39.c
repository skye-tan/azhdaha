#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_game_state(int players, int attributes) {
    LINEAR_TYPE int *state = malloc(players * attributes * sizeof(int));
    return state;
}

void initialize_player_states(int *state, int players, int attributes) {
    for (int i = 0; i < players; i++) {
        for (int j = 0; j < attributes; j++) {
            state[i * attributes + j] = (i + 1) * (j + 1);
        }
    }
}

int calculate_player_score(int *player_data, int attributes) {
    int score = 0;
    for (int i = 0; i < attributes; i++) {
        score += player_data[i];
    }
    return score;
}

int find_winner(int *state, int players, int attributes) {
    int winner = 0;
    int highest_score = calculate_player_score(&state[0], attributes);
    for (int i = 1; i < players; i++) {
        int score = calculate_player_score(&state[i * attributes], attributes);
        if (score > highest_score) {
            highest_score = score;
            winner = i;
        }
    }
    return winner;
}

void release_game_state(LINEAR_TYPE int *state, int players, int attributes) {
    int winner = find_winner(state, players, attributes);
    free(state);
}

int main() {
    LINEAR_TYPE int *game_state = create_game_state(4, 3);
    initialize_player_states(game_state, 4, 3);
    release_game_state(game_state, 4, 3);
    return 0;
}