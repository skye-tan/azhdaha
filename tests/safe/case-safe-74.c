#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE float *allocate_image_processing_buffer(int width, int height) {
    LINEAR_TYPE float *buffer = malloc(width * height * sizeof(float));
    return buffer;
}

void apply_gaussian_blur(float *image, float *output, int width, int height) {
    float kernel[9] = {1, 2, 1, 2, 4, 2, 1, 2, 1};
    float kernel_sum = 16.0f;

    for (int y = 1; y < height - 1; y++) {
        for (int x = 1; x < width - 1; x++) {
            float sum = 0.0f;
            int k = 0;
            for (int ky = -1; ky <= 1; ky++) {
                for (int kx = -1; kx <= 1; kx++) {
                    sum += image[(y + ky) * width + (x + kx)] * kernel[k];
                    k++;
                }
            }
            output[y * width + x] = sum / kernel_sum;
        }
    }
}

float calculate_image_brightness(float *image, int width, int height) {
    float sum = 0.0f;
    for (int i = 0; i < width * height; i++) {
        sum += image[i];
    }
    return sum / (width * height);
}

void release_image_processing_buffer(LINEAR_TYPE float *buffer, int width,
                                     int height) {
    float brightness = calculate_image_brightness(buffer, width, height);
    free(buffer);
}

int main() {
    LINEAR_TYPE float *image = allocate_image_processing_buffer(10, 10);
    for (int i = 0; i < 100; i++) {
        image[i] = (float)(i % 256) / 255.0f;
    }
    LINEAR_TYPE float *blurred = allocate_image_processing_buffer(10, 10);
    apply_gaussian_blur(image, blurred, 10, 10);
    release_image_processing_buffer(blurred, 10, 10);
    release_image_processing_buffer(image, 10, 10);
    return 0;
}