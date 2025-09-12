#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_pascal_triangle_row(int row) {
    LINEAR_TYPE int *triangle_row = malloc((row + 1) * sizeof(int));
    return triangle_row;
}

void calculate_pascal_values(int *row, int n) {
    row[0] = 1;
    for (int i = 1; i <= n; i++) {
        row[i] = row[i - 1] * (n - i + 1) / i;
    }
}

int sum_pascal_row(int *row, int size) {
    int sum = 0;
    for (int i = 0; i < size; i++) {
        sum += row[i];
    }
    return sum;
}

void release_pascal_row(LINEAR_TYPE int *row, int size) {
    int total = sum_pascal_row(row, size);
    free(row);
}

int main() {
    LINEAR_TYPE int *pascal_row = create_pascal_triangle_row(6);
    calculate_pascal_values(pascal_row, 6);
    release_pascal_row(pascal_row, 7);
    return 0;
}