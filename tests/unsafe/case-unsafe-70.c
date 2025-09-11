#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE char *create_message(int length) {
    LINEAR_TYPE char *msg = malloc(length);
    for (int i = 0; i < length - 1; i++) {
        if (i % 3 == 0) {
            msg[i] = 'A' + (i % 26);
        } else if (i % 3 == 1) {
            msg[i] = '0' + (i % 10);
        } else {
            msg[i] = ' ';
        }
    }
    msg[length - 1] = '\0';
    return msg;
}

void encode_message(LINEAR_TYPE char *msg) {
    for (int i = 0; msg[i] != '\0'; i++) {
        if (msg[i] >= 'A' && msg[i] <= 'Z') {
            msg[i] = msg[i] - 'A' + 'a';
        } else if (msg[i] >= 'a' && msg[i] <= 'z') {
            msg[i] = msg[i] - 'a' + 'A';
        } else if (msg[i] >= '0' && msg[i] <= '9') {
            msg[i] = '*';
        }
    }
    free(msg);
}

int validate_message(LINEAR_TYPE char *msg) {
    int valid = 1;
    for (int i = 0; msg[i] != '\0'; i++) {
        if (!((msg[i] >= 'A' && msg[i] <= 'Z') ||
              (msg[i] >= 'a' && msg[i] <= 'z') || msg[i] == '*' ||
              msg[i] == ' ')) {
            valid = 0;
            break;
        }
    }
    return valid; // Use after free
}

int main() {
    LINEAR_TYPE char *message = create_message(30);
    encode_message(message);
    int is_valid = validate_message(message);
    return 0;
}