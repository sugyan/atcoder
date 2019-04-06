#include <bits/stdc++.h>

using namespace std;

int main() {
    int v[5], k;
    for (int i = 0; i < 5; ++i) {
        cin >> v[i];
    }
    cin >> k;
    if (v[4] - v[0] <= k) {
        cout << "Yay!" << endl;
    } else {
        cout << ":(" << endl;
    }
}
