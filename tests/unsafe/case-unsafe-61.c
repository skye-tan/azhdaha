#include <azhdaha.h>
#include <stdlib.h>

void init_buffers(LINEAR_TYPE unsigned char **buf1,
                  LINEAR_TYPE unsigned char **buf2, int size) {
    *buf1 = malloc(size);
    *buf2 = malloc(size);
    for (int i = 0; i < size; i++) {
        if (i % 5 == 0) {
            (*buf1)[i] = i % 256;
            (*buf2)[i] = (255 - i) % 256;
        } else if (i % 5 == 1) {
            (*buf1)[i] = (i * 2) % 256;
            (*buf2)[i] = (i * 3) % 256;
        } else if (i % 5 == 2) {
            (*buf1)[i] = (i * 5) % 256;
            (*buf2)[i] = (i * 7) % 256;
        } else if (i % 5 == 3) {
            (*buf1)[i] = 0;
            (*buf2)[i] = 255;
        } else {
            (*buf1)[i] = 128;
            (*buf2)[i] = 127;
        }
    }
}

void xor_buffers(LINEAR_TYPE unsigned char *buf1,
                 LINEAR_TYPE unsigned char *buf2, int size) {
    for (int i = 0; i < size; i++) {
        if (buf1[i] > 128) {
            buf1[i] ^= buf2[i];
        } else {
            buf2[i] ^= buf1[i];
        }
    }
    free(buf1);
}

void release_buffers(LINEAR_TYPE unsigned char *buf1,
                     LINEAR_TYPE unsigned char *buf2) {
    free(buf1); // Double free
    free(buf2);
}

int compare_buffers(LINEAR_TYPE unsigned char *buf1,
                    LINEAR_TYPE unsigned char *buf2, int size) {
    int matches = 0;
    for (int i = 0; i < size; i++) {
        if (buf1[i] == buf2[i]) {
            matches++; // Use after free
        }
    }
    return matches;
}

int main() {
    LINEAR_TYPE unsigned char *b1, *b2;
    init_buffers(&b1, &b2, 32);
    xor_buffers(b1, b2, 32);
    release_buffers(b1, b2);
    int equal = compare_buffers(b1, b2, 32);
    return 0;
}