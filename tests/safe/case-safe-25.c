#include <azhdaha.h>
#include <math.h>
#include <stdlib.h>

LINEAR_TYPE double *create_statistics_array(int size) {
    LINEAR_TYPE double *stats = malloc(size * sizeof(double));
    return stats;
}

void calculate_statistics(double *data, int size, double *results) {
    double sum = 0.0;
    for (int i = 0; i < size; i++) {
        sum += data[i];
    }
    results[0] = sum / size;

    double variance = 0.0;
    for (int i = 0; i < size; i++) {
        variance += (data[i] - results[0]) * (data[i] - results[0]);
    }
    results[1] = variance / size;
}

double find_standard_deviation(double *results) { return sqrt(results[1]); }

void release_statistics_array(LINEAR_TYPE double *array) {
    double std_dev = find_standard_deviation(array);
    free(array);
}

int main() {
    LINEAR_TYPE double *data = create_statistics_array(2);
    double sample[] = {1.0, 2.0, 3.0, 4.0, 5.0};
    calculate_statistics(sample, 5, data);
    release_statistics_array(data);
    return 0;
}