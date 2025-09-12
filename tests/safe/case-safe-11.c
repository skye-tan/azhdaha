#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE double *create_double_list(int count) {
    LINEAR_TYPE double *list = malloc(count * sizeof(double));
    return list;
}

void populate_with_powers_of_e(double *list, int count) {
    double e = 2.718281828;
    for (int i = 0; i < count; i++) {
        double result = 1.0;
        for (int j = 0; j < i; j++) {
            result *= e;
        }
        list[i] = result;
    }
}

double calculate_list_variance(double *list, int count) {
    double sum = 0.0;
    for (int i = 0; i < count; i++) {
        sum += list[i];
    }
    double mean = sum / count;
    double variance = 0.0;
    for (int i = 0; i < count; i++) {
        variance += (list[i] - mean) * (list[i] - mean);
    }
    return variance / count;
}

void release_double_list(LINEAR_TYPE double *list, int count) {
    double variance = calculate_list_variance(list, count);
    free(list);
}

int main() {
    LINEAR_TYPE double *values = create_double_list(7);
    populate_with_powers_of_e(values, 7);
    release_double_list(values, 7);
    return 0;
}