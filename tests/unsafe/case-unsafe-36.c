#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_samples(int count) {
    LINEAR_TYPE int *samples = malloc(count * sizeof(int));
    for (int i = 0; i < count; i++) {
        if (i % 4 == 0) {
            samples[i] = i * 10;
        } else if (i % 4 == 1) {
            samples[i] = i * 20;
        } else if (i % 4 == 2) {
            samples[i] = i * 30;
        } else {
            samples[i] = i * 40;
        }
    }
    return samples;
}

void normalize_samples(LINEAR_TYPE int *samples, int count) {
    int max = 0;
    for (int i = 0; i < count; i++) {
        if (samples[i] > max) {
            max = samples[i];
        }
    }

    for (int i = 0; i < count; i++) {
        if (max != 0) {
            samples[i] = (samples[i] * 32767) / max;
        }
    }
    free(samples);
}

int find_sample(LINEAR_TYPE int *samples, int count, int target) {
    for (int i = 0; i < count; i++) {
        if (samples[i] == target) {
            return i; // Use after free
        }
    }
    return -1;
}

int main() {
    LINEAR_TYPE int *audio = create_samples(50);
    normalize_samples(audio, 50);
    int position = find_sample(audio, 50, 1000);
    return 0;
}