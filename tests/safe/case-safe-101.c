#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE char *create_character_array(int length) {
    LINEAR_TYPE char *array = malloc(length * sizeof(char));
    return array;
}

void fill_with_pattern(char *array, int length) {
    for (int i = 0; i < length - 1; i++) {
        array[i] = (i % 2 == 0) ? 'X' : 'O';
    }
    array[length - 1] = '\0';
}

int count_character(char *array, char target) {
    int count = 0;
    int i = 0;
    while (array[i] != '\0') {
        if (array[i] == target) {
            count++;
        }
        i++;
    }
    return count;
}

void release_character_array(LINEAR_TYPE char *array) {
    int x_count = count_character(array, 'X');
    free(array);
}

int main() {
    LINEAR_TYPE char *pattern = create_character_array(16);
    fill_with_pattern(pattern, 16);
    release_character_array(pattern);
    return 0;
}