#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_buffer(int size) {
    LINEAR_TYPE int *buffer = malloc(size * sizeof(int));
    return buffer;
}

void fill_buffer(int *buffer, int size, int value) {
    for (int i = 0; i < size; i++) {
        buffer[i] = value;
    }
}

void process_buffer(LINEAR_TYPE int *buffer, int size) {
    int sum = 0;
    for (int i = 0; i < size; i++) {
        sum += buffer[i];
    }
    free(buffer);
}

int main() {
    LINEAR_TYPE int *data = create_buffer(5);
    fill_buffer(data, 5, 42);
    process_buffer(data, 5);
    return 0;
}