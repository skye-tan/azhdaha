#include <azhdaha.h>
#include <math.h>
#include <stdlib.h>

LINEAR_TYPE double *allocate_statistics_buffer(int samples) {
    LINEAR_TYPE double *buffer = malloc(samples * sizeof(double));
    return buffer;
}

void calculate_moments(double *data, int samples, double *results) {
    double mean = 0.0;
    for (int i = 0; i < samples; i++) {
        mean += data[i];
    }
    mean /= samples;
    results[0] = mean;

    double variance = 0.0;
    for (int i = 0; i < samples; i++) {
        variance += (data[i] - mean) * (data[i] - mean);
    }
    results[1] = variance / samples;

    double skewness = 0.0;
    for (int i = 0; i < samples; i++) {
        skewness += pow(data[i] - mean, 3);
    }
    results[2] = skewness / (samples * pow(sqrt(variance / samples), 3));
}

double calculate_correlation(double *x, double *y, int samples) {
    double mean_x = 0.0, mean_y = 0.0;
    for (int i = 0; i < samples; i++) {
        mean_x += x[i];
        mean_y += y[i];
    }
    mean_x /= samples;
    mean_y /= samples;

    double numerator = 0.0, denom_x = 0.0, denom_y = 0.0;
    for (int i = 0; i < samples; i++) {
        numerator += (x[i] - mean_x) * (y[i] - mean_y);
        denom_x += (x[i] - mean_x) * (x[i] - mean_x);
        denom_y += (y[i] - mean_y) * (y[i] - mean_y);
    }
    return numerator / sqrt(denom_x * denom_y);
}

void release_statistics_buffer(LINEAR_TYPE double *buffer, int samples) {
    LINEAR_TYPE double *moments = allocate_statistics_buffer(3);
    calculate_moments(buffer, samples, moments);
    free(moments);
    free(buffer);
}

int main() {
    LINEAR_TYPE double *data_x = allocate_statistics_buffer(10);
    LINEAR_TYPE double *data_y = allocate_statistics_buffer(10);
    for (int i = 0; i < 10; i++) {
        data_x[i] = (double)i;
        data_y[i] = (double)(i * 2);
    }
    double correlation = calculate_correlation(data_x, data_y, 10);
    release_statistics_buffer(data_x, 10);
    release_statistics_buffer(data_y, 10);
    return 0;
}