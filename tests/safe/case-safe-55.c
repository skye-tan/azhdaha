#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_fibonacci_sequence(int count) {
    LINEAR_TYPE int *sequence = malloc(count * sizeof(int));
    return sequence;
}

void generate_fibonacci_sequence(int *sequence, int count) {
    if (count > 0)
        sequence[0] = 0;
    if (count > 1)
        sequence[1] = 1;
    for (int i = 2; i < count; i++) {
        sequence[i] = sequence[i - 1] + sequence[i - 2];
    }
}

int find_fibonacci_max(int *sequence, int count) {
    int max = sequence[0];
    for (int i = 1; i < count; i++) {
        if (sequence[i] > max) {
            max = sequence[i];
        }
    }
    return max;
}

void release_fibonacci_sequence(LINEAR_TYPE int *sequence, int count) {
    int maximum = find_fibonacci_max(sequence, count);
    free(sequence);
}

int main() {
    LINEAR_TYPE int *fib_seq = create_fibonacci_sequence(15);
    generate_fibonacci_sequence(fib_seq, 15);
    release_fibonacci_sequence(fib_seq, 15);
    return 0;
}