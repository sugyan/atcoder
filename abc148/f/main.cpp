#include <iostream>
#include <queue>
#include <unordered_map>
#include <vector>

using namespace std;

int solve(unordered_map<int, vector<int>>& um, int u, int v) {
    unordered_map<int, int> ud, vd;
    {
        queue<pair<int, int>> d;
        d.push({u, 0});
        while (!d.empty()) {
            pair<int, int> p = d.front();
            ud[p.first] = p.second;
            d.pop();
            for (auto e : um[p.first]) {
                if (ud.find(e) == ud.end()) {
                    d.push({e, p.second + 1});
                }
            }
        }
    }
    {
        queue<pair<int, int>> d;
        d.push({v, 0});
        while (!d.empty()) {
            pair<int, int> p = d.front();
            vd[p.first] = p.second;
            d.pop();
            for (auto e : um[p.first]) {
                if (vd.find(e) == vd.end()) {
                    d.push({e, p.second + 1});
                }
            }
        }
    }
    int ret = 0;
    for (auto p : um) {
        if (ud[p.first] < vd[p.first]) {
            ret = max(ret, vd[p.first] - 1);
        }
    }
    return ret;
}

int main() {
    int n, u, v;
    cin >> n >> u >> v;
    unordered_map<int, vector<int>> um;
    for (int i = 0; i < n - 1; ++i) {
        int a, b;
        cin >> a >> b;
        um[a].push_back(b);
        um[b].push_back(a);
    }
    cout << solve(um, u, v) << endl;
}
