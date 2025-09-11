#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_graph(int vertices) {
    LINEAR_TYPE int *graph = malloc(vertices * vertices * sizeof(int));
    for (int i = 0; i < vertices; i++) {
        for (int j = 0; j < vertices; j++) {
            if (i == j) {
                graph[i * vertices + j] = 0;
            } else if ((i + j) % 3 == 0) {
                graph[i * vertices + j] = 1;
            } else {
                graph[i * vertices + j] = 0;
            }
        }
    }
    return graph;
}

void update_graph(LINEAR_TYPE int *graph, int vertices) {
    for (int i = 0; i < vertices; i++) {
        for (int j = 0; j < vertices; j++) {
            if (graph[i * vertices + j] == 1) {
                graph[i * vertices + j] = 2;
            } else if (graph[i * vertices + j] == 0 && (i + j) % 2 == 0) {
                graph[i * vertices + j] = 1;
            }
        }
    }
    free(graph);
}

int count_edges(LINEAR_TYPE int *graph, int vertices) {
    int edges = 0;
    for (int i = 0; i < vertices; i++) {
        for (int j = 0; j < vertices; j++) {
            if (graph[i * vertices + j] > 0) {
                edges++; // Use after free
            }
        }
    }
    return edges;
}

int main() {
    LINEAR_TYPE int *adj_matrix = create_graph(8);
    update_graph(adj_matrix, 8);
    int edge_count = count_edges(adj_matrix, 8);
    return 0;
}