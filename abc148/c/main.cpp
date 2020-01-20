#include <iostream>

using namespace std;

int gcd(long long a, long long b) {
    return a == 0 ? b : gcd(b % a, a);
}

int main() {
    long long a, b;
    cin >> a >> b;
    cout << a / gcd(a, b) * b << endl;
}
