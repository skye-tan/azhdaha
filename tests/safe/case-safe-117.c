#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_bloom_filter(int size) {
    LINEAR_TYPE int *filter = malloc(size * sizeof(int));
    return filter;
}

void initialize_bloom_filter(int *filter, int size) {
    for (int i = 0; i < size; i++) {
        filter[i] = 0;
    }
}

int hash_function_1(int key, int size) { return key % size; }

int hash_function_2(int key, int size) { return (key / 7) % size; }

void add_to_bloom_filter(int *filter, int size, int key) {
    int index1 = hash_function_1(key, size);
    int index2 = hash_function_2(key, size);
    filter[index1] = 1;
    filter[index2] = 1;
}

int check_bloom_filter(int *filter, int size, int key) {
    int index1 = hash_function_1(key, size);
    int index2 = hash_function_2(key, size);
    return filter[index1] && filter[index2];
}

void release_bloom_filter(LINEAR_TYPE int *filter, int size) {
    int exists = check_bloom_filter(filter, size, 42);
    free(filter);
}

int main() {
    LINEAR_TYPE int *bloom = create_bloom_filter(32);
    initialize_bloom_filter(bloom, 32);
    add_to_bloom_filter(bloom, 32, 42);
    release_bloom_filter(bloom, 32);
    return 0;
}