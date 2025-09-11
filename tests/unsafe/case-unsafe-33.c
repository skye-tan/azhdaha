#include <azhdaha.h>
#include <stdlib.h>

void init_packets(LINEAR_TYPE unsigned char **packets, int count, int size) {
    *packets = malloc(count * size);
    for (int i = 0; i < count; i++) {
        for (int j = 0; j < size; j++) {
            if (i % 3 == 0) {
                (*packets)[i * size + j] = j % 256;
            } else if (i % 3 == 1) {
                (*packets)[i * size + j] = (j * 2) % 256;
            } else {
                (*packets)[i * size + j] = (j * 3) % 256;
            }
        }
    }
}

void process_packets(LINEAR_TYPE unsigned char *packets, int count, int size) {
    for (int i = 0; i < count; i++) {
        for (int j = 0; j < size; j++) {
            if (packets[i * size + j] > 128) {
                packets[i * size + j] -= 128;
            } else {
                packets[i * size + j] += 128;
            }
        }
    }
    free(packets);
}

int validate_packet(LINEAR_TYPE unsigned char *packets, int packet_index,
                    int size) {
    int checksum = 0;
    for (int i = 0; i < size; i++) {
        checksum += packets[packet_index * size + i]; // Use after free
    }
    return checksum % 256;
}

int main() {
    LINEAR_TYPE unsigned char *data;
    init_packets(&data, 10, 16);
    process_packets(data, 10, 16);
    int checksum = validate_packet(data, 5, 16);
    return 0;
}