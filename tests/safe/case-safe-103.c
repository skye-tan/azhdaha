#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_routing_table(int entries) {
    LINEAR_TYPE int *table = malloc(entries * 3 * sizeof(int));
    return table;
}

void initialize_routing_table(int *table, int entries) {
    for (int i = 0; i < entries; i++) {
        table[i * 3] = i;
        table[i * 3 + 1] = i * 2;
        table[i * 3 + 2] = i * 3;
    }
}

int lookup_route(int *table, int entries, int destination) {
    for (int i = 0; i < entries; i++) {
        if (table[i * 3] == destination) {
            return table[i * 3 + 2];
        }
    }
    return -1;
}

void update_routing_table(int *table, int entries, int destination, int gateway,
                          int interface) {
    for (int i = 0; i < entries; i++) {
        if (table[i * 3] == destination) {
            table[i * 3 + 1] = gateway;
            table[i * 3 + 2] = interface;
            return;
        }
    }
}

void release_routing_table(LINEAR_TYPE int *table, int entries) {
    int interface = lookup_route(table, entries, 5);
    free(table);
}

int main() {
    LINEAR_TYPE int *routes = create_routing_table(10);
    initialize_routing_table(routes, 10);
    update_routing_table(routes, 10, 5, 10, 15);
    release_routing_table(routes, 10);
    return 0;
}