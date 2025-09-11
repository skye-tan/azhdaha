#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE char *create_buffer(int length) {
    LINEAR_TYPE char *buffer = malloc(length);
    for (int i = 0; i < length - 1; i++) {
        if (i % 4 == 0) {
            buffer[i] = 'A' + (i % 26);
        } else if (i % 4 == 1) {
            buffer[i] = 'a' + (i % 26);
        } else if (i % 4 == 2) {
            buffer[i] = '0' + (i % 10);
        } else {
            buffer[i] = '+';
        }
    }
    buffer[length - 1] = '\0';
    return buffer;
}

void encode_buffer(LINEAR_TYPE char *buffer) {
    for (int i = 0; buffer[i] != '\0'; i++) {
        if (buffer[i] >= 'A' && buffer[i] <= 'Z') {
            buffer[i] = buffer[i] - 'A' + 'a';
        } else if (buffer[i] >= 'a' && buffer[i] <= 'z') {
            buffer[i] = buffer[i] - 'a' + 'A';
        } else if (buffer[i] >= '0' && buffer[i] <= '9') {
            buffer[i] = '*';
        }
    }
    free(buffer);
}

int validate_encoding(LINEAR_TYPE char *buffer) {
    int valid = 1;
    for (int i = 0; buffer[i] != '\0'; i++) {
        if (!((buffer[i] >= 'A' && buffer[i] <= 'Z') ||
              (buffer[i] >= 'a' && buffer[i] <= 'z') || buffer[i] == '*' ||
              buffer[i] == '+')) {
            valid = 0;
            break;
        }
    }
    return valid; // Use after free
}

int main() {
    LINEAR_TYPE char *data = create_buffer(30);
    encode_buffer(data);
    int is_valid = validate_encoding(data);
    return 0;
}