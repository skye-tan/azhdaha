#include <azhdaha.h>
#include <stdlib.h>

void allocate_memory(LINEAR_TYPE int **ptr) { *ptr = malloc(50); }

void double_free_memory(LINEAR_TYPE int *ptr) {
    free(ptr);
    free(ptr); // Double free
}

void use_memory(LINEAR_TYPE int *ptr) {
    for (int i = 0; i < 10; i++) {
        ptr[i] = i * 3;
    }
}

int main() {
    LINEAR_TYPE int *data;
    allocate_memory(&data);
    use_memory(data);
    double_free_memory(data);
    return 0;
}