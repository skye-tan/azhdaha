#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_multiplication_table(int size) {
    LINEAR_TYPE int *table = malloc(size * size * sizeof(int));
    return table;
}

void fill_multiplication_table(int *table, int size) {
    for (int i = 0; i < size; i++) {
        for (int j = 0; j < size; j++) {
            table[i * size + j] = (i + 1) * (j + 1);
        }
    }
}

int calculate_table_sum(int *table, int size) {
    int sum = 0;
    for (int i = 0; i < size * size; i++) {
        sum += table[i];
    }
    return sum;
}

void release_multiplication_table(LINEAR_TYPE int *table, int size) {
    int total = calculate_table_sum(table, size);
    free(table);
}

int main() {
    LINEAR_TYPE int *mult_table = create_multiplication_table(5);
    fill_multiplication_table(mult_table, 5);
    release_multiplication_table(mult_table, 5);
    return 0;
}