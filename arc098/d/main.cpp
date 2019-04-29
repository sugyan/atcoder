#include <bits/stdc++.h>

using namespace std;

int a[200001];

int main() {
    int n;
    cin >> n;
    for (int i = 0; i < n; ++i) {
        cin >> a[i];
    }
    long answer = n;
    int sum = sum = a[0];
    for (int l = 0, r = 0; l < n; ++l) {
        while (r + 1 < n && sum + a[r + 1] == (sum ^ a[r + 1])) {
            sum ^= a[++r];
        }
        answer += r - l;
        sum ^= a[l];
    }
    cout << answer << endl;
}
