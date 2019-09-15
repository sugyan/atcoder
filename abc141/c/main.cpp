#include <iostream>
#include <vector>

using namespace std;

int main() {
    int n, k, q;
    cin >> n >> k >> q;
    vector<int> v(n, k - q);
    for (int i = 0; i < q; ++i) {
        int a;
        cin >> a;
        ++v[a - 1];
    }
    for (int i = 0; i < n; ++i) {
        if (v[i] > 0) {
            cout << "Yes" << endl;
        } else {
            cout << "No" <<  endl;
        }
    }
}
