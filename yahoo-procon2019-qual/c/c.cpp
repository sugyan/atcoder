#include <iostream>

int main() {
    long n = 0;
    long k, a, b;
    std::cin >> k >> a >> b;
    if (k - a + 1 > 0 && b - a > 2) {
        n = (k - a + 1) / 2;
        std::cout << a + n * (b - a) + (k - a + 1) % 2;
    } else {
        std::cout << k + 1;
    }
    std::cout << std::endl;
}