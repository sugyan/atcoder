#include <bits/stdc++.h>

using namespace std;

int a[1001][1001];

int main() {
    int h, w;
    cin >> h >> w;
    for (int i = 0; i < h; ++i) {
        string s;
        cin >> s;
        for (int j = 0; j < w; ++j) {
            a[i][j] = s[j] == '#' ? 0 : 2001;
        }
    }
    for (int i = 0; i < h; ++i) {
        for (int j = 1; j < w; ++j) {
            a[i][j] = min(a[i][j], a[i][j - 1] + 1);
        }
        for (int j = w - 2; j >= 0; --j) {
            a[i][j] = min(a[i][j], a[i][j + 1] + 1);
        }
    }
    for (int j = 0; j < w; ++j) {
        for (int i = 1; i < h; ++i) {
            a[i][j] = min(a[i][j], a[i - 1][j] + 1);
        }
        for (int i = h - 2; i >= 0; --i) {
            a[i][j] = min(a[i][j], a[i + 1][j] + 1);
        }
    }
    int answer = 0;
    for (int i = 0; i < h; ++i) {
        for (int j = 0; j < w; ++j) {
            answer = max(answer, a[i][j]);
        }
    }
    cout << answer << endl;
}
