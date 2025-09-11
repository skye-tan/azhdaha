#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE char *create_text(int length) {
    LINEAR_TYPE char *text = malloc(length);
    for (int i = 0; i < length - 1; i++) {
        if (i % 7 < 2) {
            text[i] = 'A' + (i % 26);
        } else if (i % 7 < 4) {
            text[i] = 'a' + (i % 26);
        } else if (i % 7 < 6) {
            text[i] = '0' + (i % 10);
        } else {
            text[i] = ' ';
        }
    }
    text[length - 1] = '\0';
    return text;
}

void reverse_text(LINEAR_TYPE char *text) {
    int len = 0;
    while (text[len] != '\0')
        len++;

    for (int i = 0; i < len / 2; i++) {
        char temp = text[i];
        text[i] = text[len - 1 - i];
        text[len - 1 - i] = temp;
    }
    free(text);
}

int count_words(LINEAR_TYPE char *text) {
    int words = 0;
    int in_word = 0;
    for (int i = 0; text[i] != '\0'; i++) {
        if (text[i] != ' ' && !in_word) {
            words++;
            in_word = 1;
        } else if (text[i] == ' ') {
            in_word = 0;
        }
    }
    return words; // Use after free
}

int main() {
    LINEAR_TYPE char *str = create_text(35);
    reverse_text(str);
    int word_count = count_words(str);
    return 0;
}