#include "doubler.h"
#include <iostream>

extern const int FACTOR;
extern "C" {
    int doubler(int x) {
        std::cout << "doubler function runs... \n";
        return x * FACTOR;
    }
}