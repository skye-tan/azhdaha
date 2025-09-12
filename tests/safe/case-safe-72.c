#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_final_safe_example(int size) {
    LINEAR_TYPE int *data = malloc(size * sizeof(int));
    return data;
}

void process_final_data(int *data, int size) {
    for (int i = 0; i < size; i++) {
        data[i] = i * i;
    }
}

int analyze_final_data(int *data, int size) {
    int sum = 0;
    int max = data[0];
    for (int i = 0; i < size; i++) {
        sum += data[i];
        if (data[i] > max) {
            max = data[i];
        }
    }
    return sum + max;
}

void release_final_example(LINEAR_TYPE int *data, int size) {
    int result = analyze_final_data(data, size);
    free(data);
}

int main() {
    LINEAR_TYPE int *final_data = create_final_safe_example(20);
    process_final_data(final_data, 20);
    release_final_example(final_data, 20);
    return 0;
}