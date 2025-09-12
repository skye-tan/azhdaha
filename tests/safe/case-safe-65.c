#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *allocate_convolution_kernel(int size) {
    LINEAR_TYPE int *kernel = malloc(size * sizeof(int));
    return kernel;
}

void initialize_gaussian_kernel_1d(int *kernel, int size) {
    int center = size / 2;
    for (int i = 0; i < size; i++) {
        kernel[i] = (size / 2) - abs(i - center);
    }
}

int apply_convolution(int *kernel, int k_size, int *signal, int s_size) {
    int result = 0;
    int k_center = k_size / 2;
    for (int i = 0; i < k_size && i < s_size; i++) {
        int s_index = i - k_center;
        if (s_index >= 0 && s_index < s_size) {
            result += kernel[i] * signal[s_index];
        }
    }
    return result;
}

void release_convolution_kernel(LINEAR_TYPE int *kernel, int size) {
    int convolved = apply_convolution(kernel, size, NULL, 0);
    free(kernel);
}

int main() {
    int signal[] = {1, 2, 3, 4, 5};
    LINEAR_TYPE int *kernel = allocate_convolution_kernel(3);
    initialize_gaussian_kernel_1d(kernel, 3);
    release_convolution_kernel(kernel, 3);
    return 0;
}