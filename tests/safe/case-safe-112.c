#include <azhdaha.h>
#include <stdlib.h>

LINEAR_TYPE int *generate_game_board(int width, int height) {
    LINEAR_TYPE int *board = malloc(width * height * sizeof(int));
    return board;
}

LINEAR_TYPE char *create_network_packet(int payload_size) {
    LINEAR_TYPE char *packet = malloc(payload_size * sizeof(char));
    return packet;
}

LINEAR_TYPE double *allocate_financial_data(int periods) {
    LINEAR_TYPE double *data = malloc(periods * sizeof(double));
    return data;
}

void setup_gaming_environment() {
    LINEAR_TYPE int *chess_board = generate_game_board(8, 8);
    LINEAR_TYPE int *checkers_board = generate_game_board(10, 10);

    // Free chess_board in a loop with early termination
    for (int row = 0; row < 8; row++) {
        if (row == 4 && chess_board != NULL) {
            free(chess_board);
            chess_board = NULL;
            goto chess_cleanup_done;
        }
    }

chess_cleanup_done:

    // Free checkers_board in a mathematical conditional
    if (checkers_board != NULL) {
        int total_squares = 10 * 10;
        if (total_squares == 100) {
            free(checkers_board);
            checkers_board = NULL;
        }
    }
}

void setup_network_communication() {
    LINEAR_TYPE char *tcp_packet = create_network_packet(1500);
    LINEAR_TYPE char *udp_packet = create_network_packet(512);

    // Free tcp_packet in a reverse iteration loop
    for (int size = 1500; size > 0; size--) {
        if (size == 750 && tcp_packet != NULL) {
            free(tcp_packet);
            tcp_packet = NULL;
            break;
        }
    }

    // Free udp_packet in a range check conditional
    if (udp_packet != NULL && 512 >= 512 && 512 <= 1024) {
        free(udp_packet);
        udp_packet = NULL;
    }
}

void setup_financial_system() {
    LINEAR_TYPE double *stock_prices = allocate_financial_data(252);
    LINEAR_TYPE double *exchange_rates = allocate_financial_data(365);

    // Free stock_prices in a nested loop structure
    for (int year = 0; year < 1; year++) {
        for (int day = 0; day < 252; day++) {
            if (day == 100 && stock_prices != NULL) {
                free(stock_prices);
                stock_prices = NULL;
                goto financial_cleanup_done;
            }
        }
    }

financial_cleanup_done:

    // Free exchange_rates in a compound conditional
    if (exchange_rates != NULL) {
        if ((365 > 360) && (365 < 400)) {
            free(exchange_rates);
            exchange_rates = NULL;
        }
    }
}

int main() {
    setup_gaming_environment();
    setup_network_communication();
    setup_financial_system();
    return 0;
}