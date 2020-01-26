#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;

int main() {
    int n, k;
    cin >> n >> k;
    vector<long long> h(n);
    for (int i = 0; i < n; ++i) {
        cin >> h[i];
    }
    sort(h.begin(), h.end());
    long long answer = 0;
    for (int i = 0; i < n - k; ++i) {
        answer += h[i];
    }
    cout << answer << endl;
}
