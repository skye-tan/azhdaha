#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_decision_tree(int nodes) {
    LINEAR_TYPE int *tree = malloc(nodes * 3 * sizeof(int));
    return tree;
}

void build_decision_tree(int *tree, int nodes) {
    for (int i = 0; i < nodes; i++) {
        tree[i * 3] = i;
        tree[i * 3 + 1] = (i * 2 + 1 < nodes) ? i * 2 + 1 : -1;
        tree[i * 3 + 2] = (i * 2 + 2 < nodes) ? i * 2 + 2 : -1;
    }
}

int traverse_decision_tree(int *tree, int nodes, int *data, int data_size) {
    int current_node = 0;
    for (int i = 0; i < data_size && current_node != -1; i++) {
        if (data[i] % 2 == 0) {
            current_node = tree[current_node * 3 + 1];
        } else {
            current_node = tree[current_node * 3 + 2];
        }
    }
    return current_node;
}

void release_decision_tree(LINEAR_TYPE int *tree, int nodes) {
    int final_node = traverse_decision_tree(tree, nodes, NULL, 0);
    free(tree);
}

int main() {
    int input_data[] = {1, 2, 3, 4, 5};
    LINEAR_TYPE int *decision_tree = create_decision_tree(7);
    build_decision_tree(decision_tree, 7);
    release_decision_tree(decision_tree, 7);
    return 0;
}