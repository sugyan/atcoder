#include <iostream>

using namespace std;

long b[100005];

int main() {
    int n, a;
    long k;
    cin >> n >> k;
    b[0] = 0;
    for (int i = 0; i < n; ++i) {
        cin >> a;
        b[i + 1] = b[i] + a;
    }
    long answer = 0;
    int j = 0;
    for (int i = 0; i < n; ++i) {
        while (j < n + 1 && b[j] - b[i] < k) ++j;
        answer += n - j + 1;
    }
    cout << answer << endl;
}
