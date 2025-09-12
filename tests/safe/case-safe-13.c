#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE char *create_text_buffer(int size) {
    LINEAR_TYPE char *buffer = malloc(size * sizeof(char));
    return buffer;
}

void fill_buffer_with_pattern(char *buffer, int size) {
    for (int i = 0; i < size - 1; i++) {
        buffer[i] = '0' + (i % 10);
    }
    buffer[size - 1] = '\0';
}

int count_digits(char *buffer) {
    int count = 0;
    int i = 0;
    while (buffer[i] != '\0') {
        if (buffer[i] >= '0' && buffer[i] <= '9') {
            count++;
        }
        i++;
    }
    return count;
}

void release_text_buffer(LINEAR_TYPE char *buffer) {
    int digit_count = count_digits(buffer);
    free(buffer);
}

int main() {
    LINEAR_TYPE char *text = create_text_buffer(25);
    fill_buffer_with_pattern(text, 25);
    release_text_buffer(text);
    return 0;
}