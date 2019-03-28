#include <iostream>
#include <vector>

using namespace std;

int dfs(vector<pair<int, int> > v, int x) {
    pair<int, int> p = v[0];
    if (v.size() == 1) {
        if (p.first * p.second >= x) {
            return 1;
        } else {
            return 0;
        }
    }
    int result = 0;
    v.erase(v.begin());
    for (int i = 0; i < p.second + 1; ++i) {
        if (p.first * i > x) {
            break;
        }
        result += dfs(v, x - i * p.first);
    }
    return result;
}

int main() {
    int a, b, c, x;
    cin >> a >> b >> c >> x;
    vector<pair<int, int> > v = { {500, a}, {100, b}, {50, c} };
    cout << dfs(v, x) << endl;
}
