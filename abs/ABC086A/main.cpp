#include <iostream>
using namespace std;

int main() {
    unsigned int a, b;
    cin >> a >> b;
    cout << ((a * b % 2 == 0) ? "Even" : "Odd") << endl;
    return 0;
}