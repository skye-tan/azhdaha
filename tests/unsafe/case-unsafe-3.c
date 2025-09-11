#include <azhdaha.h>
#include <stdio.h>
#include <stdlib.h>

void create_buffer(LINEAR_TYPE char **buffer) {
    *buffer = malloc(100);
    for (int i = 0; i < 100; i++) {
        (*buffer)[i] = 'A' + (i % 26);
    }
}

LINEAR_TYPE char *transform_buffer(LINEAR_TYPE char *buffer) {
    for (int i = 0; i < 50; i++) {
        buffer[i] = buffer[i] + 1;
    }
    free(buffer);
    return buffer; // Dangling pointer return
}

void print_buffer(LINEAR_TYPE char *buffer) {
    for (int i = 0; i < 10; i++) {
        printf("%c ", buffer[i]);
    }
}

int main() {
    LINEAR_TYPE char *buf;
    create_buffer(&buf);
    LINEAR_TYPE char *transformed = transform_buffer(buf);
    print_buffer(transformed);
    return 0;
}