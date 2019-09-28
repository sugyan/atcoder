#include <iostream>
#include <iomanip>

using namespace std;

int main() {
    int n;
    cin >> n;
    int odd = n / 2;
    if (n % 2 != 0) ++odd;
    cout << setprecision(8) << 1.0 * odd / n << endl;
}
