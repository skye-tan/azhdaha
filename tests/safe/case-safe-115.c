#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_linked_list_array(int size) {
    LINEAR_TYPE int *list = malloc(size * sizeof(int));
    return list;
}

void build_linked_list(int *array, int size) {
    for (int i = 0; i < size; i++) {
        array[i] = i * 2 + 1;
    }
}

int traverse_and_sum(int *array, int size) {
    int sum = 0;
    for (int i = 0; i < size; i++) {
        sum += array[i];
    }
    return sum;
}

void release_linked_list(LINEAR_TYPE int *array, int size) {
    int total = traverse_and_sum(array, size);
    free(array);
}

int main() {
    LINEAR_TYPE int *linked_list = create_linked_list_array(6);
    build_linked_list(linked_list, 6);
    release_linked_list(linked_list, 6);
    return 0;
}