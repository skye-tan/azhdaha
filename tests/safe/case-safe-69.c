#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_hash_table(int size) {
    LINEAR_TYPE int *table = malloc(size * sizeof(int));
    return table;
}

void initialize_hash_table(int *table, int size) {
    for (int i = 0; i < size; i++) {
        table[i] = -1;
    }
}

int hash_function(int key, int size) { return key % size; }

void insert_into_hash_table(int *table, int size, int key, int value) {
    int index = hash_function(key, size);
    table[index] = value;
}

void release_hash_table(LINEAR_TYPE int *table, int size) {
    int inserted_value = table[0];
    free(table);
}

int main() {
    LINEAR_TYPE int *hash_table = create_hash_table(16);
    initialize_hash_table(hash_table, 16);
    insert_into_hash_table(hash_table, 16, 5, 42);
    release_hash_table(hash_table, 16);
    return 0;
}