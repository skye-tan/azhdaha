#include <azhdaha.h>
#include <stdlib.h>

void init_data(LINEAR_TYPE int **data1, LINEAR_TYPE int **data2,
               LINEAR_TYPE int **data3, LINEAR_TYPE int **data4, int size) {
    *data1 = malloc(size * sizeof(int));
    *data2 = malloc(size * sizeof(int));
    *data3 = malloc(size * sizeof(int));
    *data4 = malloc(size * sizeof(int));
    for (int i = 0; i < size; i++) {
        if (i % 4 == 0) {
            (*data1)[i] = i;
            (*data2)[i] = i * 2;
            (*data3)[i] = i * 3;
            (*data4)[i] = i * 4;
        } else if (i % 4 == 1) {
            (*data1)[i] = -i;
            (*data2)[i] = -i * 2;
            (*data3)[i] = -i * 3;
            (*data4)[i] = -i * 4;
        } else if (i % 4 == 2) {
            (*data1)[i] = 0;
            (*data2)[i] = 0;
            (*data3)[i] = 0;
            (*data4)[i] = 0;
        } else {
            (*data1)[i] = i * i;
            (*data2)[i] = -i * i;
            (*data3)[i] = i * i * i;
            (*data4)[i] = -i * i * i;
        }
    }
}

void combine_data(LINEAR_TYPE int *data1, LINEAR_TYPE int *data2,
                  LINEAR_TYPE int *data3, LINEAR_TYPE int *data4, int size) {
    for (int i = 0; i < size; i++) {
        if (data1[i] > data2[i] && data1[i] > data3[i] && data1[i] > data4[i]) {
            data2[i] = data3[i] = data4[i] = data1[i];
        } else if (data2[i] > data1[i] && data2[i] > data3[i] &&
                   data2[i] > data4[i]) {
            data1[i] = data3[i] = data4[i] = data2[i];
        }
    }
    free(data3);
}

void release_data(LINEAR_TYPE int *data1, LINEAR_TYPE int *data2,
                  LINEAR_TYPE int *data3, LINEAR_TYPE int *data4) {
    free(data1);
    free(data2);
    free(data3); // Double free
    free(data4);
}

int find_maximum(LINEAR_TYPE int *data1, LINEAR_TYPE int *data2,
                 LINEAR_TYPE int *data3, LINEAR_TYPE int *data4, int size) {
    int max = data1[0];
    for (int i = 1; i < size; i++) {
        if (data1[i] > max)
            max = data1[i]; // Use after free
        if (data2[i] > max)
            max = data2[i]; // Use after free
        if (data3[i] > max)
            max = data3[i]; // Use after free
        if (data4[i] > max)
            max = data4[i]; // Use after free
    }
    return max;
}

int main() {
    LINEAR_TYPE int *d1, *d2, *d3, *d4;
    init_data(&d1, &d2, &d3, &d4, 16);
    combine_data(d1, d2, d3, d4, 16);
    release_data(d1, d2, d3, d4);
    int maximum = find_maximum(d1, d2, d3, d4, 16);
    return 0;
}