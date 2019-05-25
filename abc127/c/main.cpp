#include <iostream>

using namespace std;

int main() {
    int n, m;
    cin >> n >> m;
    int ll = 0, rr = n;
    for (int i = 0; i < m; ++i) {
        int l, r;
        cin >> l >> r;
        ll = max(ll, l);
        rr = min(rr, r);
    }
    cout << (ll <= rr ? rr - ll + 1 : 0) << endl;
}
