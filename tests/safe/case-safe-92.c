#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE char *allocate_protocol_frame(int frame_size) {
    LINEAR_TYPE char *frame = malloc(frame_size * sizeof(char));
    return frame;
}

void format_network_frame(char *frame, int frame_size, int src_addr,
                          int dst_addr, char *payload, int payload_size) {
    frame[0] = (src_addr >> 8) & 0xFF;
    frame[1] = src_addr & 0xFF;
    frame[2] = (dst_addr >> 8) & 0xFF;
    frame[3] = dst_addr & 0xFF;
    frame[4] = (payload_size >> 8) & 0xFF;
    frame[5] = payload_size & 0xFF;

    for (int i = 0; i < payload_size && i < frame_size - 6; i++) {
        frame[6 + i] = payload[i];
    }
}

int validate_frame_checksum(char *frame, int frame_size) {
    int checksum = 0;
    for (int i = 0; i < frame_size - 1; i++) {
        checksum += (unsigned char)frame[i];
    }
    return (checksum & 0xFF) == (unsigned char)frame[frame_size - 1];
}

void process_network_frame(char *frame, int frame_size) {
    int src_addr = (frame[0] << 8) | frame[1];
    int dst_addr = (frame[2] << 8) | frame[3];
    int payload_size = (frame[4] << 8) | frame[5];
    int is_valid = validate_frame_checksum(frame, frame_size);
}

void release_protocol_frame(LINEAR_TYPE char *frame, int frame_size) {
    process_network_frame(frame, frame_size);
    free(frame);
}

int main() {
    LINEAR_TYPE char *network_frame = allocate_protocol_frame(64);
    char payload[] = "Hello Network";
    format_network_frame(network_frame, 64, 0x1234, 0x5678, payload, 13);
    network_frame[63] = 0xAB; // checksum
    release_protocol_frame(network_frame, 64);
    return 0;
}