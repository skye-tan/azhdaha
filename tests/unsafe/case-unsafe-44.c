#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE char *create_encoded_string(int length) {
    LINEAR_TYPE char *str = malloc(length);
    for (int i = 0; i < length - 1; i++) {
        if (i % 4 == 0) {
            str[i] = 'A' + (i % 26);
        } else if (i % 4 == 1) {
            str[i] = 'a' + (i % 26);
        } else if (i % 4 == 2) {
            str[i] = '0' + (i % 10);
        } else {
            str[i] = '+';
        }
    }
    str[length - 1] = '\0';
    return str;
}

void decode_string(LINEAR_TYPE char *str) {
    for (int i = 0; str[i] != '\0'; i++) {
        if (str[i] >= 'A' && str[i] <= 'Z') {
            str[i] = str[i] - 'A' + 'a';
        } else if (str[i] >= 'a' && str[i] <= 'z') {
            str[i] = str[i] - 'a' + 'A';
        } else if (str[i] >= '0' && str[i] <= '9') {
            str[i] = '*';
        }
    }
    free(str);
}

int validate_encoding(LINEAR_TYPE char *str) {
    int valid = 1;
    for (int i = 0; str[i] != '\0'; i++) {
        if ((str[i] >= 'A' && str[i] <= 'Z') ||
            (str[i] >= 'a' && str[i] <= 'z')) {
            continue;
        } else if (str[i] != '*') {
            valid = 0;
            break;
        }
    }
    return valid; // Use after free
}

int main() {
    LINEAR_TYPE char *encoded = create_encoded_string(30);
    decode_string(encoded);
    int is_valid = validate_encoding(encoded);
    return 0;
}