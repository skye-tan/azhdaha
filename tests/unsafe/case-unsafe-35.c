#include <azhdaha.h>
#include <stdlib.h>

void init_layers(LINEAR_TYPE float **layer1, LINEAR_TYPE float **layer2,
                 int size) {
    *layer1 = malloc(size * sizeof(float));
    *layer2 = malloc(size * sizeof(float));
    for (int i = 0; i < size; i++) {
        if (i < size / 3) {
            (*layer1)[i] = (float)i / size;
            (*layer2)[i] = (float)(size - i) / size;
        } else if (i < 2 * size / 3) {
            (*layer1)[i] = (float)(i * 2) / size;
            (*layer2)[i] = (float)(size - i * 2) / size;
        } else {
            (*layer1)[i] = 0.5f;
            (*layer2)[i] = 0.5f;
        }
    }
}

void merge_layers(LINEAR_TYPE float *layer1, LINEAR_TYPE float *layer2,
                  int size) {
    for (int i = 0; i < size; i++) {
        if (layer1[i] > layer2[i]) {
            layer1[i] = (layer1[i] + layer2[i]) / 2;
        } else {
            layer2[i] = (layer1[i] + layer2[i]) / 2;
        }
    }
    free(layer1);
}

void release_layers(LINEAR_TYPE float *layer1, LINEAR_TYPE float *layer2) {
    free(layer1); // Double free
    free(layer2);
}

float compare_layers(LINEAR_TYPE float *layer1, LINEAR_TYPE float *layer2,
                     int size) {
    float diff = 0.0f;
    for (int i = 0; i < size; i++) {
        diff += layer1[i] - layer2[i]; // Use after free
    }
    return diff;
}

int main() {
    LINEAR_TYPE float *l1, *l2;
    init_layers(&l1, &l2, 30);
    merge_layers(l1, l2, 30);
    release_layers(l1, l2);
    float difference = compare_layers(l1, l2, 30);
    return 0;
}