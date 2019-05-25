#include <iostream>
#include <vector>

using namespace std;

void color(vector<vector<pair<int, bool>>>& t, vector<int>& answer, int i) {
    for (pair<int, bool> p : t[i]) {
        if (answer[p.first] == -1) {
            if (p.second) {
                answer[p.first] = answer[i];
            } else {
                answer[p.first] = 1 - answer[i];
            }
            color(t, answer, p.first);
        }
    }
}

int main() {
    int n;
    cin >> n;
    vector<vector<pair<int, bool>>> t(n);
    for (int i = 0; i < n - 1; ++i) {
        int u, v, w;
        cin >> u >> v >> w;
        t[u - 1].push_back({ v - 1, w % 2 == 0 });
        t[v - 1].push_back({ u - 1, w % 2 == 0 });
    }
    vector<int> answer(n, -1);
    for (int i = 0; i < n; ++i) {
        if (answer[i] == -1) {
            answer[i] = 0;
            color(t, answer, i);
        }
    }
    for (int n : answer) cout << n << endl;
}
