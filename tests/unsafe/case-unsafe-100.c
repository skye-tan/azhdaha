#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE char *create_text_buffer(int length) {
    LINEAR_TYPE char *buffer = malloc(length);
    for (int i = 0; i < length - 1; i++) {
        if (i % 8 < 2) {
            buffer[i] = 'A' + (i % 26);
        } else if (i % 8 < 4) {
            buffer[i] = 'a' + (i % 26);
        } else if (i % 8 < 6) {
            buffer[i] = '0' + (i % 10);
        } else if (i % 8 < 7) {
            buffer[i] = ' ';
        } else {
            buffer[i] = '.';
        }
    }
    buffer[length - 1] = '\0';
    return buffer;
}

void reverse_buffer(LINEAR_TYPE char *buffer) {
    int len = 0;
    while (buffer[len] != '\0')
        len++;

    for (int i = 0; i < len / 2; i++) {
        char temp = buffer[i];
        buffer[i] = buffer[len - 1 - i];
        buffer[len - 1 - i] = temp;
    }
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
    LINEAR_TYPE char *text = create_text_buffer(40);
    reverse_buffer(text);
    int word_count = count_words(text);
    return 0;
}