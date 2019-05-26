#include <iostream>
#include <vector>
#include <map>
#include <algorithm>

using namespace std;

int main() {
    int n;
    cin >> n;
    map<string, vector<pair<int, int>>> m;
    for (int i = 0; i < n; ++i) {
        string s;
        int p;
        cin >> s >> p;
        m[s].push_back({ p, i });
    }
    for (pair<string, vector<pair<int, int>>> p : m) {
        vector<pair<int, int>> v = p.second;
        sort(v.rbegin(), v.rend());
        for (pair<int, int> pp : v) {
            cout << pp.second + 1 << endl;
        }
    }
}
