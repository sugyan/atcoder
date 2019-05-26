#include <iostream>
#include <vector>

using namespace std;

int main() {
    int n, m;
    cin >> n >> m;
    vector<vector<int>> s(m);
    vector<int> p(m);
    for (int i = 0; i < m; ++i) {
        int k, v;
        cin >> k;
        for (int j = 0; j < k; ++j) {
            cin >> v;
            s[i].push_back(v - 1);
        }
    }
    for (int i = 0; i < m; ++i) {
        cin >> p[i];
    }
    int answer = 0;
    bool b[n];
    for (int i = 0; i < (1 << n); ++i) {
        for (int j = 0; j < n; ++j) b[j] = (i & (1 << j)) != 0;
        bool ok = true;
        for (int j = 0; j < m; ++j) {
            int count = 0;
            for (int k : s[j]) {
                if (b[k]) ++count;
            }
            if (count % 2 != p[j]) {
                ok = false;
                break;
            }
        }
        if (ok) ++answer;
    }
    cout << answer << endl;
}
