#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE double *create_double_sequence(int count) {
    LINEAR_TYPE double *sequence = malloc(count * sizeof(double));
    return sequence;
}

void fill_with_geometric_progression(double *sequence, int count) {
    double ratio = 1.5;
    sequence[0] = 1.0;
    for (int i = 1; i < count; i++) {
        sequence[i] = sequence[i - 1] * ratio;
    }
}

double calculate_geometric_sum(double *sequence, int count) {
    double sum = 0.0;
    for (int i = 0; i < count; i++) {
        sum += sequence[i];
    }
    return sum;
}

void release_double_sequence(LINEAR_TYPE double *sequence, int count) {
    double total = calculate_geometric_sum(sequence, count);
    free(sequence);
}

int main() {
    LINEAR_TYPE double *progression = create_double_sequence(6);
    fill_with_geometric_progression(progression, 6);
    release_double_sequence(progression, 6);
    return 0;
}