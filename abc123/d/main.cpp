#include <bits/stdc++.h>

using namespace std;

int main() {
    int x, y, z, k;
    long long ll;
    vector<long long> a, b, c;
    cin >> x >> y >> z >> k;
    for (int i = 0; i < x; ++i) {
        cin >> ll;
        a.push_back(ll);
    }
    for (int i = 0; i < y; ++i) {
        cin >> ll;
        b.push_back(ll);
    }
    for (int i = 0; i < z; ++i) {
        cin >> ll;
        c.push_back(ll);
    }
    sort(a.rbegin(), a.rend());
    sort(b.rbegin(), b.rend());
    sort(c.rbegin(), c.rend());
    vector<long long> v;
    for (int l = 0; l < x; ++l) {
        for (int m = 0; m < y; ++m) {
            for (int n = 0; n < z; ++n) {
                if ((l + 1) * (m + 1) * (n + 1) <= k) {
                    v.push_back(a[l] + b[m] + c[n]);
                } else {
                    break;
                }
            }
        }
    }
    sort(v.rbegin(), v.rend());
    for (int i = 0; i < k; ++i) {
        cout << v[i] << endl;
    }
}
