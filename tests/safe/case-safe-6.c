#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE double *create_polynomial_coefficients(int degree) {
    LINEAR_TYPE double *coeffs = malloc((degree + 1) * sizeof(double));
    return coeffs;
}

void initialize_polynomial(double *coeffs, int degree) {
    for (int i = 0; i <= degree; i++) {
        coeffs[i] = (double)(i + 1) / 2.0;
    }
}

double evaluate_polynomial(double *coeffs, int degree, double x) {
    double result = 0.0;
    double power = 1.0;
    for (int i = 0; i <= degree; i++) {
        result += coeffs[i] * power;
        power *= x;
    }
    return result;
}

void release_polynomial(LINEAR_TYPE double *coeffs, int degree) {
    double value = evaluate_polynomial(coeffs, degree, 2.0);
    free(coeffs);
}

int main() {
    LINEAR_TYPE double *poly = create_polynomial_coefficients(4);
    initialize_polynomial(poly, 4);
    release_polynomial(poly, 4);
    return 0;
}