#include <azhdaha.h>
#include <stdlib.h>
#include <string.h>

void allocate_structures(LINEAR_TYPE void ***structs, int count) {
    *structs = malloc(count * sizeof(void *));
    for (int i = 0; i < count; i++) {
        (*structs)[i] = malloc(16);
        if (i % 2 == 0) {
            memset((*structs)[i], 0, 16);
        } else {
            memset((*structs)[i], 0xFF, 16);
        }
    }
}

void process_structures(LINEAR_TYPE void **structs, int count) {
    for (int i = 0; i < count; i++) {
        unsigned char *ptr = (unsigned char *)structs[i];
        for (int j = 0; j < 16; j++) {
            if (ptr[j] == 0) {
                ptr[j] = 0xAA;
            } else {
                ptr[j] = 0x55;
            }
        }
    }
    // Free first third
    for (int i = 0; i < count / 3; i++) {
        free(structs[i]);
    }
}

void cleanup_structures(LINEAR_TYPE void **structs, int count) {
    // Try to free all, causing double free for first third
    for (int i = 0; i < count; i++) {
        free(structs[i]);
    }
    free(structs);
}

int check_pattern(LINEAR_TYPE void **structs, int index,
                  unsigned char pattern) {
    unsigned char *ptr = (unsigned char *)structs[index];
    for (int i = 0; i < 16; i++) {
        if (ptr[i] != pattern) {
            return 0; // Use after free
        }
    }
    return 1;
}

int main() {
    LINEAR_TYPE void **data;
    allocate_structures(&data, 12);
    process_structures(data, 12);
    cleanup_structures(data, 12);
    int valid = check_pattern(data, 5, 0x55);
    return 0;
}