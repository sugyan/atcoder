#include <bits/stdc++.h>

using namespace std;

int a[100001], l[100001], r[100001];

int gcd(int a, int b) {
    return a == 0 ? b : gcd(b % a, a);
}

int main() {
    int n;
    cin >> n;
    for (int i = 0; i < n; ++i) {
        cin >> a[i];
    }
    l[0] = a[0];
    r[n - 1] = a[n - 1];
    for (int i = 0; i < n - 1; ++i) {
        l[i + 1] = gcd(l[i], a[i + 1]);
        r[n - 1 - i - 1] = gcd(r[n - 1 - i], a[n - 1 - i - 1]);
    }
    int answer = 0;
    for (int i = 0; i < n; ++i) {
        if (i == 0) {
            answer = max(answer, r[1]);
        } else if (i == n - 1) {
            answer = max(answer, l[n - 2]);
        } else {
            answer = max(answer, gcd(l[i - 1], r[i + 1]));
        }
    }
    cout << answer << endl;
}
