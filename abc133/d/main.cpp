#include <iostream>
#include <vector>

using namespace std;

int main() {
    int n;
    cin >> n;
    vector<long> a(n);
    long m = 0;
    for (int i = 0; i < n; ++i) {
        cin >> a[i];
        m = a[i] * 2 - m;
    }
    m /= 2;
    for (int i = 0; i < n; ++i) {
        cout << m;
        if (i < n - 1) {
            cout << " ";
            m = a[i] * 2 - m;
        }
    }
    cout << endl;
}
