#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_memory_pool(int blocks, int block_size) {
    LINEAR_TYPE int *pool = malloc(blocks * block_size * sizeof(int));
    return pool;
}

void initialize_memory_blocks(int *pool, int blocks, int block_size) {
    for (int i = 0; i < blocks * block_size; i++) {
        pool[i] = 0;
    }
}

int allocate_block(int *pool, int blocks, int block_size, int *allocation_map) {
    for (int i = 0; i < blocks; i++) {
        if (allocation_map[i] == 0) {
            allocation_map[i] = 1;
            return i;
        }
    }
    return -1;
}

void deallocate_block(int *pool, int blocks, int block_size,
                      int *allocation_map, int block_id) {
    if (block_id >= 0 && block_id < blocks) {
        allocation_map[block_id] = 0;
        for (int i = 0; i < block_size; i++) {
            pool[block_id * block_size + i] = 0;
        }
    }
}

void release_memory_pool(LINEAR_TYPE int *pool, int blocks, int block_size) {
    int *allocation_map = (int *)malloc(blocks * sizeof(int));
    for (int i = 0; i < blocks; i++) {
        allocation_map[i] = 0;
    }
    int block = allocate_block(pool, blocks, block_size, allocation_map);
    deallocate_block(pool, blocks, block_size, allocation_map, block);
    free(allocation_map);
    free(pool);
}

int main() {
    LINEAR_TYPE int *mem_pool = create_memory_pool(16, 8);
    int *alloc_map = (int *)malloc(16 * sizeof(int));
    for (int i = 0; i < 16; i++) {
        alloc_map[i] = 0;
    }
    initialize_memory_blocks(mem_pool, 16, 8);
    int block_id = allocate_block(mem_pool, 16, 8, alloc_map);
    deallocate_block(mem_pool, 16, 8, alloc_map, block_id);
    free(alloc_map);
    release_memory_pool(mem_pool, 16, 8);
    return 0;
}