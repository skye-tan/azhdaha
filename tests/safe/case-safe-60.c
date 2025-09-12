#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE char *allocate_parsing_buffer(int size) {
    LINEAR_TYPE char *buffer = malloc(size * sizeof(char));
    return buffer;
}

int parse_json_object(char *json, int *values, int max_values) {
    int value_count = 0;
    int i = 0;
    while (json[i] != '\0' && value_count < max_values) {
        if (json[i] >= '0' && json[i] <= '9') {
            int value = 0;
            while (json[i] >= '0' && json[i] <= '9') {
                value = value * 10 + (json[i] - '0');
                i++;
            }
            values[value_count++] = value;
        } else {
            i++;
        }
    }
    return value_count;
}

int validate_json_structure(char *json) {
    int brace_count = 0;
    int i = 0;
    while (json[i] != '\0') {
        if (json[i] == '{') {
            brace_count++;
        } else if (json[i] == '}') {
            brace_count--;
        }
        i++;
    }
    return brace_count == 0;
}

void release_parsing_buffer(LINEAR_TYPE char *buffer) {
    int is_valid = validate_json_structure(buffer);
    free(buffer);
}

int main() {
    LINEAR_TYPE char *json_data = allocate_parsing_buffer(100);
    for (int i = 0; i < 99; i++) {
        json_data[i] = "{\"key\": 123}"[i % 10];
    }
    json_data[99] = '\0';
    release_parsing_buffer(json_data);
    return 0;
}