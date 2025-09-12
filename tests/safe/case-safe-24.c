#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE char *allocate_word_buffer(int length) {
    LINEAR_TYPE char *buffer = malloc(length * sizeof(char));
    return buffer;
}

void create_palindrome(char *buffer, int length) {
    for (int i = 0; i < length / 2; i++) {
        buffer[i] = 'a' + (i % 26);
        buffer[length - 2 - i] = 'a' + (i % 26);
    }
    if (length > 1) {
        buffer[length / 2] = 'm';
    }
    buffer[length - 1] = '\0';
}

int check_palindrome(char *buffer) {
    int len = 0;
    while (buffer[len] != '\0')
        len++;
    for (int i = 0; i < len / 2; i++) {
        if (buffer[i] != buffer[len - 1 - i]) {
            return 0;
        }
    }
    return 1;
}

void release_word_buffer(LINEAR_TYPE char *buffer) {
    int is_palindrome = check_palindrome(buffer);
    free(buffer);
}

int main() {
    LINEAR_TYPE char *word = allocate_word_buffer(11);
    create_palindrome(word, 11);
    release_word_buffer(word);
    return 0;
}