#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_suffix_array(int length) {
    LINEAR_TYPE int *array = malloc(length * sizeof(int));
    return array;
}

void build_suffix_array(int *suffixes, char *text, int length) {
    for (int i = 0; i < length; i++) {
        suffixes[i] = i;
    }
}

int compare_suffixes(int *suffixes, char *text, int length, int i, int j) {
    while (i < length && j < length && text[i] == text[j]) {
        i++;
        j++;
    }
    if (i == length)
        return -1;
    if (j == length)
        return 1;
    return text[i] - text[j];
}

int search_pattern(int *suffixes, char *text, int length, char *pattern) {
    int pattern_length = 0;
    while (pattern[pattern_length] != '\0')
        pattern_length++;

    for (int i = 0; i < length; i++) {
        int match = 1;
        for (int j = 0; j < pattern_length; j++) {
            if (text[suffixes[i] + j] != pattern[j]) {
                match = 0;
                break;
            }
        }
        if (match)
            return suffixes[i];
    }
    return -1;
}

void release_suffix_array(LINEAR_TYPE int *array, int length) {
    int position = search_pattern(array, "banana", 6, "ana");
    free(array);
}

int main() {
    LINEAR_TYPE int *suffix_array = create_suffix_array(6);
    char text[] = "banana";
    build_suffix_array(suffix_array, text, 6);
    release_suffix_array(suffix_array, 6);
    return 0;
}