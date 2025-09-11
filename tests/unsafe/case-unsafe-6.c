#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_list(int size) {
    LINEAR_TYPE int *list = malloc(size * sizeof(int));
    for (int i = 0; i < size; i++) {
        list[i] = i * i;
    }
    return list;
}

void modify_list(LINEAR_TYPE int *list) {
    for (int i = 0; i < 10; i++) {
        list[i] += 10;
    }
    free(list);
}

int sum_list(LINEAR_TYPE int *list) {
    int sum = 0;
    for (int i = 0; i < 10; i++) {
        sum += list[i]; // Use after free
    }
    return sum;
}

int main() {
    LINEAR_TYPE int *numbers = create_list(10);
    modify_list(numbers);
    int total = sum_list(numbers);
    return 0;
}