#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *generate_prime_array(int count) {
    LINEAR_TYPE int *primes = malloc(count * sizeof(int));
    return primes;
}

void fill_with_primes(int *primes, int count) {
    int num = 2;
    int found = 0;
    while (found < count) {
        int is_prime = 1;
        for (int i = 2; i * i <= num; i++) {
            if (num % i == 0) {
                is_prime = 0;
                break;
            }
        }
        if (is_prime) {
            primes[found] = num;
            found++;
        }
        num++;
    }
}

int sum_primes(int *primes, int count) {
    int sum = 0;
    for (int i = 0; i < count; i++) {
        sum += primes[i];
    }
    return sum;
}

void release_prime_array(LINEAR_TYPE int *primes, int count) {
    int total = sum_primes(primes, count);
    free(primes);
}

int main() {
    LINEAR_TYPE int *prime_numbers = generate_prime_array(10);
    fill_with_primes(prime_numbers, 10);
    release_prime_array(prime_numbers, 10);
    return 0;
}