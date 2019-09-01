#include <iostream>
#include <vector>

using namespace std;

int main() {
    int n;
    cin >> n;
    vector<int> h(n);
    for (int i = 0; i < n; ++i) cin >> h[i];
    int answer = 0, m = 0;
    for (int i = n - 2; i >= 0; --i) {
        if (h[i] >= h[i + 1]) ++m;
        else {
            answer = max(answer, m);
            m = 0;
        }
    }
    cout << max(answer, m) << endl;
}
