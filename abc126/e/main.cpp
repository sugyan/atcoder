#include <iostream>
#include <vector>

using namespace std;

void color(vector<vector<pair<int, bool>>>& t, vector<int>& a, int i) {
    for (pair<int, bool> p : t[i]) {
        if (a[p.first] == -1) {
            if (p.second) {
                a[p.first] = a[i];
            } else {
                a[p.first] = 1 - a[i];
            }
            color(t, a, p.first);
        }
    }
}

int main() {
    int n, m;
    cin >> n >> m;
    vector<vector<pair<int, bool>>> t(n);
    for (int i = 0; i < m; ++i) {
        int x, y, z;
        cin >> x >> y >> z;
        t[x - 1].push_back({ y - 1, z % 2 == 0 });
        t[y - 1].push_back({ x - 1, z % 2 == 0 });
    }
    int answer = 0;
    vector<int> a(n, -1);
    for (int i = 0; i < n; ++i) {
        if (a[i] == -1) {
            a[i] = 0;
            color(t, a, i);
            ++answer;
        }
    }
    cout << answer << endl;
}
