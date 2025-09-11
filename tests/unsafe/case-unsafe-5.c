#include <azhdaha.h>
#include <stdio.h>
#include <stdlib.h>

void allocate_strings(LINEAR_TYPE char ***strings, int count) {
    *strings = malloc(count * sizeof(char *));
    for (int i = 0; i < count; i++) {
        (*strings)[i] = malloc(20);
        sprintf((*strings)[i], "String %d", i);
    }
}

void process_strings(LINEAR_TYPE char **strings) {
    for (int i = 0; i < 5; i++) {
        strings[i][0] = 'X';
    }
}

void cleanup_strings(LINEAR_TYPE char **strings) {
    for (int i = 0; i < 5; i++) {
        free(strings[i]);
    }
    free(strings);
    free(strings); // Double free
}

int main() {
    LINEAR_TYPE char **strs;
    allocate_strings(&strs, 5);
    process_strings(strs);
    cleanup_strings(strs);
    return 0;
}