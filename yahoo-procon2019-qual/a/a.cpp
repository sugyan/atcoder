#include <iostream>

int main() {
    unsigned int n, k;
    std::cin >> n >> k;
    if (n >= k * 2 - 1)
        std::cout << "YES";
    else 
        std::cout << "NO";
    std::cout << std::endl;
}