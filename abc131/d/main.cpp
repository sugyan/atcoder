#include <iostream>
#include <vector>
#include <map>

using namespace std;

int main() {
    int n;
    cin >> n;
    map<int, vector<int>> m;
    for (int i = 0; i < n; ++i) {
        int a, b;
        cin >> a >> b;
        m[b].push_back(a);
    }
    int t = 0;
    for (pair<int, vector<int>> p : m) {
        for (int c: p.second) {
            t += c;
            if (t > p.first) {
                cout << "No" << endl;
                return 0;
            }
        }
    }
    cout << "Yes" << endl;
}
