#include <azhdaha.h>
#include <stdlib.h>

void init_data(LINEAR_TYPE long long **data1, LINEAR_TYPE long long **data2,
               int size) {
    *data1 = malloc(size * sizeof(long long));
    *data2 = malloc(size * sizeof(long long));
    for (int i = 0; i < size; i++) {
        if (i % 3 == 0) {
            (*data1)[i] = i * 1000000LL;
            (*data2)[i] = -i * 1000000LL;
        } else if (i % 3 == 1) {
            (*data1)[i] = i * 1000LL;
            (*data2)[i] = -i * 1000LL;
        } else {
            (*data1)[i] = 0LL;
            (*data2)[i] = 0LL;
        }
    }
}

void process_data(LINEAR_TYPE long long *data1, LINEAR_TYPE long long *data2,
                  int size) {
    for (int i = 0; i < size; i++) {
        if (data1[i] > 0LL && data2[i] < 0LL) {
            long long temp = data1[i];
            data1[i] = data2[i];
            data2[i] = temp;
        } else if (data1[i] == 0LL) {
            data1[i] = 1LL;
        } else if (data2[i] == 0LL) {
            data2[i] = -1LL;
        }
    }
    free(data1);
}

void release_data(LINEAR_TYPE long long *data1, LINEAR_TYPE long long *data2) {
    free(data1); // Double free
    free(data2);
}

long long find_sum(LINEAR_TYPE long long *data1, LINEAR_TYPE long long *data2,
                   int size) {
    long long sum = 0LL;
    for (int i = 0; i < size; i++) {
        sum += data1[i] + data2[i]; // Use after free
    }
    return sum;
}

int main() {
    LINEAR_TYPE long long *d1, *d2;
    init_data(&d1, &d2, 20);
    process_data(d1, d2, 20);
    release_data(d1, d2);
    long long total = find_sum(d1, d2, 20);
    return 0;
}