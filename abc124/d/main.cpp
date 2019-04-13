#include <bits/stdc++.h>

using namespace std;

int main() {
    int n, k;
    vector<int> v;
    string s;
    cin >> n >> k >> s;
    if (s[0] == '0') v.push_back(0);
    char prev = s[0];
    int count = 0;
    for (auto c : s) {
        if (c != prev) {
            prev = c;
            v.push_back(count);
            count = 0;
        }
        ++count;
    }
    v.push_back(count);
    if (s[n - 1] == '0') v.push_back(0);
    int value = 0;
    for (int i = 0, size = v.size(); i < k * 2 + 1 && i < size; ++i) {
        value += v[i];
    }
    int answer = value;
    for (int i = 0, size = v.size(); i + 2 * k + 1 < size; i += 2) {
        value -= v[i] + v[i + 1];
        value += v[i + 2 * k + 1] + v[i + 2 * k + 1 + 1];
        answer = max(answer, value);
    }
    cout << answer << endl;
}
