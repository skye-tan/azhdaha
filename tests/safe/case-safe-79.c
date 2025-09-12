#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_sorting_array(int size) {
    LINEAR_TYPE int *array = malloc(size * sizeof(int));
    return array;
}

void bubble_sort(int *array, int size) {
    for (int i = 0; i < size - 1; i++) {
        for (int j = 0; j < size - i - 1; j++) {
            if (array[j] > array[j + 1]) {
                int temp = array[j];
                array[j] = array[j + 1];
                array[j + 1] = temp;
            }
        }
    }
}

int find_median(int *array, int size) {
    if (size % 2 == 0) {
        return (array[size / 2 - 1] + array[size / 2]) / 2;
    } else {
        return array[size / 2];
    }
}

void release_sorting_array(LINEAR_TYPE int *array, int size) {
    int median = find_median(array, size);
    free(array);
}

int main() {
    LINEAR_TYPE int *numbers = create_sorting_array(7);
    numbers[0] = 5;
    numbers[1] = 2;
    numbers[2] = 8;
    numbers[3] = 1;
    numbers[4] = 9;
    numbers[5] = 3;
    numbers[6] = 7;
    bubble_sort(numbers, 7);
    release_sorting_array(numbers, 7);
    return 0;
}