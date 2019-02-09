#include <iostream>

int main() {
    unsigned int a, b;
    unsigned int c[4] = { 0, 0, 0, 0 };
    unsigned int d1 = 0, d2 = 0;
    for (int i = 0; i < 3; ++i) {
        std::cin >> a >> b;
        c[a - 1]++;
        c[b - 1]++;
    }
    for (int i = 0; i < 4; i++) {
        if (c[i] == 1) d1++;
        if (c[i] == 2) d2++;
    }
    if (d1 == 2 && d2 == 2)
        std::cout << "YES";
    else
        std::cout << "NO";
    std::cout << std::endl;
}