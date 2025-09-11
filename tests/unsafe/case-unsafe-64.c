#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE char *create_text(int length) {
    LINEAR_TYPE char *text = malloc(length);
    for (int i = 0; i < length - 1; i++) {
        if (i % 6 < 2) {
            text[i] = 'A' + (i % 26);
        } else if (i % 6 < 4) {
            text[i] = 'a' + (i % 26);
        } else {
            text[i] = '0' + (i % 10);
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

int count_vowels(LINEAR_TYPE char *text) {
    int count = 0;
    for (int i = 0; text[i] != '\0'; i++) {
        if (text[i] == 'A' || text[i] == 'E' || text[i] == 'I' ||
            text[i] == 'O' || text[i] == 'U' || text[i] == 'a' ||
            text[i] == 'e' || text[i] == 'i' || text[i] == 'o' ||
            text[i] == 'u') {
            count++; // Use after free
        }
    }
    return count;
}

int main() {
    LINEAR_TYPE char *str = create_text(40);
    reverse_text(str);
    int vowels = count_vowels(str);
    return 0;
}