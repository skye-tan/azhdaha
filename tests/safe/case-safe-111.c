#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *allocate_memory_block(int elements) {
    LINEAR_TYPE int *block = malloc(elements * sizeof(int));
    return block;
}

void initialize_block(int *block, int elements) {
    for (int i = 0; i < elements; i++) {
        block[i] = i * i * i;
    }
}

int find_minimum(int *block, int elements) {
    int min = block[0];
    for (int i = 1; i < elements; i++) {
        if (block[i] < min) {
            min = block[i];
        }
    }
    return min;
}

void deallocate_block(LINEAR_TYPE int *block, int elements) {
    int minimum = find_minimum(block, elements);
    free(block);
}

int main() {
    LINEAR_TYPE int *data = allocate_memory_block(9);
    initialize_block(data, 9);
    deallocate_block(data, 9);
    return 0;
}