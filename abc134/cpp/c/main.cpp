#include <iostream>

using namespace std;

int main() {
    int n;
    cin >> n;
    int m1 = 0, m2 = 0, midx = -1;
    bool all = false;
    for (int i = 0; i < n; ++i) {
        int a;
        cin >> a;
        if (a >= m1) {
            midx = i;
            m2 = m1;
            m1 = a;
            all = m1 == m2;
        } else if (a > m2) {
            m2 = a;
        }
    }
    for (int i = 0; i < n; ++i) {
        if (all) {
            cout << m1 << endl;
        } else {
            cout << (i == midx ? m2 : m1) << endl;
        }
    }
}
