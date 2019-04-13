#include <bits/stdc++.h>

using namespace std;

int main() {
    int n, h, highest = 0, answer = 0;
    cin >> n;
    for (int i = 0; i < n; ++i) {
        cin >> h;
        if (h >= highest) {
            ++answer;
            highest = h;
        }
    }
    cout << answer << endl;
}
