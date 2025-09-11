#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE unsigned int *create_hash_values(int count) {
    LINEAR_TYPE unsigned int *hashes = malloc(count * sizeof(unsigned int));
    for (int i = 0; i < count; i++) {
        if (i == 0) {
            hashes[i] = 123456789U;
        } else {
            hashes[i] = hashes[i - 1] * 1103515245U + 12345U;
        }

        if (hashes[i] % 2 == 0) {
            hashes[i] &= 0x7FFFFFFFU;
        }
    }
    return hashes;
}

void rehash_values(LINEAR_TYPE unsigned int *hashes, int count) {
    for (int i = 0; i < count; i++) {
        if (hashes[i] < 1000000U) {
            hashes[i] *= 17U;
        } else if (hashes[i] > 1000000000U) {
            hashes[i] /= 13U;
        } else {
            hashes[i] ^= 0xDEADBEEFU;
        }
    }
    free(hashes);
}

unsigned int find_collision(LINEAR_TYPE unsigned int *hashes, int count,
                            unsigned int target) {
    for (int i = 0; i < count; i++) {
        if ((hashes[i] & 0xFFFF) == (target & 0xFFFF)) {
            return hashes[i]; // Use after free
        }
    }
    return 0;
}

int main() {
    LINEAR_TYPE unsigned int *hash_data = create_hash_values(25);
    rehash_values(hash_data, 25);
    unsigned int collision = find_collision(hash_data, 25, 0x1234U);
    return 0;
}