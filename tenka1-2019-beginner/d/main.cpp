#include <bits/stdc++.h>
#define DIV 998244353

using namespace std;

long a[301];
long dp1[301][90001];
long dp2[301][90001];

int main() {
    int n, sum = 0;
    cin >> n;
    for (int i = 0; i < n; ++i) {
        cin >> a[i];
        sum += a[i];
    }

    dp1[0][0] = 1;
    dp2[0][0] = 1;
    for (int i = 0; i < n; ++i) {
        for (int j = 0; j < sum; ++j) {
            if (dp1[i][j] != 0) {
                dp1[i + 1][j] = (dp1[i + 1][j] + dp1[i][j] * 2) % DIV;
                dp1[i + 1][j + a[i]] = (dp1[i + 1][j + a[i]] + dp1[i][j]) % DIV;
            }
            if (dp2[i][j] != 0) {
                dp2[i + 1][j] = (dp2[i + 1][j] + dp2[i][j]) % DIV;
                dp2[i + 1][j + a[i]] = (dp2[i + 1][j + a[i]] + dp2[i][j]) % DIV;
            }
        }
    }
    long answer = 1;
    for (int i = 0; i < n; ++i) {
        answer = answer * 3 % DIV;
    }
    for (int i = sum; i > (sum - 1) / 2; --i) {
        long sub = dp1[n][i];
        if (i * 2 == sum) {
            sub -= dp2[n][i];
        }
        answer = (answer - 3 * sub) % DIV;
    }
    if (answer < 0) answer += DIV;
    cout << answer << endl;
}
