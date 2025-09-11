#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_tree_data(int nodes) {
    LINEAR_TYPE int *data = malloc(nodes * sizeof(int));
    for (int i = 0; i < nodes; i++) {
        if (i == 0) {
            data[i] = 1;
        } else if (i % 3 == 1) {
            data[i] = data[(i - 1) / 3] * 2;
        } else if (i % 3 == 2) {
            data[i] = data[(i - 2) / 3] * 2 + 1;
        } else {
            data[i] = data[(i - 3) / 3] + 1;
        }

        if (data[i] > 100) {
            data[i] /= 2;
        }
    }
    return data;
}

void balance_tree(LINEAR_TYPE int *data, int nodes) {
    for (int i = 0; i < nodes; i++) {
        if (data[i] > 50) {
            data[i] -= 10;
        } else if (data[i] < 10) {
            data[i] += 5;
        }
    }
    free(data);
}

int search_tree(LINEAR_TYPE int *data, int nodes, int value) {
    for (int i = 0; i < nodes; i++) {
        if (data[i] == value) {
            return i; // Use after free
        }
    }
    return -1;
}

int main() {
    LINEAR_TYPE int *tree = create_tree_data(21);
    balance_tree(tree, 21);
    int pos = search_tree(tree, 21, 25);
    return 0;
}