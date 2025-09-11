#include <azhdaha.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void create_strings(LINEAR_TYPE char ***strs, int count) {
    *strs = malloc(count * sizeof(char *));
    for (int i = 0; i < count; i++) {
        (*strs)[i] = malloc(20);
        if (i % 3 == 0) {
            sprintf((*strs)[i], "Type A %d", i);
        } else if (i % 3 == 1) {
            sprintf((*strs)[i], "Type B %d", i);
        } else {
            sprintf((*strs)[i], "Type C %d", i);
        }
    }
}

void process_strings(LINEAR_TYPE char **strs, int count) {
    for (int i = 0; i < count; i++) {
        if (strlen(strs[i]) > 10) {
            strs[i][0] = 'X';
        } else {
            strs[i][0] = 'Y';
        }
    }
    // Free first half
    for (int i = 0; i < count / 2; i++) {
        free(strs[i]);
    }
}

void cleanup_strings(LINEAR_TYPE char **strs, int count) {
    // Try to free all, causing double free for first half
    for (int i = 0; i < count; i++) {
        free(strs[i]);
    }
    free(strs);
}

int count_type_a(LINEAR_TYPE char **strs, int count) {
    int a_count = 0;
    for (int i = 0; i < count; i++) {
        if (strs[i][0] == 'X' || strs[i][0] == 'Y') {
            a_count++; // Use after free
        }
    }
    return a_count;
}

int main() {
    LINEAR_TYPE char **strings;
    create_strings(&strings, 12);
    process_strings(strings, 12);
    cleanup_strings(strings, 12);
    int type_a = count_type_a(strings, 12);
    return 0;
}