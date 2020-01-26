#include <cmath>
#include <iostream>
#include <limits>
#include <vector>

using namespace std;

int main() {
    int h, n;
    cin >> h >> n;
    vector<int> dp(10000, numeric_limits<int>::max());
    vector<pair<int, int>> v;
    for (int i = 0; i < n; ++i) {
        int a, b;
        cin >> a >> b;
        v.push_back({a, b});
    }
    for (int i = 0; i < h; ++i) {
        for (int j = 0; j < n; ++j) {
            dp[i] = min(dp[i], v[j].second +
                                   (i >= v[j].first ? dp[i - v[j].first] : 0));
        }
    }
    cout << dp[h - 1] << endl;
}
