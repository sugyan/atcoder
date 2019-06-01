#include <iostream>
#include <algorithm>
#include <vector>
#include <unordered_map>
#include <queue>

using namespace std;

int main() {
    int n;
    cin >> n;
    unordered_map<int, vector<int>> um;
    for (int i = 0; i < n - 1; ++i) {
        int a, b;
        cin >> a >> b;
        um[a].push_back(b);
        um[b].push_back(a);
    }
    int start;
    for (int i = 0; i < n; ++i) {
        if (um[i + 1].size() > 1) {
            start = i + 1;
            break;
        }
    }

    vector<int> v(n, 0);
    queue<int> q;
    q.push(start);
    int i = 1;
    while (!q.empty()) {
        int target = q.front();
        q.pop();
        v[target - 1] = i++;
        for (int next : um[target]) {
            if (um[next].size() > 1) {
                if (v[next - 1] == 0) q.push(next);
            }
        }
    }
    for (int j = 0; j < n; ++j) {
        if (v[j] == 0) v[j] = i++;
    }

    vector<int> c(n);
    int sum = 0;
    int m = 0;
    for (int i = 0; i < n; ++i) {
        cin >> c[i];
        sum += c[i];
        m = max(m, c[i]);
    }
    sort(c.rbegin(), c.rend());
    cout << sum - m << endl;
    for (int i = 0; i < n; ++i) {
        cout << c[v[i] - 1];
        if (i != n - 1) cout << " ";
    }
    cout << endl;
}
