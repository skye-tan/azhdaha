#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_compression_lz77_window(int window_size) {
    LINEAR_TYPE int *window = malloc(window_size * sizeof(int));
    return window;
}

int find_longest_match(int *window, int window_size, int *input, int input_size,
                       int position) {
    int longest_length = 0;
    int longest_offset = 0;

    for (int i = 0; i < window_size && i < position; i++) {
        int length = 0;
        while (length < input_size - position && i + length < window_size &&
               window[i + length] == input[position + length]) {
            length++;
        }
        if (length > longest_length) {
            longest_length = length;
            longest_offset = i;
        }
    }
    return (longest_length << 16) | longest_offset;
}

void compress_with_lz77(int *window, int window_size, int *input,
                        int input_size, int *output) {
    int output_index = 0;
    for (int i = 0; i < input_size; i++) {
        int match =
            find_longest_match(window, window_size, input, input_size, i);
        output[output_index++] = match;
        window[i % window_size] = input[i];
    }
}

void release_compression_window(LINEAR_TYPE int *window, int window_size) {
    int test_match = find_longest_match(window, window_size, NULL, 0, 0);
    free(window);
}

int main() {
    int input_data[] = {1, 2, 3, 1, 2, 3, 4, 5};
    LINEAR_TYPE int *lz_window = create_compression_lz77_window(8);
    for (int i = 0; i < 8; i++) {
        lz_window[i] = 0;
    }
    int output_data[8];
    compress_with_lz77(lz_window, 8, input_data, 8, output_data);
    release_compression_window(lz_window, 8);
    return 0;
}