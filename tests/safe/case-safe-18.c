#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_collatz_sequence(int start) {
    LINEAR_TYPE int *sequence = malloc(50 * sizeof(int));
    return sequence;
}

int generate_collatz_sequence(int *sequence, int start) {
    int n = start;
    int index = 0;
    while (n != 1 && index < 50) {
        sequence[index] = n;
        if (n % 2 == 0) {
            n = n / 2;
        } else {
            n = 3 * n + 1;
        }
        index++;
    }
    if (index < 50) {
        sequence[index] = 1;
        index++;
    }
    return index;
}

int find_max_in_sequence(int *sequence, int length) {
    int max = sequence[0];
    for (int i = 1; i < length; i++) {
        if (sequence[i] > max) {
            max = sequence[i];
        }
    }
    return max;
}

void release_collatz_sequence(LINEAR_TYPE int *sequence) {
    int max_value = find_max_in_sequence(sequence, 50);
    free(sequence);
}

int main() {
    LINEAR_TYPE int *collatz = create_collatz_sequence(7);
    int length = generate_collatz_sequence(collatz, 7);
    release_collatz_sequence(collatz);
    return 0;
}