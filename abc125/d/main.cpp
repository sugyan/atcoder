#include <bits/stdc++.h>

using namespace std;

int a[100001];

int main() {
    int n, m = numeric_limits<int>::max();
    cin >> n;
    long answer = 0;
    int negative = 0;
    for (int i = 0; i < n; ++i) {
        cin >> a[i];
        if (a[i] < 0) ++negative;
        m = min(m, abs(a[i]));
        answer += abs(a[i]);
    }
    if (negative % 2 == 1) answer -= 2 * m;
    cout << answer << endl;
}
