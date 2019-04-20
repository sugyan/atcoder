#include <bits/stdc++.h>

using namespace std;

int main() {
    int n;
    string s;
    cin >> n >> s;
    int w = 0, b = 0;
    for (int i = 0; i < n; ++i) {
        if (s[i] == '#') ++b;
        if (s[i] == '.') ++w;
    }
    int answer = min(b, w);
    int bb = 0, ww = 0;
    for (int i = 0; i < n; ++i) {
        if (s[i] == '#') ++bb;
        if (s[i] == '.') ++ww;
        answer = min(answer, bb + (w - ww));
    }
    cout << answer << endl;
}
