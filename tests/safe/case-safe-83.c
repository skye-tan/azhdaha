#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE float *create_neural_network_weights(int layers, int neurons) {
    LINEAR_TYPE float *weights =
        malloc(layers * neurons * neurons * sizeof(float));
    return weights;
}

void initialize_weights_xavier(float *weights, int total_weights) {
    for (int i = 0; i < total_weights; i++) {
        weights[i] = ((float)rand() / RAND_MAX) * 2.0f - 1.0f;
    }
}

float forward_pass_layer(float *weights, float *input, float *output,
                         int neurons) {
    float sum = 0.0f;
    for (int i = 0; i < neurons; i++) {
        output[i] = 0.0f;
        for (int j = 0; j < neurons; j++) {
            output[i] += weights[i * neurons + j] * input[j];
        }
        output[i] = output[i] > 0 ? output[i] : 0; // ReLU
        sum += output[i];
    }
    return sum;
}

void release_neural_network_weights(LINEAR_TYPE float *weights, int layers,
                                    int neurons) {
    float *temp_input = weights;
    float *temp_output = weights;
    float result =
        forward_pass_layer(weights, temp_input, temp_output, neurons);
    free(weights);
}

int main() {
    LINEAR_TYPE float *nn_weights = create_neural_network_weights(3, 4);
    initialize_weights_xavier(nn_weights, 48);
    release_neural_network_weights(nn_weights, 3, 4);
    return 0;
}