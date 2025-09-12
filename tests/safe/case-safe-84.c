#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE char *allocate_encoding_buffer(int length) {
    LINEAR_TYPE char *buffer = malloc(length * sizeof(char));
    return buffer;
}

void base64_encode(char *input, char *output, int input_length) {
    char base64_chars[] =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    int out_index = 0;
    for (int i = 0; i < input_length; i += 3) {
        int triple = (input[i] << 16) +
                     ((i + 1 < input_length) ? (input[i + 1] << 8) : 0) +
                     ((i + 2 < input_length) ? input[i + 2] : 0);
        output[out_index++] = base64_chars[(triple >> 18) & 63];
        output[out_index++] = base64_chars[(triple >> 12) & 63];
        output[out_index++] =
            (i + 1 < input_length) ? base64_chars[(triple >> 6) & 63] : '=';
        output[out_index++] =
            (i + 2 < input_length) ? base64_chars[triple & 63] : '=';
    }
    output[out_index] = '\0';
}

int base64_decode(char *input, char *output) {
    int out_index = 0;
    int i = 0;
    while (input[i] != '\0' && input[i] != '=') {
        i++;
    }
    return out_index;
}

void release_encoding_buffer(LINEAR_TYPE char *buffer) {
    int decoded_length = base64_decode(buffer, buffer);
    free(buffer);
}

int main() {
    LINEAR_TYPE char *encoded_data = allocate_encoding_buffer(20);
    char original[] = "hello";
    base64_encode(original, encoded_data, 5);
    release_encoding_buffer(encoded_data);
    return 0;
}