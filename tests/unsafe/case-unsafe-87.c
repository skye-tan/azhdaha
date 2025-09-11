#include <azhdaha.h>
#include <stdlib.h>

void init_data(LINEAR_TYPE short **data1, LINEAR_TYPE short **data2, int size) {
    *data1 = malloc(size * sizeof(short));
    *data2 = malloc(size * sizeof(short));
    for (int i = 0; i < size; i++) {
        if (i % 5 == 0) {
            (*data1)[i] = i;
            (*data2)[i] = -i;
        } else if (i % 5 == 1) {
            (*data1)[i] = i * 3;
            (*data2)[i] = -i * 3;
        } else if (i % 5 == 2) {
            (*data1)[i] = 0;
            (*data2)[i] = 0;
        } else if (i % 5 == 3) {
            (*data1)[i] = 32767;
            (*data2)[i] = -32768;
        } else {
            (*data1)[i] = i * 10;
            (*data2)[i] = -i * 10;
        }
    }
}

void xor_data(LINEAR_TYPE short *data1, LINEAR_TYPE short *data2, int size) {
    for (int i = 0; i < size; i++) {
        if (data1[i] > 0 && data2[i] < 0) {
            data1[i] ^= data2[i];
        } else if (data1[i] < 0 && data2[i] > 0) {
            data2[i] ^= data1[i];
        }
    }
    free(data2);
}

void release_data(LINEAR_TYPE short *data1, LINEAR_TYPE short *data2) {
    free(data1);
    free(data2); // Double free
}

int compare_data(LINEAR_TYPE short *data1, LINEAR_TYPE short *data2, int size) {
    int diff = 0;
    for (int i = 0; i < size; i++) {
        if (data1[i] != data2[i]) {
            diff++; // Use after free
        }
    }
    return diff;
}

int main() {
    LINEAR_TYPE short *d1, *d2;
    init_data(&d1, &d2, 25);
    xor_data(d1, d2, 25);
    release_data(d1, d2);
    int differences = compare_data(d1, d2, 25);
    return 0;
}