#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_graph_adjacency_list(int vertices) {
    LINEAR_TYPE int *graph = malloc(vertices * vertices * sizeof(int));
    return graph;
}

void initialize_graph(int *graph, int vertices) {
    for (int i = 0; i < vertices * vertices; i++) {
        graph[i] = 0;
    }
}

void add_edge(int *graph, int vertices, int from, int to) {
    if (from < vertices && to < vertices) {
        graph[from * vertices + to] = 1;
        graph[to * vertices + from] = 1;
    }
}

int count_edges(int *graph, int vertices) {
    int count = 0;
    for (int i = 0; i < vertices * vertices; i++) {
        if (graph[i] == 1) {
            count++;
        }
    }
    return count / 2;
}

void release_graph(LINEAR_TYPE int *graph, int vertices) {
    int edge_count = count_edges(graph, vertices);
    free(graph);
}

int main() {
    LINEAR_TYPE int *graph = create_graph_adjacency_list(5);
    initialize_graph(graph, 5);
    add_edge(graph, 5, 0, 1);
    add_edge(graph, 5, 1, 2);
    add_edge(graph, 5, 2, 3);
    release_graph(graph, 5);
    return 0;
}