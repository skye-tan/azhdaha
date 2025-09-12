#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *allocate_state_machine(int states) {
    LINEAR_TYPE int *machine = malloc(states * states * sizeof(int));
    return machine;
}

void initialize_state_transitions(int *machine, int states) {
    for (int i = 0; i < states; i++) {
        for (int j = 0; j < states; j++) {
            machine[i * states + j] = (i + 1) % states;
        }
    }
}

int execute_state_machine(int *machine, int states, int initial_state,
                          int steps) {
    int current_state = initial_state;
    for (int i = 0; i < steps; i++) {
        current_state =
            machine[current_state * states + ((current_state + i) % states)];
    }
    return current_state;
}

void release_state_machine(LINEAR_TYPE int *machine, int states) {
    int final_state = execute_state_machine(machine, states, 0, 5);
    free(machine);
}

int main() {
    LINEAR_TYPE int *state_machine = allocate_state_machine(4);
    initialize_state_transitions(state_machine, 4);
    release_state_machine(state_machine, 4);
    return 0;
}