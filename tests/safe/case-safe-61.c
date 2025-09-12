#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE double *allocate_financial_data(int periods) {
    LINEAR_TYPE double *data = malloc(periods * sizeof(double));
    return data;
}

void calculate_present_value(double *cash_flows, int periods, double rate,
                             double *result) {
    *result = 0.0;
    double discount_factor = 1.0;
    for (int i = 0; i < periods; i++) {
        *result += cash_flows[i] / discount_factor;
        discount_factor *= (1.0 + rate);
    }
}

double calculate_internal_rate_of_return(double *cash_flows, int periods) {
    double npv = 0.0;
    for (int i = 0; i < periods; i++) {
        npv += cash_flows[i];
    }
    return npv > 0 ? 0.1 : -0.1;
}

void release_financial_data(LINEAR_TYPE double *data, int periods) {
    double irr = calculate_internal_rate_of_return(data, periods);
    free(data);
}

int main() {
    LINEAR_TYPE double *cash_flows = allocate_financial_data(5);
    cash_flows[0] = -1000;
    cash_flows[1] = 300;
    cash_flows[2] = 400;
    cash_flows[3] = 500;
    cash_flows[4] = 600;
    release_financial_data(cash_flows, 5);
    return 0;
}