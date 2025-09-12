#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_database_index(int records, int fields) {
    LINEAR_TYPE int *index = malloc(records * fields * sizeof(int));
    return index;
}

void initialize_database_index(int *index, int records, int fields) {
    for (int i = 0; i < records; i++) {
        for (int j = 0; j < fields; j++) {
            index[i * fields + j] = i * fields + j;
        }
    }
}

int search_database_index(int *index, int records, int fields, int key) {
    for (int i = 0; i < records; i++) {
        for (int j = 0; j < fields; j++) {
            if (index[i * fields + j] == key) {
                return i;
            }
        }
    }
    return -1;
}

void update_database_record(int *index, int records, int fields, int record_id,
                            int field_id, int value) {
    if (record_id < records && field_id < fields) {
        index[record_id * fields + field_id] = value;
    }
}

void release_database_index(LINEAR_TYPE int *index, int records, int fields) {
    int record = search_database_index(index, records, fields, 42);
    free(index);
}

int main() {
    LINEAR_TYPE int *db_index = create_database_index(100, 5);
    initialize_database_index(db_index, 100, 5);
    update_database_record(db_index, 100, 5, 5, 2, 999);
    release_database_index(db_index, 100, 5);
    return 0;
}