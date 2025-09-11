#include <azhdaha.h>
#include <stdlib.h>

void allocate_chunks(LINEAR_TYPE void ***chunks, int count) {
    *chunks = malloc(count * sizeof(void *));
    for (int i = 0; i < count; i++) {
        (*chunks)[i] = malloc(32);
    }
}

void free_chunks(LINEAR_TYPE void **chunks, int count) {
    for (int i = 0; i < count; i++) {
        free(chunks[i]);
    }
    free(chunks);
}

void **duplicate_chunks(LINEAR_TYPE void **chunks) {
    return chunks; // Just return, original will be freed
}

void use_chunk(LINEAR_TYPE void **chunks, int index) {
    char *ptr = (char *)chunks[index];
    for (int i = 0; i < 10; i++) {
        ptr[i] = 'Z'; // May be use after free
    }
}

int main() {
    LINEAR_TYPE void **memory_chunks;
    allocate_chunks(&memory_chunks, 5);
    LINEAR_TYPE void **dup = duplicate_chunks(memory_chunks);
    free_chunks(memory_chunks, 5);
    use_chunk(dup, 2);
    return 0;
}