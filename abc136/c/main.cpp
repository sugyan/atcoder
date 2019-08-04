#include <iostream>
#include <vector>

using namespace std;

int main() {
    int n;
    cin >> n;
    vector<int> h(n);
    for (int i = 0; i < n; ++i) {
        cin >> h[i];
    }
    bool answer = true;
    int count = 0;
    for (int i = n - 2; i >= 0; --i) {
        if (h[i] > h[i + 1]) {
            if (h[i] - h[i + 1] > 1) {
                answer = false;
                break;
            }
            if (++count > 1) {
                answer = false;
                break;
            }
        }
        if (h[i] < h[i + 1]) count = 0;
    }
    cout << (answer ? "Yes" : "No") << endl;
}
