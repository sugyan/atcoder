#include <iostream>
#include <vector>
#include <algorithm>
#include <cmath>

using namespace std;

int main() {
    long long n, k;
    cin >> n >> k;
    vector<long long> a(n), f(n);
    for (int i = 0; i < n; ++i) cin >> a[i];
    for (int i = 0; i < n; ++i) cin >> f[i];
    long long sum = 0;
    for (int i = 0; i < n; ++i) sum += a[i];
    sort(a.rbegin(), a.rend());
    sort(f.begin(), f.end());
    long long l = 0, r = 0;
    for (int i = 0; i < n; ++i) r = max(r, a[i] * f[i]);
    long long target = sum - k;
    while (r - l > 0) {
        long long d = 0;
        long long m = l + (r - l) / 2;
        for (int i = 0; i < n; ++i) {
            d += min(a[i], m / f[i]);
        }
        if (d < target) {
            l = m + 1;
        } else {
            r = m;
        }
    }
    cout << l << endl;
}
