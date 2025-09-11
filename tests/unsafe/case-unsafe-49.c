#include <azhdaha.h>
#include <stdlib.h>

void init_data(LINEAR_TYPE unsigned char **data1,
               LINEAR_TYPE unsigned char **data2, int size) {
    *data1 = malloc(size);
    *data2 = malloc(size);
    for (int i = 0; i < size; i++) {
        if (i % 4 == 0) {
            (*data1)[i] = i % 256;
            (*data2)[i] = (255 - i) % 256;
        } else if (i % 4 == 1) {
            (*data1)[i] = (i * 2) % 256;
            (*data2)[i] = (i * 3) % 256;
        } else if (i % 4 == 2) {
            (*data1)[i] = (i * 5) % 256;
            (*data2)[i] = (i * 7) % 256;
        } else {
            (*data1)[i] = 0;
            (*data2)[i] = 255;
        }
    }
}

void xor_data(LINEAR_TYPE unsigned char *data1,
              LINEAR_TYPE unsigned char *data2, int size) {
    for (int i = 0; i < size; i++) {
        if (data1[i] > 128) {
            data1[i] ^= data2[i];
        } else {
            data2[i] ^= data1[i];
        }
    }
    free(data2);
}

void release_data(LINEAR_TYPE unsigned char *data1,
                  LINEAR_TYPE unsigned char *data2) {
    free(data1);
    free(data2); // Double free
}

int compare_data(LINEAR_TYPE unsigned char *data1,
                 LINEAR_TYPE unsigned char *data2, int size) {
    int matches = 0;
    for (int i = 0; i < size; i++) {
        if (data1[i] == data2[i]) {
            matches++; // Use after free
        }
    }
    return matches;
}

int main() {
    LINEAR_TYPE unsigned char *d1, *d2;
    init_data(&d1, &d2, 32);
    xor_data(d1, d2, 32);
    release_data(d1, d2);
    int equal = compare_data(d1, d2, 32);
    return 0;
}