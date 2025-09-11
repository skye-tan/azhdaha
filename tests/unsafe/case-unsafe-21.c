#include <azhdaha.h>
#include <stdlib.h>

void allocate_images(LINEAR_TYPE unsigned char ***images, int count, int size) {
    *images = malloc(count * sizeof(unsigned char *));
    for (int i = 0; i < count; i++) {
        (*images)[i] = malloc(size);
        for (int j = 0; j < size; j++) {
            if ((i + j) % 2 == 0) {
                (*images)[i][j] = 255;
            } else {
                (*images)[i][j] = 0;
            }
        }
    }
}

void process_images(LINEAR_TYPE unsigned char **images, int count, int size) {
    for (int i = 0; i < count; i++) {
        for (int j = 0; j < size; j++) {
            if (images[i][j] == 255) {
                images[i][j] = 128;
            } else {
                images[i][j] = 64;
            }
        }
    }
    // Free every other image
    for (int i = 0; i < count; i += 2) {
        free(images[i]);
    }
}

void cleanup_images(LINEAR_TYPE unsigned char **images, int count) {
    // Try to free all images, causing double free for even indices
    for (int i = 0; i < count; i++) {
        free(images[i]);
    }
    free(images);
}

int count_bright_pixels(LINEAR_TYPE unsigned char **images, int count,
                        int size) {
    int bright = 0;
    for (int i = 0; i < count; i++) {
        for (int j = 0; j < size; j++) {
            if (images[i][j] > 100) {
                bright++; // Use after free
            }
        }
    }
    return bright;
}

int main() {
    LINEAR_TYPE unsigned char **imgs;
    allocate_images(&imgs, 8, 64);
    process_images(imgs, 8, 64);
    cleanup_images(imgs, 8);
    int bright_count = count_bright_pixels(imgs, 8, 64);
    return 0;
}