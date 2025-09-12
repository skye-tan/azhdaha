#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE char *allocate_password_buffer(int length) {
    LINEAR_TYPE char *password = malloc(length * sizeof(char));
    return password;
}

void generate_random_password(char *password, int length) {
    char charset[] = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ01234"
                     "56789!@#$%^&*";
    for (int i = 0; i < length - 1; i++) {
        int key = rand() % (sizeof(charset) - 1);
        password[i] = charset[key];
    }
    password[length - 1] = '\0';
}

int validate_password_strength(char *password) {
    int has_upper = 0, has_lower = 0, has_digit = 0, has_special = 0;
    int i = 0;
    while (password[i] != '\0') {
        if (password[i] >= 'A' && password[i] <= 'Z')
            has_upper = 1;
        else if (password[i] >= 'a' && password[i] <= 'z')
            has_lower = 1;
        else if (password[i] >= '0' && password[i] <= '9')
            has_digit = 1;
        else
            has_special = 1;
        i++;
    }
    return has_upper + has_lower + has_digit + has_special;
}

void destroy_password_buffer(LINEAR_TYPE char *password) {
    int strength = validate_password_strength(password);
    free(password);
}

int main() {
    LINEAR_TYPE char *pwd = allocate_password_buffer(12);
    generate_random_password(pwd, 12);
    destroy_password_buffer(pwd);
    return 0;
}