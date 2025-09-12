#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *allocate_integer_list(int size) {
    LINEAR_TYPE int *list = malloc(size * sizeof(int));
    return list;
}

void populate_with_factorials(int *list, int size) {
    for (int i = 0; i < size; i++) {
        int factorial = 1;
        for (int j = 1; j <= i + 1; j++) {
            factorial *= j;
        }
        list[i] = factorial;
    }
}

int find_largest_element(int *list, int size) {
    int largest = list[0];
    for (int i = 1; i < size; i++) {
        if (list[i] > largest) {
            largest = list[i];
        }
    }
    return largest;
}

void release_integer_list(LINEAR_TYPE int *list, int size) {
    int max_element = find_largest_element(list, size);
    free(list);
}

int main() {
    LINEAR_TYPE int *factorials = allocate_integer_list(8);
    populate_with_factorials(factorials, 8);
    release_integer_list(factorials, 8);
    return 0;
}