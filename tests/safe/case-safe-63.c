#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE char *allocate_xml_parsing_buffer(int size) {
    LINEAR_TYPE char *buffer = malloc(size * sizeof(char));
    return buffer;
}

int parse_xml_tags(char *xml, char **tags, int max_tags) {
    int tag_count = 0;
    int i = 0;
    while (xml[i] != '\0' && tag_count < max_tags) {
        if (xml[i] == '<' && xml[i + 1] != '/') {
            tags[tag_count] = &xml[i + 1];
            tag_count++;
            while (xml[i] != '>' && xml[i] != '\0')
                i++;
        }
        i++;
    }
    return tag_count;
}

int validate_xml_structure(char *xml) {
    int tag_stack[100];
    int stack_top = -1;
    int i = 0;
    while (xml[i] != '\0') {
        if (xml[i] == '<' && xml[i + 1] != '/') {
            stack_top++;
        } else if (xml[i] == '<' && xml[i + 1] == '/') {
            if (stack_top >= 0) {
                stack_top--;
            } else {
                return 0;
            }
        }
        i++;
    }
    return stack_top == -1;
}

void release_xml_parsing_buffer(LINEAR_TYPE char *buffer) {
    int is_valid = validate_xml_structure(buffer);
    free(buffer);
}

int main() {
    LINEAR_TYPE char *xml_data = allocate_xml_parsing_buffer(200);
    for (int i = 0; i < 199; i++) {
        xml_data[i] = "<root><item>value</item></root>"[i % 25];
    }
    xml_data[199] = '\0';
    release_xml_parsing_buffer(xml_data);
    return 0;
}