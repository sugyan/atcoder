#include <iostream>

using namespace std;

int main() {
    long long x, k, d;
    cin >> x >> k >> d;
    x = abs(x);
    if (k > x / d) {
        k -= x / d;
        x -= d * (x / d);
        if (k % 2 == 1) {
            cout << min(abs(x + d), abs(x - d)) << endl;
        } else {
            cout << min(x, abs(x - d * 2)) << endl;
        }
    } else {
        cout << x - d * k << endl;
    }
}
