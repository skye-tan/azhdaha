#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *allocate_data(int count) {
    LINEAR_TYPE int *data = malloc(count * sizeof(int));
    return data;
}

void process_data(int *data, int count) {
    for (int i = 0; i < count; i++) {
        data[i] = i * 3 + 1;
    }
}

void cleanup_data(LINEAR_TYPE int *data, int count) {
    int sum = 0;
    for (int i = 0; i < count; i++) {
        sum += data[i];
    }
    free(data);
}

int main() {
    LINEAR_TYPE int *values = allocate_data(6);
    process_data(values, 6);
    cleanup_data(values, 6);
    return 0;
}