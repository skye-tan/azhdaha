#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE char *create_cipher_buffer(int length) {
    LINEAR_TYPE char *cipher = malloc(length * sizeof(char));
    return cipher;
}

void caesar_cipher_encrypt(char *text, int shift, int length) {
    for (int i = 0; i < length - 1; i++) {
        if (text[i] >= 'a' && text[i] <= 'z') {
            text[i] = ((text[i] - 'a' + shift) % 26) + 'a';
        } else if (text[i] >= 'A' && text[i] <= 'Z') {
            text[i] = ((text[i] - 'A' + shift) % 26) + 'A';
        }
    }
}

void caesar_cipher_decrypt(char *text, int shift, int length) {
    for (int i = 0; i < length - 1; i++) {
        if (text[i] >= 'a' && text[i] <= 'z') {
            text[i] = ((text[i] - 'a' - shift + 26) % 26) + 'a';
        } else if (text[i] >= 'A' && text[i] <= 'Z') {
            text[i] = ((text[i] - 'A' - shift + 26) % 26) + 'A';
        }
    }
}

void release_cipher_buffer(LINEAR_TYPE char *buffer) {
    caesar_cipher_decrypt(buffer, 3, 15);
    free(buffer);
}

int main() {
    LINEAR_TYPE char *message = create_cipher_buffer(15);
    for (int i = 0; i < 14; i++) {
        message[i] = "hello world"[i];
    }
    message[14] = '\0';
    caesar_cipher_encrypt(message, 3, 15);
    release_cipher_buffer(message);
    return 0;
}