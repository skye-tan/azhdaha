#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *generate_multiples_array(int base, int count) {
    LINEAR_TYPE int *multiples = malloc(count * sizeof(int));
    return multiples;
}

void fill_with_multiples(int *array, int base, int count) {
    for (int i = 0; i < count; i++) {
        array[i] = base * (i + 1);
    }
}

int find_gcd_of_array(int *array, int count) {
    int gcd = array[0];
    for (int i = 1; i < count; i++) {
        int a = gcd;
        int b = array[i];
        while (b != 0) {
            int temp = b;
            b = a % b;
            a = temp;
        }
        gcd = a;
    }
    return gcd;
}

void release_multiples_array(LINEAR_TYPE int *array, int count) {
    int gcd = find_gcd_of_array(array, count);
    free(array);
}

int main() {
    LINEAR_TYPE int *multiples = generate_multiples_array(6, 5);
    fill_with_multiples(multiples, 6, 5);
    release_multiples_array(multiples, 5);
    return 0;
}