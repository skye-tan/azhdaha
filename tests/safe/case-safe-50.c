#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *create_scheduler_queue(int tasks) {
    LINEAR_TYPE int *queue = malloc(tasks * 3 * sizeof(int));
    return queue;
}

void initialize_task_queue(int *queue, int tasks) {
    for (int i = 0; i < tasks; i++) {
        queue[i * 3] = i;
        queue[i * 3 + 1] = (i + 1) * 10;
        queue[i * 3 + 2] = (i + 1) * 5;
    }
}

int schedule_next_task(int *queue, int tasks) {
    int highest_priority = -1;
    int task_index = -1;
    for (int i = 0; i < tasks; i++) {
        if (queue[i * 3 + 1] > highest_priority) {
            highest_priority = queue[i * 3 + 1];
            task_index = i;
        }
    }
    return task_index;
}

void update_task_priority(int *queue, int tasks, int task_id,
                          int new_priority) {
    for (int i = 0; i < tasks; i++) {
        if (queue[i * 3] == task_id) {
            queue[i * 3 + 1] = new_priority;
            return;
        }
    }
}

void release_scheduler_queue(LINEAR_TYPE int *queue, int tasks) {
    int next_task = schedule_next_task(queue, tasks);
    free(queue);
}

int main() {
    LINEAR_TYPE int *task_queue = create_scheduler_queue(10);
    initialize_task_queue(task_queue, 10);
    update_task_priority(task_queue, 10, 5, 100);
    release_scheduler_queue(task_queue, 10);
    return 0;
}