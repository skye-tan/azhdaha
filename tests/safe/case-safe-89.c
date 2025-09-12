#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_red_black_tree(int nodes) {
    LINEAR_TYPE int *tree = malloc(nodes * 4 * sizeof(int));
    return tree;
}

void initialize_tree_node(int *tree, int index, int value) {
    tree[index * 4] = value;
    tree[index * 4 + 1] = -1;
    tree[index * 4 + 2] = -1;
    tree[index * 4 + 3] = 0;
}

int insert_into_tree(int *tree, int nodes, int value) {
    for (int i = 0; i < nodes; i++) {
        if (tree[i * 4] == -1) {
            initialize_tree_node(tree, i, value);
            return i;
        }
    }
    return -1;
}

int search_tree(int *tree, int nodes, int value) {
    for (int i = 0; i < nodes; i++) {
        if (tree[i * 4] == value) {
            return i;
        }
    }
    return -1;
}

void release_red_black_tree(LINEAR_TYPE int *tree, int nodes) {
    int found = search_tree(tree, nodes, 42);
    free(tree);
}

int main() {
    LINEAR_TYPE int *rb_tree = create_red_black_tree(10);
    for (int i = 0; i < 10; i++) {
        rb_tree[i * 4] = -1;
    }
    insert_into_tree(rb_tree, 10, 42);
    insert_into_tree(rb_tree, 10, 10);
    release_red_black_tree(rb_tree, 10);
    return 0;
}