#include <azhdaha.h>
#include <math.h>
#include <stdlib.h>

LINEAR_TYPE double *create_interpolation_points(int points) {
    LINEAR_TYPE double *coordinates = malloc(points * 2 * sizeof(double));
    return coordinates;
}

void generate_sine_points(double *points, int count) {
    for (int i = 0; i < count; i++) {
        points[i * 2] = (double)i / (double)(count - 1) * 3.14159 * 2.0;
        points[i * 2 + 1] = sin(points[i * 2]);
    }
}

double interpolate_value(double *points, int count, double x) {
    for (int i = 0; i < count - 1; i++) {
        if (x >= points[i * 2] && x <= points[(i + 1) * 2]) {
            double ratio =
                (x - points[i * 2]) / (points[(i + 1) * 2] - points[i * 2]);
            return points[i * 2 + 1] +
                   ratio * (points[(i + 1) * 2 + 1] - points[i * 2 + 1]);
        }
    }
    return 0.0;
}

void release_interpolation_points(LINEAR_TYPE double *points, int count) {
    double interpolated = interpolate_value(points, count, 1.57);
    free(points);
}

int main() {
    LINEAR_TYPE double *sine_points = create_interpolation_points(10);
    generate_sine_points(sine_points, 10);
    release_interpolation_points(sine_points, 10);
    return 0;
}