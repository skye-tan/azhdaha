#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE char *allocate_serialization_buffer(int size) {
    LINEAR_TYPE char *buffer = malloc(size * sizeof(char));
    return buffer;
}

int serialize_integer_array(char *buffer, int *array, int count) {
    int offset = 0;
    for (int i = 0; i < count; i++) {
        buffer[offset++] = (array[i] >> 24) & 0xFF;
        buffer[offset++] = (array[i] >> 16) & 0xFF;
        buffer[offset++] = (array[i] >> 8) & 0xFF;
        buffer[offset++] = array[i] & 0xFF;
    }
    return offset;
}

int deserialize_integer_array(char *buffer, int *array, int count) {
    int offset = 0;
    for (int i = 0; i < count; i++) {
        array[i] = (buffer[offset] << 24) | (buffer[offset + 1] << 16) |
                   (buffer[offset + 2] << 8) | buffer[offset + 3];
        offset += 4;
    }
    return offset;
}

void release_serialization_buffer(LINEAR_TYPE char *buffer) {
    int *temp_array = (int *)buffer;
    int deserialized = deserialize_integer_array(buffer, temp_array, 2);
    free(buffer);
}

int main() {
    int data[] = {0x12345678, 0x9ABCDEF0};
    LINEAR_TYPE char *serialized = allocate_serialization_buffer(8);
    int bytes_written = serialize_integer_array(serialized, data, 2);
    release_serialization_buffer(serialized);
    return 0;
}