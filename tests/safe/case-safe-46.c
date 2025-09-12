#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_compiler_symbol_table(int symbols) {
    LINEAR_TYPE int *table = malloc(symbols * 3 * sizeof(int));
    return table;
}

void initialize_symbol_table(int *table, int symbols) {
    for (int i = 0; i < symbols; i++) {
        table[i * 3] = i;
        table[i * 3 + 1] = i * 100;
        table[i * 3 + 2] = i * 200;
    }
}

int lookup_symbol(int *table, int symbols, int symbol_id) {
    for (int i = 0; i < symbols; i++) {
        if (table[i * 3] == symbol_id) {
            return table[i * 3 + 1];
        }
    }
    return -1;
}

void update_symbol(int *table, int symbols, int symbol_id, int address,
                   int type) {
    for (int i = 0; i < symbols; i++) {
        if (table[i * 3] == symbol_id) {
            table[i * 3 + 1] = address;
            table[i * 3 + 2] = type;
            return;
        }
    }
}

void release_compiler_symbol_table(LINEAR_TYPE int *table, int symbols) {
    int address = lookup_symbol(table, symbols, 5);
    free(table);
}

int main() {
    LINEAR_TYPE int *symbol_table = create_compiler_symbol_table(20);
    initialize_symbol_table(symbol_table, 20);
    update_symbol(symbol_table, 20, 5, 1000, 1);
    release_compiler_symbol_table(symbol_table, 20);
    return 0;
}