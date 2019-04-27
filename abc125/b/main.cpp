#include <bits/stdc++.h>

using namespace std;

int main() {
    int n, c;
    int v[51];
    cin >> n;
    int answer = 0;
    for (int i = 0; i < n; ++i) cin >> v[i];
    for (int i = 0; i < n; ++i) {
        cin >> c;
        answer += max(0, v[i] - c);
    }
    cout << answer << endl;
}
