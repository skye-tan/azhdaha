#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_prime_sieve(int limit) {
    LINEAR_TYPE int *sieve = malloc(limit * sizeof(int));
    return sieve;
}

void initialize_sieve(int *sieve, int limit) {
    for (int i = 0; i < limit; i++) {
        sieve[i] = 1;
    }
    sieve[0] = sieve[1] = 0;
    for (int i = 2; i * i < limit; i++) {
        if (sieve[i]) {
            for (int j = i * i; j < limit; j += i) {
                sieve[j] = 0;
            }
        }
    }
}

int count_primes(int *sieve, int limit) {
    int count = 0;
    for (int i = 0; i < limit; i++) {
        if (sieve[i]) {
            count++;
        }
    }
    return count;
}

void release_sieve(LINEAR_TYPE int *sieve, int limit) {
    int prime_count = count_primes(sieve, limit);
    free(sieve);
}

int main() {
    LINEAR_TYPE int *prime_sieve = create_prime_sieve(100);
    initialize_sieve(prime_sieve, 100);
    release_sieve(prime_sieve, 100);
    return 0;
}