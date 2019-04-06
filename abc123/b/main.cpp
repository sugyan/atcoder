#include <bits/stdc++.h>

using namespace std;

int main() {
    int v[5], answer = 0, last = 5, min = 10;
    for (int i = 0; i < 5; ++i) {
        cin >> v[i];
        answer += v[i];
        int d = v[i] % 10;
        if (d > 0 && d < min) {
            min = d;
            last = i;
        }
    }
    for (int i = 0; i < 5; ++i) {
        if (i != last && v[i] % 10 != 0) {
            answer += 10 - v[i] % 10;
        }
    }
    cout << answer << endl;
}
