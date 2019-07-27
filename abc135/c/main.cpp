#include <iostream>
#include <vector>

using namespace std;

int main() {
    int n;
    cin >> n;
    vector<long> a(n + 1);
    vector<long> b(n);
    for (int i = 0; i < n + 1; ++i) {
        cin >> a[i];
    }
    for (int i = 0; i < n; ++i) {
        cin >> b[i];
    }
    long answer = 0;
    for (int i = 0; i < n; ++i) {
        if (b[i] >= a[i]) {
            answer += a[i];
            b[i] -= a[i];
            if (b[i] < a[i + 1]) {
                answer += b[i];
                a[i + 1] -= b[i];
            } else {
                answer += a[i + 1];
                a[i + 1] = 0;
            }
        } else {
            answer += b[i];
        }
    }
    cout << answer << endl;
}
