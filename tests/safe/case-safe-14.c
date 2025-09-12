#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_histogram_array(int bins) {
    LINEAR_TYPE int *histogram = malloc(bins * sizeof(int));
    return histogram;
}

void fill_histogram(int *hist, int bins, int *data, int data_size) {
    for (int i = 0; i < bins; i++) {
        hist[i] = 0;
    }
    for (int i = 0; i < data_size; i++) {
        int bin_index = data[i] % bins;
        hist[bin_index]++;
    }
}

int find_peak_bin(int *hist, int bins) {
    int peak = 0;
    int peak_index = 0;
    for (int i = 0; i < bins; i++) {
        if (hist[i] > peak) {
            peak = hist[i];
            peak_index = i;
        }
    }
    return peak_index;
}

void release_histogram(LINEAR_TYPE int *hist, int bins) {
    int peak = find_peak_bin(hist, bins);
    free(hist);
}

int main() {
    int sample_data[] = {1, 3, 2, 5, 4, 6, 7, 8, 9, 1};
    LINEAR_TYPE int *histogram = create_histogram_array(5);
    fill_histogram(histogram, 5, sample_data, 10);
    release_histogram(histogram, 5);
    return 0;
}