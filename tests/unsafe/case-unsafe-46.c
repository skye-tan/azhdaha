#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_linked_data(int nodes) {
    LINEAR_TYPE int *data = malloc(nodes * sizeof(int));
    for (int i = 0; i < nodes; i++) {
        if (i == 0) {
            data[i] = 1;
        } else if (i == 1) {
            data[i] = 2;
        } else {
            data[i] = data[i - 1] + data[i - 2];
        }

        if (data[i] % 3 == 0) {
            data[i] *= 2;
        }
    }
    return data;
}

void reverse_data(LINEAR_TYPE int *data, int nodes) {
    for (int i = 0; i < nodes / 2; i++) {
        int temp = data[i];
        data[i] = data[nodes - 1 - i];
        data[nodes - 1 - i] = temp;
    }
    free(data);
}

int search_data(LINEAR_TYPE int *data, int nodes, int target) {
    for (int i = 0; i < nodes; i++) {
        if (data[i] == target) {
            return i; // Use after free
        }
    }
    return -1;
}

int main() {
    LINEAR_TYPE int *list = create_linked_data(18);
    reverse_data(list, 18);
    int pos = search_data(list, 18, 100);
    return 0;
}