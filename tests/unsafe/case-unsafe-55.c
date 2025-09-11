#include <azhdaha.h>
#include <stdlib.h>

void init_data(LINEAR_TYPE short int **data1, LINEAR_TYPE short int **data2,
               int size) {
    *data1 = malloc(size * sizeof(short int));
    *data2 = malloc(size * sizeof(short int));
    for (int i = 0; i < size; i++) {
        if (i % 6 < 2) {
            (*data1)[i] = i;
            (*data2)[i] = -i;
        } else if (i % 6 < 4) {
            (*data1)[i] = i * 10;
            (*data2)[i] = -i * 10;
        } else {
            (*data1)[i] = 0;
            (*data2)[i] = 0;
        }
    }
}

void process_data(LINEAR_TYPE short int *data1, LINEAR_TYPE short int *data2,
                  int size) {
    for (int i = 0; i < size; i++) {
        if (data1[i] > data2[i]) {
            short int temp = data1[i];
            data1[i] = data2[i];
            data2[i] = temp;
        } else if (data1[i] == data2[i]) {
            data1[i] = data2[i] = 0;
        }
    }
    // Free data1
    free(data1);
}

void release_data(LINEAR_TYPE short int *data1, LINEAR_TYPE short int *data2) {
    free(data1); // Double free
    free(data2);
}

int compare_data(LINEAR_TYPE short int *data1, LINEAR_TYPE short int *data2,
                 int size) {
    int diff = 0;
    for (int i = 0; i < size; i++) {
        if (data1[i] != data2[i]) {
            diff++; // Use after free
        }
    }
    return diff;
}

int main() {
    LINEAR_TYPE short int *d1, *d2;
    init_data(&d1, &d2, 30);
    process_data(d1, d2, 30);
    release_data(d1, d2);
    int differences = compare_data(d1, d2, 30);
    return 0;
}