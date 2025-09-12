#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_cache_array(int size) {
    LINEAR_TYPE int *cache = malloc(size * sizeof(int));
    return cache;
}

void initialize_cache(int *cache, int size) {
    for (int i = 0; i < size; i++) {
        cache[i] = -1;
    }
}

int access_cache(int *cache, int size, int key) {
    int index = key % size;
    if (cache[index] == key) {
        return 1;
    } else {
        cache[index] = key;
        return 0;
    }
}

void flush_cache(LINEAR_TYPE int *cache, int size) {
    int hits = 0;
    for (int i = 0; i < size; i++) {
        if (cache[i] != -1) {
            hits++;
        }
    }
    free(cache);
}

int main() {
    LINEAR_TYPE int *cache = create_cache_array(16);
    initialize_cache(cache, 16);
    access_cache(cache, 16, 5);
    access_cache(cache, 16, 10);
    flush_cache(cache, 16);
    return 0;
}