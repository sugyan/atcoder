#include <iostream>

using namespace std;

int main() {
    int n, k;
    cin >> n >> k;
    int l = (n - 1) * (n - 2) / 2;
    if (k > l) {
        cout << -1 << endl;
        return 0;
    }
    cout << n - 1 + l - k << endl;
    for (int i = 0; i < n - 1; ++i) {
        cout << 1 << " " << i + 2 << endl;
    }
    int i = 2, j = 3;
    for (int m = l - k; m > 0; --m) {
        cout << i << " " << j << endl;
        if (++j > n) {
            ++i;
            j = i + 1;
        };
    }
}
