#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE char *create_text_buffer(int length) {
    LINEAR_TYPE char *buffer = malloc(length);
    for (int i = 0; i < length - 1; i++) {
        if (i % 5 == 0) {
            buffer[i] = 'A' + (i % 26);
        } else if (i % 5 == 1) {
            buffer[i] = 'a' + (i % 26);
        } else if (i % 5 == 2) {
            buffer[i] = '0' + (i % 10);
        } else if (i % 5 == 3) {
            buffer[i] = ' ';
        } else {
            buffer[i] = '.';
        }
    }
    buffer[length - 1] = '\0';
    return buffer;
}

void compress_text(LINEAR_TYPE char *buffer) {
    int write_index = 0;
    for (int read_index = 0; buffer[read_index] != '\0'; read_index++) {
        if (buffer[read_index] != ' ') {
            buffer[write_index++] = buffer[read_index];
        } else if (write_index == 0 || buffer[write_index - 1] != ' ') {
            buffer[write_index++] = buffer[read_index];
        }
    }
    buffer[write_index] = '\0';
    free(buffer);
}

int count_words(LINEAR_TYPE char *buffer) {
    int words = 0;
    int in_word = 0;
    for (int i = 0; buffer[i] != '\0'; i++) {
        if (buffer[i] != ' ' && !in_word) {
            words++;
            in_word = 1;
        } else if (buffer[i] == ' ') {
            in_word = 0;
        }
    }
    return words; // Use after free
}

int main() {
    LINEAR_TYPE char *text = create_text_buffer(50);
    compress_text(text);
    int word_count = count_words(text);
    return 0;
}