#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *generate_primes(int limit) {
    LINEAR_TYPE int *primes = malloc(limit * sizeof(int));
    int count = 0;
    for (int i = 2; i < limit; i++) {
        int is_prime = 1;
        for (int j = 2; j * j <= i; j++) {
            if (i % j == 0) {
                is_prime = 0;
                break;
            }
        }
        if (is_prime) {
            primes[count++] = i;
        }
    }
    return primes;
}

void filter_primes(LINEAR_TYPE int *primes, int count) {
    for (int i = 0; i < count; i++) {
        if (primes[i] < 10) {
            primes[i] = 0;
        }
    }
    free(primes);
}

int sum_large_primes(LINEAR_TYPE int *primes, int count) {
    int sum = 0;
    for (int i = 0; i < count; i++) {
        if (primes[i] > 50) {
            sum += primes[i]; // Use after free
        }
    }
    return sum;
}

int main() {
    LINEAR_TYPE int *prime_list = generate_primes(100);
    filter_primes(prime_list, 100);
    int large_sum = sum_large_primes(prime_list, 100);
    return 0;
}