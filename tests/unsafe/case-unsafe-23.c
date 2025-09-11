#include <azhdaha.h>
#include <stdlib.h>

void init_buffers(LINEAR_TYPE char **buf1, LINEAR_TYPE char **buf2, int size) {
    *buf1 = malloc(size);
    *buf2 = malloc(size);
    for (int i = 0; i < size; i++) {
        if (i < size / 2) {
            (*buf1)[i] = 'A' + (i % 26);
            (*buf2)[i] = 'a' + (i % 26);
        } else {
            (*buf1)[i] = '0' + (i % 10);
            (*buf2)[i] = '!' + (i % 15);
        }
    }
}

void merge_buffers(LINEAR_TYPE char *buf1, LINEAR_TYPE char *buf2, int size) {
    for (int i = 0; i < size; i++) {
        if (buf1[i] >= 'A' && buf1[i] <= 'Z') {
            buf1[i] = buf2[i];
        } else {
            buf2[i] = buf1[i];
        }
    }
    free(buf1);
}

void release_buffers(LINEAR_TYPE char *buf1, LINEAR_TYPE char *buf2) {
    free(buf1); // Double free
    free(buf2);
}

int count_vowels(LINEAR_TYPE char *buf1, LINEAR_TYPE char *buf2, int size) {
    int vowels = 0;
    for (int i = 0; i < size; i++) {
        if (buf1[i] == 'A' || buf1[i] == 'E' || buf1[i] == 'I' ||
            buf1[i] == 'O' || buf1[i] == 'U') {
            vowels++; // Use after free
        }
        if (buf2[i] == 'a' || buf2[i] == 'e' || buf2[i] == 'i' ||
            buf2[i] == 'o' || buf2[i] == 'u') {
            vowels++; // Use after free
        }
    }
    return vowels;
}

int main() {
    LINEAR_TYPE char *buffer1, *buffer2;
    init_buffers(&buffer1, &buffer2, 50);
    merge_buffers(buffer1, buffer2, 50);
    release_buffers(buffer1, buffer2);
    int vowel_count = count_vowels(buffer1, buffer2, 50);
    return 0;
}