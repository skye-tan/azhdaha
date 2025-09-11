#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE char *create_buffer(int length) {
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

void compress_buffer(LINEAR_TYPE char *buffer) {
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

int count_chars(LINEAR_TYPE char *buffer, char target) {
    int count = 0;
    for (int i = 0; buffer[i] != '\0'; i++) {
        if (buffer[i] == target) {
            count++; // Use after free
        }
    }
    return count;
}

int main() {
    LINEAR_TYPE char *text = create_buffer(40);
    compress_buffer(text);
    int letter_a = count_chars(text, 'A');
    return 0;
}