#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_neural_network_layer(int neurons, int inputs) {
    LINEAR_TYPE int *layer = malloc(neurons * inputs * sizeof(int));
    return layer;
}

void initialize_weights(int *weights, int size) {
    for (int i = 0; i < size; i++) {
        weights[i] = (i % 20) - 10;
    }
}

int forward_pass(int *weights, int neurons, int inputs, int *input_data) {
    int output = 0;
    for (int i = 0; i < neurons; i++) {
        int neuron_output = 0;
        for (int j = 0; j < inputs; j++) {
            neuron_output += weights[i * inputs + j] * input_data[j];
        }
        output += neuron_output > 0 ? neuron_output : 0;
    }
    return output;
}

void release_neural_network_layer(LINEAR_TYPE int *layer, int neurons,
                                  int inputs) {
    int result = forward_pass(layer, neurons, inputs, NULL);
    free(layer);
}

int main() {
    int input_data[] = {1, 2, 3, 4};
    LINEAR_TYPE int *layer = create_neural_network_layer(3, 4);
    initialize_weights(layer, 12);
    release_neural_network_layer(layer, 3, 4);
    return 0;
}