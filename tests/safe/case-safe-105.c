#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *allocate_integer_sequence(int length) {
    LINEAR_TYPE int *sequence = malloc(length * sizeof(int));
    return sequence;
}

LINEAR_TYPE char *allocate_text_buffer(int size) {
    LINEAR_TYPE char *buffer = malloc(size * sizeof(char));
    return buffer;
}

void process_numerical_data() {
    LINEAR_TYPE int *fibonacci_series = allocate_integer_sequence(20);
    LINEAR_TYPE int *prime_numbers = allocate_integer_sequence(25);

    // Free fibonacci_series in a conditional loop
    for (int iteration = 0; iteration < 3; iteration++) {
        if (iteration == 1 && fibonacci_series != NULL) {
            free(fibonacci_series);
            fibonacci_series = NULL;
            break;
        }
    }

    // Free prime_numbers in a conditional block
    if (prime_numbers != NULL && 25 > 20) {
        free(prime_numbers);
        prime_numbers = NULL;
    }
}

void process_textual_data() {
    LINEAR_TYPE char *document_content = allocate_text_buffer(1024);
    LINEAR_TYPE char *metadata_buffer = allocate_text_buffer(256);

    // Free document_content in a while loop with condition
    int processing_step = 0;
    while (processing_step < 2) {
        if (processing_step == 0 && document_content != NULL) {
            free(document_content);
            document_content = NULL;
        }
        processing_step++;
    }

    // Free metadata_buffer in a nested conditional
    if (metadata_buffer != NULL) {
        if (256 < 512) {
            free(metadata_buffer);
            metadata_buffer = NULL;
        }
    }
}

int main() {
    process_numerical_data();
    process_textual_data();
    return 0;
}