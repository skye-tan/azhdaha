#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE char *create_fsm_buffer(int states) {
    LINEAR_TYPE char *buffer = malloc(states * states * sizeof(char));
    return buffer;
}

void define_state_transitions(char *buffer, int states) {
    for (int i = 0; i < states; i++) {
        for (int j = 0; j < states; j++) {
            buffer[i * states + j] = (i + 1) % states;
        }
    }
}

int execute_fsm(char *transitions, int states, int initial_state, char *input,
                int input_length) {
    int current_state = initial_state;
    for (int i = 0; i < input_length; i++) {
        int next_state =
            transitions[current_state * states + (input[i] % states)];
        current_state = next_state;
    }
    return current_state;
}

void release_fsm_buffer(LINEAR_TYPE char *buffer, int states) {
    char test_input[] = "abcd";
    int final_state = execute_fsm(buffer, states, 0, test_input, 4);
    free(buffer);
}

int main() {
    LINEAR_TYPE char *fsm = create_fsm_buffer(4);
    define_state_transitions(fsm, 4);
    release_fsm_buffer(fsm, 4);
    return 0;
}