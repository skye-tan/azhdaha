#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE char *allocate_text_buffer(int size) {
    LINEAR_TYPE char *buffer = malloc(size * sizeof(char));
    return buffer;
}

void fill_text_buffer(char *buffer, int size) {
    for (int i = 0; i < size - 1; i++) {
        buffer[i] = 'a' + (i % 26);
    }
    buffer[size - 1] = '\0';
}

int count_vowels(char *buffer) {
    int count = 0;
    int i = 0;
    while (buffer[i] != '\0') {
        if (buffer[i] == 'a' || buffer[i] == 'e' || buffer[i] == 'i' ||
            buffer[i] == 'o' || buffer[i] == 'u') {
            count++;
        }
        i++;
    }
    return count;
}

void release_text_buffer(LINEAR_TYPE char *buffer) {
    int vowels = count_vowels(buffer);
    free(buffer);
}

int main() {
    LINEAR_TYPE char *text = allocate_text_buffer(20);
    fill_text_buffer(text, 20);
    release_text_buffer(text);
    return 0;
}