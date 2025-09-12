#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_compression_dictionary(int size) {
    LINEAR_TYPE int *dict = malloc(size * 2 * sizeof(int));
    return dict;
}

void build_dictionary(int *dictionary, int size) {
    for (int i = 0; i < size; i++) {
        dictionary[i * 2] = i;
        dictionary[i * 2 + 1] = i * 2;
    }
}

int lookup_dictionary(int *dictionary, int size, int key) {
    for (int i = 0; i < size; i++) {
        if (dictionary[i * 2] == key) {
            return dictionary[i * 2 + 1];
        }
    }
    return -1;
}

void update_dictionary(int *dictionary, int size, int key, int value) {
    for (int i = 0; i < size; i++) {
        if (dictionary[i * 2] == key) {
            dictionary[i * 2 + 1] = value;
            return;
        }
    }
}

void release_compression_dictionary(LINEAR_TYPE int *dictionary, int size) {
    int value = lookup_dictionary(dictionary, size, 5);
    free(dictionary);
}

int main() {
    LINEAR_TYPE int *dict = create_compression_dictionary(16);
    build_dictionary(dict, 16);
    update_dictionary(dict, 16, 5, 42);
    release_compression_dictionary(dict, 16);
    return 0;
}