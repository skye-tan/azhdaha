#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE double *create_markov_chain(int states) {
    LINEAR_TYPE double *chain = malloc(states * states * sizeof(double));
    return chain;
}

void initialize_transition_matrix(double *matrix, int states) {
    for (int i = 0; i < states; i++) {
        double row_sum = 0.0;
        for (int j = 0; j < states; j++) {
            matrix[i * states + j] = (double)rand() / RAND_MAX;
            row_sum += matrix[i * states + j];
        }
        for (int j = 0; j < states; j++) {
            matrix[i * states + j] /= row_sum;
        }
    }
}

int simulate_markov_chain(double *matrix, int states, int initial_state,
                          int steps) {
    int current_state = initial_state;
    for (int i = 0; i < steps; i++) {
        double random = (double)rand() / RAND_MAX;
        double cumulative = 0.0;
        for (int j = 0; j < states; j++) {
            cumulative += matrix[current_state * states + j];
            if (random <= cumulative) {
                current_state = j;
                break;
            }
        }
    }
    return current_state;
}

void release_markov_chain(LINEAR_TYPE double *chain, int states) {
    int final_state = simulate_markov_chain(chain, states, 0, 10);
    free(chain);
}

int main() {
    LINEAR_TYPE double *markov = create_markov_chain(4);
    initialize_transition_matrix(markov, 4);
    release_markov_chain(markov, 4);
    return 0;
}