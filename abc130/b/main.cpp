#include <iostream>
#include <vector>

using namespace std;

int main() {
    int n, x;
    cin >> n >> x;
    vector<int> l(n);
    for (int i = 0; i < n; ++i) {
        cin >> l[i];
    }
    int answer = 1;
    int d = 0;
    for (int ll : l) {
        d += ll;
        if (d > x) break;
        ++answer;
    }
    cout << answer << endl;
}
