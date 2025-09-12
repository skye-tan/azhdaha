#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE char *create_protocol_buffer(int size) {
    LINEAR_TYPE char *buffer = malloc(size * sizeof(char));
    return buffer;
}

void format_protocol_message(char *buffer, int size, int msg_type,
                             int payload_size) {
    buffer[0] = (msg_type >> 8) & 0xFF;
    buffer[1] = msg_type & 0xFF;
    buffer[2] = (payload_size >> 8) & 0xFF;
    buffer[3] = payload_size & 0xFF;
    for (int i = 4; i < size && i < payload_size + 4; i++) {
        buffer[i] = 'A' + (i % 26);
    }
}

int validate_protocol_message(char *buffer, int size) {
    int msg_type = (buffer[0] << 8) | buffer[1];
    int payload_size = (buffer[2] << 8) | buffer[3];
    return (msg_type >= 0 && msg_type <= 255 && payload_size >= 0 &&
            payload_size <= size - 4);
}

void release_protocol_buffer(LINEAR_TYPE char *buffer, int size) {
    int is_valid = validate_protocol_message(buffer, size);
    free(buffer);
}

int main() {
    LINEAR_TYPE char *protocol_msg = create_protocol_buffer(32);
    format_protocol_message(protocol_msg, 32, 1, 20);
    release_protocol_buffer(protocol_msg, 32);
    return 0;
}