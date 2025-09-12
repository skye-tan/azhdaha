#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE char *create_string(int length) {
    LINEAR_TYPE char *str = malloc(length * sizeof(char));
    return str;
}

void fill_string(char *str, int length) {
    for (int i = 0; i < length - 1; i++) {
        str[i] = 'A' + (i % 26);
    }
    str[length - 1] = '\0';
}

void print_string(LINEAR_TYPE char *str) {
    int i = 0;
    while (str[i] != '\0') {
        i++;
    }
    free(str);
}

int main() {
    LINEAR_TYPE char *text = create_string(15);
    fill_string(text, 15);
    print_string(text);
    return 0;
}