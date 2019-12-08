#include <iostream>
#include <vector>
#define DIV 1000000007

using namespace std;

int main() {
    int n;
    cin >> n;
    vector<long long> v(n);
    for (int i = 0; i < n; ++i) cin >> v[i];
    long long answer = 0;
    for (int i = 60; i >= 0; --i) {
        int n = 0, m = 0;
        for (long long a : v) {
            if (((a >> i) & 1) > 0) {
                ++n;
            } else {
                ++m;
            }
        }
        answer = (answer * 2 + 1L * n * m) % DIV;
    }
    cout << answer % DIV << endl;
}
