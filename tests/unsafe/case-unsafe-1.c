#include <azhdaha.h>
#include <stdlib.h>

void init_array(LINEAR_TYPE int **arr, int size) {
    *arr = malloc(size * sizeof(int));
    for (int i = 0; i < size; i++) {
        (*arr)[i] = i;
    }
}

void process_array(LINEAR_TYPE int *arr) {
    for (int i = 0; i < 10; i++) {
        arr[i] *= 2;
    }
    free(arr);
}

int get_value(LINEAR_TYPE int *arr, int index) {
    return arr[index]; // Use after free
}

int main() {
    LINEAR_TYPE int *data;
    init_array(&data, 10);
    process_array(data);
    int value = get_value(data, 5);
    return 0;
}