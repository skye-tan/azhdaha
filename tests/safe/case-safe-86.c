#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE char *create_hex_buffer(int length) {
    LINEAR_TYPE char *buffer = malloc(length * sizeof(char));
    return buffer;
}

void fill_with_hex_pattern(char *buffer, int length) {
    char hex_chars[] = "0123456789ABCDEF";
    for (int i = 0; i < length - 1; i++) {
        buffer[i] = hex_chars[i % 16];
    }
    buffer[length - 1] = '\0';
}

int convert_hex_to_decimal(char *buffer) {
    int result = 0;
    int i = 0;
    while (buffer[i] != '\0') {
        result *= 16;
        if (buffer[i] >= '0' && buffer[i] <= '9') {
            result += buffer[i] - '0';
        } else if (buffer[i] >= 'A' && buffer[i] <= 'F') {
            result += buffer[i] - 'A' + 10;
        }
        i++;
    }
    return result;
}

void release_hex_buffer(LINEAR_TYPE char *buffer) {
    int decimal = convert_hex_to_decimal(buffer);
    free(buffer);
}

int main() {
    LINEAR_TYPE char *hex_data = create_hex_buffer(9);
    fill_with_hex_pattern(hex_data, 9);
    release_hex_buffer(hex_data);
    return 0;
}