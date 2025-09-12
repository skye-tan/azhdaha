#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *generate_arithmetic_sequence(int first, int diff, int count) {
    LINEAR_TYPE int *sequence = malloc(count * sizeof(int));
    return sequence;
}

void fill_arithmetic_sequence(int *sequence, int first, int diff, int count) {
    for (int i = 0; i < count; i++) {
        sequence[i] = first + i * diff;
    }
}

int find_sequence_sum(int *sequence, int count) {
    int sum = 0;
    for (int i = 0; i < count; i++) {
        sum += sequence[i];
    }
    return sum;
}

void release_arithmetic_sequence(LINEAR_TYPE int *sequence, int count) {
    int total = find_sequence_sum(sequence, count);
    free(sequence);
}

int main() {
    LINEAR_TYPE int *arith_seq = generate_arithmetic_sequence(3, 4, 5);
    fill_arithmetic_sequence(arith_seq, 3, 4, 5);
    release_arithmetic_sequence(arith_seq, 5);
    return 0;
}