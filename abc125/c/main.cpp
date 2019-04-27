#include <bits/stdc++.h>

using namespace std;

int a[100001];

int gcd(int a, int b) {
    return a == 0 ? b : gcd(b % a, a);
}

int main() {
    int n;
    cin >> n;
    for (int i = 0; i < n; ++i) {
        cin >> a[i];
    }
    int l = 0, r = n - 1;
    while (r - l > 3) {
        int m = (l + r) / 2;
        int g1 = a[0], g2 = a[m + 1];
        for (int i = 1; i < m + 1; ++i) {
            g1 = gcd(g1, a[i]);
        }
        for (int i = m + 1; i <= r; ++i) {
            g2 = gcd(g2, a[i]);
        }
        if (g1 < g2) {
            r = m;
        } else {
            l = m + 1;
        }
    }
    int answer = 0;
    for (int j = l; j <= r; ++j) {
        int g = a[0];
        if (j == 0) {
            g = a[1];
        }
        for (int i = 0; i < n; ++i) {
            if (i != j) g = gcd(g, a[i]);
        }
        answer = max(answer, g);
    }
    cout << answer << endl;
}
