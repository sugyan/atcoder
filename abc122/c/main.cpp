#include <iostream>

using namespace std;

int c[100005];

int main() {
    int n, q;
    int l, r;
    string s;
    cin >> n >> q;
    cin >> s;
    int count = 0;
    for (int i = 0; i < n - 1; ++i) {
        if (s[i] == 'A' && s[i + 1] == 'C') {
            ++count;
        }
        c[i + 1] = count;
    }
    c[n] = count;
    for (int i = 0; i < q; ++i) {
        cin >> l >> r;
        cout << c[r - 1] - c[l - 1] << endl;
    }
}