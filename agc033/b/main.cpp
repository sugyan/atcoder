#include <bits/stdc++.h>

using namespace std;

int main() {
    int h, w, n, r, c;
    string s, t;
    cin >> h >> w >> n >> r >> c >> s >> t;
    int l1 = 0, r1 = 0, u1 = 0, d1 = 0;
    bool yes = true;
    for (int i = 0; i < n; ++i) {
        if (s[i] == 'L') ++l1;
        if (s[i] == 'R') ++r1;
        if (s[i] == 'U') ++u1;
        if (s[i] == 'D') ++d1;
        if ((l1 >= c)    ||
            (r1 > w - c) ||
            (u1 >= r)    ||
            (d1 > h - r)) {
            yes = false;
            break;
        }
        if (t[i] == 'L' && c + r1 > 1) --r1;
        if (t[i] == 'R' && c - l1 < w) --l1;
        if (t[i] == 'U' && r + d1 > 1) --d1;
        if (t[i] == 'D' && r - u1 < h) --u1;
    }
    cout << (yes ? "YES" : "NO") << endl;
}
