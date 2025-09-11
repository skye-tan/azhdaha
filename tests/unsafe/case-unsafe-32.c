#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_tree_data(int nodes) {
    LINEAR_TYPE int *tree = malloc(nodes * sizeof(int));
    for (int i = 0; i < nodes; i++) {
        if (i == 0) {
            tree[i] = 1;
        } else if (i % 2 == 1) {
            tree[i] = tree[(i - 1) / 2] * 2;
        } else {
            tree[i] = tree[(i - 2) / 2] * 2 + 1;
        }
    }
    return tree;
}

void balance_tree(LINEAR_TYPE int *tree, int nodes) {
    for (int i = 0; i < nodes; i++) {
        if (tree[i] > 100) {
            tree[i] /= 2;
        } else if (tree[i] < 10) {
            tree[i] *= 2;
        }
    }
    free(tree);
}

int find_node(LINEAR_TYPE int *tree, int nodes, int value) {
    for (int i = 0; i < nodes; i++) {
        if (tree[i] == value) {
            return i; // Use after free
        }
    }
    return -1;
}

int main() {
    LINEAR_TYPE int *tree_data = create_tree_data(20);
    balance_tree(tree_data, 20);
    int pos = find_node(tree_data, 20, 50);
    return 0;
}