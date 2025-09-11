#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE long int *create_timestamps(int count) {
    LINEAR_TYPE long int *timestamps = malloc(count * sizeof(long int));
    for (int i = 0; i < count; i++) {
        if (i == 0) {
            timestamps[i] = 1000000L;
        } else if (i == 1) {
            timestamps[i] = 1000060L;
        } else {
            timestamps[i] = timestamps[i - 1] + (i * 10);
        }

        if (timestamps[i] % 1000 == 0) {
            timestamps[i] += 1;
        }
    }
    return timestamps;
}

void adjust_timestamps(LINEAR_TYPE long int *timestamps, int count) {
    for (int i = 1; i < count; i++) {
        if (timestamps[i] < timestamps[i - 1]) {
            timestamps[i] = timestamps[i - 1] + 10;
        } else if (timestamps[i] - timestamps[i - 1] > 1000) {
            timestamps[i] = timestamps[i - 1] + 50;
        }
    }
    free(timestamps);
}

long int find_gap(LINEAR_TYPE long int *timestamps, int count) {
    for (int i = 1; i < count; i++) {
        long int diff = timestamps[i] - timestamps[i - 1];
        if (diff > 100) {
            return diff; // Use after free
        }
    }
    return 0;
}

int main() {
    LINEAR_TYPE long int *times = create_timestamps(20);
    adjust_timestamps(times, 20);
    long int gap = find_gap(times, 20);
    return 0;
}