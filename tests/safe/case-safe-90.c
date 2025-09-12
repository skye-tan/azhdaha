#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE char *allocate_string_buffer(int length) {
    LINEAR_TYPE char *buffer = malloc(length * sizeof(char));
    return buffer;
}

void fill_string_with_reversed_pattern(char *buffer, int length) {
    for (int i = 0; i < length - 1; i++) {
        buffer[i] = 'z' - (i % 26);
    }
    buffer[length - 1] = '\0';
}

int calculate_string_length(char *buffer) {
    int length = 0;
    while (buffer[length] != '\0') {
        length++;
    }
    return length;
}

void deallocate_string_buffer(LINEAR_TYPE char *buffer) {
    int str_length = calculate_string_length(buffer);
    free(buffer);
}

int main() {
    LINEAR_TYPE char *text = allocate_string_buffer(18);
    fill_string_with_reversed_pattern(text, 18);
    deallocate_string_buffer(text);
    return 0;
}