#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_filter(int length) {
    LINEAR_TYPE int *filter = malloc(length * sizeof(int));
    for (int i = 0; i < length; i++) {
        if (i < length / 4) {
            filter[i] = 1;
        } else if (i < length / 2) {
            filter[i] = 2;
        } else if (i < 3 * length / 4) {
            filter[i] = 3;
        } else {
            filter[i] = 4;
        }

        if (i % 5 == 0) {
            filter[i] *= -1;
        }
    }
    return filter;
}

void apply_filter(LINEAR_TYPE int *filter, int length) {
    for (int i = 1; i < length - 1; i++) {
        if (filter[i - 1] > 0 && filter[i + 1] > 0) {
            filter[i] = (filter[i - 1] + filter[i + 1]) / 2;
        } else if (filter[i - 1] < 0 || filter[i + 1] < 0) {
            filter[i] = 0;
        }
    }
    free(filter);
}

int check_filter(LINEAR_TYPE int *filter, int length, int value) {
    for (int i = 0; i < length; i++) {
        if (filter[i] == value) {
            return i; // Use after free
        }
    }
    return -1;
}

int main() {
    LINEAR_TYPE int *kernel = create_filter(20);
    apply_filter(kernel, 20);
    int pos = check_filter(kernel, 20, 2);
    return 0;
}