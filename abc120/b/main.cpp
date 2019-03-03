#include <iostream>

using namespace std;

int gcd(int n, int m) {
    return n == 0 ? m : gcd(m % n, n);
}

int main() {
    int a, b, k;
    cin >> a >> b >> k;
    int result = gcd(a, b);
    for (int i = 1; i <= result; ++i) {
        if (result % i == 0) {
            k--;
            if (k == 0) {
                cout << result / i << endl;
                break;
            }
        }
    }
}