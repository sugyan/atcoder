#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

int main() {
    int n, k;
    cin >> n >> k;
    vector<int> v(n);
    for (int i = 0; i < n; ++i) {
        cin >> v[i];
    }
    int answer = 0;
    for (int i = 0; i <= min(n, k); ++i) {
        for (int j = 0; j <= i; ++j) {
            vector<int> d;
            int sum = 0;
            for (int l = 0; l < i - j; ++l) {
                sum += v[l];
                d.push_back(v[l]);
            }
            for (int l = 0; l < j; ++l) {
                sum += v[n - 1 - l];
                d.push_back(v[n - 1 - l]);
            }
            sort(d.begin(), d.end());
            for (int l = 0; l < d.size() && l < k - i; ++l) {
                if (d[l] < 0) sum -= d[l];
            }
            answer = max(answer, sum);
        }
    }
    cout << answer << endl;
}
