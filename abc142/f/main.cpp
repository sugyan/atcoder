#include <iostream>
#include <vector>

#include <iterator>

using namespace std;

int main() {
    int n, m;
    cin >> n >> m;
    vector<vector<int>> in(n);
    vector<vector<int>> out(n);
    for (int i = 0; i < m; ++i) {
        int a, b;
        cin >> a >> b;
        in[b - 1].push_back(a - 1);
        out[a - 1].push_back(b - 1);
    }
    for (int i = 0; i < n; ++i) {
        copy(out[i].begin(), out[i].end(), ostream_iterator<int>(cout, ", "));
        cout << endl;
    }
}
