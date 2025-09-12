#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *allocate_binary_array(int bits) {
    LINEAR_TYPE int *array = malloc(bits * sizeof(int));
    return array;
}

void convert_to_binary(int *array, int number, int bits) {
    for (int i = 0; i < bits; i++) {
        array[bits - 1 - i] = (number >> i) & 1;
    }
}

int count_ones(int *array, int bits) {
    int count = 0;
    for (int i = 0; i < bits; i++) {
        if (array[i] == 1) {
            count++;
        }
    }
    return count;
}

void release_binary_array(LINEAR_TYPE int *array, int bits) {
    int ones = count_ones(array, bits);
    free(array);
}

int main() {
    LINEAR_TYPE int *binary = allocate_binary_array(8);
    convert_to_binary(binary, 42, 8);
    release_binary_array(binary, 8);
    return 0;
}