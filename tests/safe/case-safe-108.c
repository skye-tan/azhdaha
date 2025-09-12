#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_priority_queue(int size) {
    LINEAR_TYPE int *queue = malloc(size * sizeof(int));
    return queue;
}

void heapify_up(int *heap, int index) {
    while (index > 0) {
        int parent = (index - 1) / 2;
        if (heap[index] <= heap[parent]) {
            break;
        }
        int temp = heap[index];
        heap[index] = heap[parent];
        heap[parent] = temp;
        index = parent;
    }
}

void insert_into_heap(int *heap, int *size, int value) {
    heap[*size] = value;
    heapify_up(heap, *size);
    (*size)++;
}

int extract_max(int *heap, int *size) {
    if (*size <= 0) {
        return -1;
    }
    int max = heap[0];
    heap[0] = heap[*size - 1];
    (*size)--;
    return max;
}

void release_priority_queue(LINEAR_TYPE int *queue, int size) {
    int max_value = extract_max(queue, &size);
    free(queue);
}

int main() {
    int heap_size = 0;
    LINEAR_TYPE int *priority_queue = create_priority_queue(10);
    insert_into_heap(priority_queue, &heap_size, 5);
    insert_into_heap(priority_queue, &heap_size, 10);
    insert_into_heap(priority_queue, &heap_size, 3);
    release_priority_queue(priority_queue, heap_size);
    return 0;
}