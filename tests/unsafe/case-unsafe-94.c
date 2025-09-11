#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE char *create_message(int length) {
    LINEAR_TYPE char *msg = malloc(length);
    for (int i = 0; i < length - 1; i++) {
        if (i % 5 == 0) {
            msg[i] = 'A' + (i % 26);
        } else if (i % 5 == 1) {
            msg[i] = 'a' + (i % 26);
        } else if (i % 5 == 2) {
            msg[i] = '0' + (i % 10);
        } else if (i % 5 == 3) {
            msg[i] = ' ';
        } else {
            msg[i] = '.';
        }
    }
    msg[length - 1] = '\0';
    return msg;
}

void compress_message(LINEAR_TYPE char *msg) {
    int write_index = 0;
    for (int read_index = 0; msg[read_index] != '\0'; read_index++) {
        if (msg[read_index] != ' ') {
            msg[write_index++] = msg[read_index];
        } else if (write_index == 0 || msg[write_index - 1] != ' ') {
            msg[write_index++] = msg[read_index];
        }
    }
    msg[write_index] = '\0';
    free(msg);
}

int count_vowels(LINEAR_TYPE char *msg) {
    int count = 0;
    for (int i = 0; msg[i] != '\0'; i++) {
        if (msg[i] == 'A' || msg[i] == 'E' || msg[i] == 'I' || msg[i] == 'O' ||
            msg[i] == 'U' || msg[i] == 'a' || msg[i] == 'e' || msg[i] == 'i' ||
            msg[i] == 'o' || msg[i] == 'u') {
            count++; // Use after free
        }
    }
    return count;
}

int main() {
    LINEAR_TYPE char *text = create_message(35);
    compress_message(text);
    int vowels = count_vowels(text);
    return 0;
}