#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE char *create_text(int length) {
    LINEAR_TYPE char *text = malloc(length);
    for (int i = 0; i < length - 1; i++) {
        if (i % 3 == 0) {
            text[i] = 'A' + (i % 26);
        } else if (i % 3 == 1) {
            text[i] = 'a' + (i % 26);
        } else {
            text[i] = '0' + (i % 10);
        }
    }
    text[length - 1] = '\0';
    return text;
}

void modify_text(LINEAR_TYPE char *text) {
    for (int i = 0; text[i] != '\0'; i++) {
        if (text[i] >= 'A' && text[i] <= 'Z') {
            text[i] += 32;
        } else if (text[i] >= 'a' && text[i] <= 'z') {
            text[i] -= 32;
        }
    }
    free(text);
}

int count_uppercase(LINEAR_TYPE char *text) {
    int count = 0;
    for (int i = 0; text[i] != '\0'; i++) {
        if (text[i] >= 'A' && text[i] <= 'Z') {
            count++; // Use after free
        }
    }
    return count;
}

int main() {
    LINEAR_TYPE char *str = create_text(40);
    modify_text(str);
    int upper = count_uppercase(str);
    return 0;
}