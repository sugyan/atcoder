#include <cmath>
#include <iomanip>
#include <iostream>
#include <vector>

using namespace std;

int main() {
    int n;
    cin >> n;
    vector<pair<int, int>> v;
    for (int i = 0; i < n; ++i) {
        int x, y;
        cin >> x >> y;
        v.push_back({x, y});
    }
    double d = 0.0;
    for (int i = 0; i < n - 1; ++i) {
        for (int j = i + 1; j < n; ++j) {
            d +=
                sqrt((v[i].first - v[j].first) * (v[i].first - v[j].first) +
                     (v[i].second - v[j].second) * (v[i].second - v[j].second));
        }
    }
    double answer = d * (n - 1) / (n * (n - 1) / 2);
    cout << setprecision(15) << answer << endl;
}
