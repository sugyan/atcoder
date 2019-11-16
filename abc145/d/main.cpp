#include <iostream>

using namespace std;

const int MAX = 1000001;
long long fact[MAX], fact_inv[MAX];

const long long MOD = 1000000007l;

long long power(long long a, long long b) {
    long long res = 1;
    while (b > 0) {
        if (b & 1) res = res * a % MOD;
        a = a * a % MOD;
        b >>= 1;
    }
    return res;
}

long long comb(long long n, long long r) {
    return (fact[n] * fact_inv[r]) % MOD * fact_inv[n - r] % MOD;
}

int main() {
    int x, y;
    cin >> x >> y;
    if ((x + y) % 3 != 0 || x > y * 2 || y > x * 2) {
        cout << 0 << endl;
        return 0;
    }
    int n = (x + y) / 3;
    int d = min(x, y) - n;
    d = min(d, n - d);
    if (d == 0) {
        cout << 1 << endl;
        return 0;
    }

    fact[0] = 1;
    for (int i = 0; i < n; ++i) {
        fact[i + 1] = fact[i] * (i + 1) % MOD;
    }
    fact_inv[n] = power(fact[n], MOD - 2);
    for (int i = n - 1; i >= 0; --i) {
        fact_inv[i] = fact_inv[i + 1] * (i + 1) % MOD;
    }
    cout << comb(n, d) << endl;
}
