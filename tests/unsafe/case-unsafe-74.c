#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE long int *create_timestamps(int count) {
    LINEAR_TYPE long int *timestamps = malloc(count * sizeof(long int));
    for (int i = 0; i < count; i++) {
        if (i == 0) {
            timestamps[i] = 1000000L;
        } else {
            timestamps[i] = timestamps[i - 1] + (i * 50);
        }

        if (timestamps[i] % 10000 == 0) {
            timestamps[i] += 1;
        }
    }
    return timestamps;
}

void adjust_timestamps(LINEAR_TYPE long int *timestamps, int count) {
    for (int i = 1; i < count; i++) {
        if (timestamps[i] < timestamps[i - 1]) {
            timestamps[i] = timestamps[i - 1] + 100;
        } else if (timestamps[i] - timestamps[i - 1] > 5000) {
            timestamps[i] = timestamps[i - 1] + 2500;
        }
    }
    free(timestamps);
}

long int find_max_gap(LINEAR_TYPE long int *timestamps, int count) {
    long int max_gap = 0L;
    for (int i = 1; i < count; i++) {
        long int gap = timestamps[i] - timestamps[i - 1];
        if (gap > max_gap) {
            max_gap = gap; // Use after free
        }
    }
    return max_gap;
}

int main() {
    LINEAR_TYPE long int *times = create_timestamps(25);
    adjust_timestamps(times, 25);
    long int max_gap = find_max_gap(times, 25);
    return 0;
}