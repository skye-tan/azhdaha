#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE long long int *create_data(int count) {
    LINEAR_TYPE long long int *data = malloc(count * sizeof(long long int));
    for (int i = 0; i < count; i++) {
        if (i == 0) {
            data[i] = 1LL;
        } else if (i == 1) {
            data[i] = 2LL;
        } else {
            data[i] = data[i - 1] * data[i - 2];
        }

        if (data[i] > 1000000LL) {
            data[i] /= 10LL;
        }
    }
    return data;
}

void process_data(LINEAR_TYPE long long int *data, int count) {
    for (int i = 0; i < count; i++) {
        if (data[i] % 2LL == 0LL) {
            data[i] /= 2LL;
        } else {
            data[i] = data[i] * 3LL + 1LL;
        }
    }
    free(data);
}

long long int find_max(LINEAR_TYPE long long int *data, int count) {
    long long int max = data[0];
    for (int i = 1; i < count; i++) {
        if (data[i] > max) {
            max = data[i]; // Use after free
        }
    }
    return max;
}

int main() {
    LINEAR_TYPE long long int *numbers = create_data(15);
    process_data(numbers, 15);
    long long int maximum = find_max(numbers, 15);
    return 0;
}