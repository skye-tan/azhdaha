#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_list(int size) {
    LINEAR_TYPE int *list = malloc(size * sizeof(int));
    return list;
}

void populate_list(int *list, int size) {
    for (int i = 0; i < size; i++) {
        list[i] = i * i;
    }
}

int find_max(LINEAR_TYPE int *list, int size) {
    int max = list[0];
    for (int i = 1; i < size; i++) {
        if (list[i] > max) {
            max = list[i];
        }
    }
    free(list);
    return max;
}

int main() {
    LINEAR_TYPE int *numbers = create_list(8);
    populate_list(numbers, 8);
    int maximum = find_max(numbers, 8);
    return 0;
}