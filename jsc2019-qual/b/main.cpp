#include <iostream>
#include <vector>

using namespace std;

int main() {
    unsigned long long n, k;
    cin >> n >> k;
    vector<int> a(n);
    for (int i = 0; i < n; ++i) {
        cin >> a[i];
    }
    unsigned long long a1 = 0, a2 = 0;
    for (int i = 0; i < n - 1; ++i) {
        for (int j = i + 1; j < n; ++j) {
            if (a[i] > a[j]) ++a1;
            if (a[i] < a[j]) ++a2;
        }
    }
    unsigned long long answer = a1 * (k + 1) + a2 * (k - 1);
    if (answer % 2 == 0) {
        answer /= 2;
    } else {
        k /= 2;
    }
    answer %= 1000000007;
    cout << (answer * k) % 1000000007 << endl;
}
