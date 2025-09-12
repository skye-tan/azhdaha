#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE char *allocate_compression_buffer(int original_size) {
    LINEAR_TYPE char *compressed = malloc(original_size * sizeof(char));
    return compressed;
}

int compress_data(char *input, char *output, int length) {
    int out_index = 0;
    for (int i = 0; i < length;) {
        char current = input[i];
        int count = 1;
        while (i + count < length && input[i + count] == current && count < 9) {
            count++;
        }
        output[out_index++] = current;
        output[out_index++] = '0' + count;
        i += count;
    }
    return out_index;
}

int decompress_data(char *input, char *output, int length) {
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

void release_compression_buffer(LINEAR_TYPE char *buffer) {
    int decompressed_length = decompress_data(buffer, buffer, 10);
    free(buffer);
}

int main() {
    LINEAR_TYPE char *compressed_data = allocate_compression_buffer(30);
    char original[] = "aaabbbcccaaa";
    int compressed_length = compress_data(original, compressed_data, 12);
    release_compression_buffer(compressed_data);
    return 0;
}