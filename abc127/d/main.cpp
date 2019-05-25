#include <iostream>
#include <vector>
#include <map>
#include <algorithm>

using namespace std;

int main() {
    int n, m;
    cin >> n >> m;
    vector<int> a(n);
    for (int i = 0; i < n; ++i) {
        cin >> a[i];
    }
    map<int, int> mm;
    for (int i = 0; i < m; ++i) {
        long b, c;
        cin >> b >> c;
        mm[c] += b;
    }
    vector<pair<int, int>> v(mm.rbegin(), mm.rend());
    sort(a.begin(), a.end());
    long answer = 0;
    for (int i = 0, j = 0; i < n; ++i) {
        if (j < v.size() && v[j].first > a[i] && v[j].second > 0) {
            answer += v[j].first;
            if (--v[j].second == 0) ++j;
        } else {
            answer += a[i];
        }
    }
    cout << answer << endl;
}
