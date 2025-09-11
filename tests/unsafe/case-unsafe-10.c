#include <azhdaha.h>
#include <stdlib.h>

void allocate_blocks(LINEAR_TYPE char **blocks, int count) {
    *blocks = malloc(count);
    for (int i = 0; i < count; i++) {
        if (i < count / 2) {
            (*blocks)[i] = 'A' + i;
        } else {
            (*blocks)[i] = 'a' + (i - count / 2);
        }
    }
}

void transform_blocks(LINEAR_TYPE char *blocks, int count) {
    for (int i = 0; i < count; i++) {
        if (blocks[i] >= 'A' && blocks[i] <= 'Z') {
            blocks[i] += 32;
        } else {
            blocks[i] -= 32;
        }
    }
    free(blocks);
}

int count_upper(LINEAR_TYPE char *blocks, int count) {
    int upper = 0;
    for (int i = 0; i < count; i++) {
        if (blocks[i] >= 'A' && blocks[i] <= 'Z') {
            upper++; // Use after free
        }
    }
    return upper;
}

int main() {
    LINEAR_TYPE char *block_data;
    allocate_blocks(&block_data, 20);
    transform_blocks(block_data, 20);
    int upper_count = count_upper(block_data, 20);
    return 0;
}