#include <azhdaha.h>
#include <stdlib.h>

void init_maps(LINEAR_TYPE int **map_a, LINEAR_TYPE int **map_b, int size) {
    *map_a = malloc(size * sizeof(int));
    *map_b = malloc(size * sizeof(int));
    for (int i = 0; i < size; i++) {
        if (i % 5 < 2) {
            (*map_a)[i] = i;
            (*map_b)[i] = -i;
        } else if (i % 5 < 4) {
            (*map_a)[i] = i * 2;
            (*map_b)[i] = -i * 2;
        } else {
            (*map_a)[i] = 0;
            (*map_b)[i] = 0;
        }
    }
}

void combine_maps(LINEAR_TYPE int *map_a, LINEAR_TYPE int *map_b, int size) {
    for (int i = 0; i < size; i++) {
        if (map_a[i] > 0 && map_b[i] < 0) {
            map_a[i] += map_b[i];
        } else if (map_a[i] < 0 && map_b[i] > 0) {
            map_b[i] += map_a[i];
        } else {
            map_a[i] = map_b[i] = 0;
        }
    }
    // Free map_b
    free(map_b);
}

void release_maps(LINEAR_TYPE int *map_a, LINEAR_TYPE int *map_b) {
    free(map_a);
    free(map_b); // Double free
}

int lookup_value(LINEAR_TYPE int *map_a, LINEAR_TYPE int *map_b, int index) {
    return map_a[index] + map_b[index]; // Use after free
}

int main() {
    LINEAR_TYPE int *map1, *map2;
    init_maps(&map1, &map2, 25);
    combine_maps(map1, map2, 25);
    release_maps(map1, map2);
    int value = lookup_value(map1, map2, 10);
    return 0;
}