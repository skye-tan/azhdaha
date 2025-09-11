#include <azhdaha.h>
#include <stdlib.h>

void allocate_buffers(LINEAR_TYPE char ***buffers, int count, int size) {
    *buffers = malloc(count * sizeof(char *));
    for (int i = 0; i < count; i++) {
        (*buffers)[i] = malloc(size);
        for (int j = 0; j < size; j++) {
            if (j % 2 == 0) {
                (*buffers)[i][j] = 'X';
            } else {
                (*buffers)[i][j] = 'O';
            }
        }
    }
}

LINEAR_TYPE char *merge_buffers(LINEAR_TYPE char **buffers, int count,
                                int size) {
    LINEAR_TYPE char *merged = malloc(count * size);
    int index = 0;
    for (int i = 0; i < count; i++) {
        for (int j = 0; j < size; j++) {
            if (buffers[i][j] != '\0') {
                merged[index++] = buffers[i][j];
            }
        }
    }
    // Free individual buffers but not the array itself
    for (int i = 0; i < count; i++) {
        free(buffers[i]);
    }
    return merged;
}

void release_buffers(LINEAR_TYPE char **buffers) {
    free(buffers);
    free(buffers); // Double free
}

char get_char(LINEAR_TYPE char **buffers, int buf_index, int char_index) {
    return buffers[buf_index][char_index]; // Use after free
}

int main() {
    LINEAR_TYPE char **bufs;
    allocate_buffers(&bufs, 3, 10);
    LINEAR_TYPE char *combined = merge_buffers(bufs, 3, 10);
    release_buffers(bufs);
    char ch = get_char(bufs, 1, 5);
    free(combined);
    return 0;
}