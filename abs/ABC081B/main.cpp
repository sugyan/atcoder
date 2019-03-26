#include <iostream>

using namespace std;

long gcd(long a, long b) {
    return a == 0 ? b : gcd(b % a, a);
}

int main() {
    int n;
    long a[201];
    cin >> n;
    for (int i = 0; i < n; ++i) {
        cin >> a[i];
    }
    long g = a[0];
    for (int i = 0; i < n - 1; ++i) {
        g = gcd(g, a[i + 1]);
    }
    int answer = 0;
    while (g % 2 == 0) {
        ++answer;
        g /= 2;
    }
    cout << answer << endl;
}
