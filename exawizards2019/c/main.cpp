#include <iostream>
#include <vector>

using namespace std;

int n, q;
string s;
int g[200005] = { 0 };
char t[200005], d[200005];
vector<int> idx[26];

int after(int x) {
    for (int i = 0; i < q; ++i) {
        if (s[x] == t[i]) {
            if (d[i] == 'R') ++x;
            if (d[i] == 'L') --x;
        }
        if (x < 0 || x >= n) {
            return x;
        }
    }
    return x;
}

int main() {
    cin >> n >> q >> s;
    for (int i = 0; i < q; ++i) {
        cin >> t[i] >> d[i];
    }
    int ll, rr;
    if (after(0) >= 0) {
        ll = 0;
    } else if (after(n - 1) < 0) {
        ll = n;
    } else {
        int l = 0, r = n - 1, m;
        while (abs(l - r) > 1) {
            m = (l + r) / 2;
            if (after(m) < 0) {
                l = m;
            } else {
                r = m;
            }
        }
        ll = r;
    }
    if (after(0) > n - 1) {
        rr = 0;
    } else if (after(n - 1) < n) {
        rr = n;
    } else {
        int l = 0, r = n - 1, m;
        while (abs(l - r) > 1) {
            m = (l + r) / 2;
            if (after(m) <= n - 1) {
                l = m;
            } else {
                r = m;
            }
        }
        rr = r;
    }
    cout << rr - ll << endl;
}
