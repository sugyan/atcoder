#include <iostream>

using namespace std;

int main() {
    int k, x;
    cin >> k >> x;
    for (int a = x - k + 1; a < x + k; ++a) {
        cout << a;
        if (a < x + k - 1) cout << " ";
    }
    cout << endl;
}
