#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE char *allocate_compressed_buffer(int original_size) {
    LINEAR_TYPE char *compressed = malloc(original_size * sizeof(char));
    return compressed;
}

int run_length_encode(char *input, char *output, int length) {
    int out_index = 0;
    for (int i = 0; i < length;) {
        char current = input[i];
        int count = 1;
        while (i + count < length && input[i + count] == current) {
            count++;
        }
        output[out_index++] = current;
        output[out_index++] = '0' + count;
        i += count;
    }
    return out_index;
}

int run_length_decode(char *input, char *output, int length) {
    int out_index = 0;
    for (int i = 0; i < length; i += 2) {
        char character = input[i];
        int count = input[i + 1] - '0';
        for (int j = 0; j < count; j++) {
            output[out_index++] = character;
        }
    }
    return out_index;
}

void release_compressed_buffer(LINEAR_TYPE char *buffer) {
    int decoded_length = run_length_decode(buffer, buffer, 10);
    free(buffer);
}

int main() {
    LINEAR_TYPE char *compressed = allocate_compressed_buffer(20);
    char original[] = "aaabbbcccaaa";
    int compressed_length = run_length_encode(original, compressed, 12);
    release_compressed_buffer(compressed);
    return 0;
}