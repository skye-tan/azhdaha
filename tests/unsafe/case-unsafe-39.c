#include <azhdaha.h>
#include <stdlib.h>

void init_buffers(LINEAR_TYPE char **input, LINEAR_TYPE char **output,
                  int size) {
    *input = malloc(size);
    *output = malloc(size);
    for (int i = 0; i < size - 1; i++) {
        if (i % 3 == 0) {
            (*input)[i] = 'A' + (i % 26);
        } else if (i % 3 == 1) {
            (*input)[i] = '0' + (i % 10);
        } else {
            (*input)[i] = ' ';
        }
        (*output)[i] = '\0';
    }
    (*input)[size - 1] = '\0';
    (*output)[size - 1] = '\0';
}

void process_buffers(LINEAR_TYPE char *input, LINEAR_TYPE char *output,
                     int size) {
    int out_idx = 0;
    for (int i = 0; i < size && input[i] != '\0'; i++) {
        if (input[i] != ' ') {
            if (input[i] >= 'a' && input[i] <= 'z') {
                output[out_idx++] = input[i] - 32;
            } else if (input[i] >= 'A' && input[i] <= 'Z') {
                output[out_idx++] = input[i] + 32;
            } else {
                output[out_idx++] = input[i];
            }
        }
    }
    free(input);
}

void release_buffers(LINEAR_TYPE char *input, LINEAR_TYPE char *output) {
    free(input); // Double free
    free(output);
}

int compare_buffers(LINEAR_TYPE char *input, LINEAR_TYPE char *output) {
    int i = 0;
    while (input[i] != '\0' && output[i] != '\0') {
        if (input[i] != output[i]) {
            return i; // Use after free
        }
        i++;
    }
    return -1;
}

int main() {
    LINEAR_TYPE char *in_buf, *out_buf;
    init_buffers(&in_buf, &out_buf, 50);
    process_buffers(in_buf, out_buf, 50);
    release_buffers(in_buf, out_buf);
    int diff = compare_buffers(in_buf, out_buf);
    return 0;
}