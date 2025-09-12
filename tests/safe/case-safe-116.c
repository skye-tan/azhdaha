#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE double *create_regression_data(int points) {
    LINEAR_TYPE double *data = malloc(points * 2 * sizeof(double));
    return data;
}

void generate_linear_data(double *data, int points, double slope,
                          double intercept) {
    for (int i = 0; i < points; i++) {
        data[i * 2] = (double)i;
        data[i * 2 + 1] = slope * data[i * 2] + intercept +
                          ((double)rand() / RAND_MAX - 0.5) * 2.0;
    }
}

double calculate_regression_slope(double *data, int points) {
    double sum_x = 0.0, sum_y = 0.0, sum_xy = 0.0, sum_x2 = 0.0;
    for (int i = 0; i < points; i++) {
        sum_x += data[i * 2];
        sum_y += data[i * 2 + 1];
        sum_xy += data[i * 2] * data[i * 2 + 1];
        sum_x2 += data[i * 2] * data[i * 2];
    }
    double numerator = points * sum_xy - sum_x * sum_y;
    double denominator = points * sum_x2 - sum_x * sum_x;
    return numerator / denominator;
}

void release_regression_data(LINEAR_TYPE double *data, int points) {
    double slope = calculate_regression_slope(data, points);
    free(data);
}

int main() {
    LINEAR_TYPE double *regression_data = create_regression_data(10);
    generate_linear_data(regression_data, 10, 2.5, 1.0);
    release_regression_data(regression_data, 10);
    return 0;
}