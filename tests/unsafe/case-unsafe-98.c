#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_graph_data(int vertices) {
    LINEAR_TYPE int *graph = malloc(vertices * vertices * sizeof(int));
    for (int i = 0; i < vertices; i++) {
        for (int j = 0; j < vertices; j++) {
            int idx = i * vertices + j;
            if (i == j) {
                graph[idx] = 0;
            } else if ((i + j) % 5 == 0) {
                graph[idx] = 1;
            } else if ((i + j) % 5 == 1) {
                graph[idx] = 2;
            } else if ((i + j) % 5 == 2) {
                graph[idx] = 3;
            } else if ((i + j) % 5 == 3) {
                graph[idx] = 4;
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
                graph[idx] = 5;
            } else if (graph[idx] == 2) {
                graph[idx] = 1;
            } else if (graph[idx] == 3) {
                graph[idx] = 2;
            } else if (graph[idx] == 4) {
                graph[idx] = 3;
            }
        }
    }
    free(graph);
}

int count_connections(LINEAR_TYPE int *graph, int vertices) {
    int connections = 0;
    for (int i = 0; i < vertices; i++) {
        for (int j = 0; j < vertices; j++) {
            int idx = i * vertices + j;
            if (graph[idx] > 0) {
                connections++; // Use after free
            }
        }
    }
    return connections;
}

int main() {
    LINEAR_TYPE int *adj_matrix = create_graph_data(7);
    update_graph(adj_matrix, 7);
    int connections = count_connections(adj_matrix, 7);
    return 0;
}