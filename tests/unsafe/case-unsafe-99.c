#include <azhdaha.h>
#include <stdlib.h>

void init_buffers(LINEAR_TYPE unsigned char **buf1,
                  LINEAR_TYPE unsigned char **buf2, int size) {
    *buf1 = malloc(size);
    *buf2 = malloc(size);
    for (int i = 0; i < size; i++) {
        if (i % 7 < 2) {
            (*buf1)[i] = i % 256;
            (*buf2)[i] = (255 - i) % 256;
        } else if (i % 7 < 4) {
            (*buf1)[i] = (i * 2) % 256;
            (*buf2)[i] = (i * 3) % 256;
        } else if (i % 7 < 6) {
            (*buf1)[i] = 0;
            (*buf2)[i] = 255;
        } else {
            (*buf1)[i] = 128;
            (*buf2)[i] = 127;
        }
    }
}

void encrypt_buffers(LINEAR_TYPE unsigned char *buf1,
                     LINEAR_TYPE unsigned char *buf2, int size) {
    for (int i = 0; i < size; i++) {
        if (buf1[i] > 128) {
            buf1[i] ^= buf2[i];
        } else {
            buf2[i] ^= buf1[i];
        }
    }
    free(buf2);
}

void release_buffers(LINEAR_TYPE unsigned char *buf1,
                     LINEAR_TYPE unsigned char *buf2) {
    free(buf1);
    free(buf2); // Double free
}

int validate_encryption(LINEAR_TYPE unsigned char *buf1,
                        LINEAR_TYPE unsigned char *buf2, int size) {
    for (int i = 0; i < size; i++) {
        if ((buf1[i] ^ buf2[i]) != 0) {
            return 0; // Use after free
        }
    }
    return 1;
}

int main() {
    LINEAR_TYPE unsigned char *b1, *b2;
    init_buffers(&b1, &b2, 28);
    encrypt_buffers(b1, b2, 28);
    release_buffers(b1, b2);
    int valid = validate_encryption(b1, b2, 28);
    return 0;
}