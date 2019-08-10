#include <iostream>
#include <vector>
#include <algorithm>
#include <map>

using namespace std;

int main() {
    int n, m;
    cin >> n >> m;
    map<int, vector<int>> mm;
    for (int i = 0; i < n; ++i) {
        int a, b;
        cin >> a >> b;
        mm[b].push_back(a);
    }
    long answer = 0;
    vector<bool> d(m + 1, false);
    for (auto it = mm.rbegin(); it != mm.rend(); ++it) {
        sort(it->second.begin(), it->second.end());
        int available = it->second[0];
        while (available < m + 1 && d[available]) ++available;
        for (int dd: it->second) {
            int ddd = max(dd, available);
            while (ddd < m + 1 && d[ddd]) ++ddd;
            if (ddd < m + 1) {
                answer += it->first;
                d[ddd] = true;
                while (available < m + 1 && d[available]) ++available;
            } else {
                break;
            }
        }
    }
    cout << answer << endl;
}
