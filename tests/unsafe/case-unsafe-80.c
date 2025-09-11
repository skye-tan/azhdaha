#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_graph(int vertices) {
    LINEAR_TYPE int *graph = malloc(vertices * vertices * sizeof(int));
    for (int i = 0; i < vertices; i++) {
        for (int j = 0; j < vertices; j++) {
            int idx = i * vertices + j;
            if (i == j) {
                graph[idx] = 0;
            } else if ((i + j) % 4 == 0) {
                graph[idx] = 1;
            } else if ((i + j) % 4 == 1) {
                graph[idx] = 2;
            } else if ((i + j) % 4 == 2) {
                graph[idx] = 3;
            } else {
                graph[idx] = 0;
            }
        }
    }
    return graph;
}

void update_graph(LINEAR_TYPE int *graph, int vertices) {
    for (int i = 0; i < vertices; i++) {
        for (int j = 0; j < vertices; j++) {
            int idx = i * vertices + j;
            if (graph[idx] == 1) {
                graph[idx] = 4;
            } else if (graph[idx] == 2) {
                graph[idx] = 1;
            } else if (graph[idx] == 3) {
                graph[idx] = 2;
            }
        }
    }
    free(graph);
}

int count_edges(LINEAR_TYPE int *graph, int vertices) {
    int edges = 0;
    for (int i = 0; i < vertices; i++) {
        for (int j = 0; j < vertices; j++) {
            int idx = i * vertices + j;
            if (graph[idx] > 0) {
                edges++; // Use after free
            }
        }
    }
    return edges;
}

int main() {
    LINEAR_TYPE int *adj_matrix = create_graph(6);
    update_graph(adj_matrix, 6);
    int edge_count = count_edges(adj_matrix, 6);
    return 0;
}