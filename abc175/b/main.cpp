#include <iostream>
#include <map>
#include <vector>

using namespace std;

int main() {
    int n;
    cin >> n;
    map<int, int> m;
    for (int i = 0; i < n; ++i) {
        int l;
        cin >> l;
        ++m[l];
    }
    vector<int> v;
    for (pair<int, int> p : m) {
        v.push_back(p.first);
    }
    int answer = 0;
    if (v.size() >= 3) {
        for (int i = 0; i < v.size() - 2; ++i) {
            for (int j = i + 1; j < v.size() - 1; ++j) {
                for (int k = j + 1; k < v.size(); ++k) {
                    if (v[i] + v[j] > v[k]) {
                        answer += m[v[i]] * m[v[j]] * m[v[k]];
                    }
                }
            }
        }
    }
    cout << answer << endl;
}
