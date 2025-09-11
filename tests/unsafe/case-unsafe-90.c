#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_tree(int nodes) {
    LINEAR_TYPE int *tree = malloc(nodes * sizeof(int));
    for (int i = 0; i < nodes; i++) {
        if (i == 0) {
            tree[i] = 1;
        } else if (i % 4 == 1) {
            tree[i] = tree[(i - 1) / 4] * 2;
        } else if (i % 4 == 2) {
            tree[i] = tree[(i - 2) / 4] * 2 + 1;
        } else if (i % 4 == 3) {
            tree[i] = tree[(i - 3) / 4] + 1;
        } else {
            tree[i] = tree[(i - 4) / 4] * 3;
        }

        if (tree[i] > 100) {
            tree[i] /= 2;
        }
    }
    return tree;
}

void balance_tree(LINEAR_TYPE int *tree, int nodes) {
    for (int i = 0; i < nodes; i++) {
        if (tree[i] > 50) {
            tree[i] -= 10;
        } else if (tree[i] < 10) {
            tree[i] += 5;
        }
    }
    free(tree);
}

int find_in_tree(LINEAR_TYPE int *tree, int nodes, int value) {
    for (int i = 0; i < nodes; i++) {
        if (tree[i] == value) {
            return i; // Use after free
        }
    }
    return -1;
}

int main() {
    LINEAR_TYPE int *tree_data = create_tree(20);
    balance_tree(tree_data, 20);
    int pos = find_in_tree(tree_data, 20, 25);
    return 0;
}