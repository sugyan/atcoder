#include <iostream>
#include <climits>
#include <algorithm>
using namespace std;

#define SIZE 250000
#define INF

unsigned long a[SIZE];
unsigned long dp[SIZE][5];

unsigned long score(unsigned long a, int t) {
	if (t == 0 || t == 4) return a;
    if (t == 2) return (a + 1) % 2;
    if (a == 0) return 2;
    return a % 2;
}

int main() {
    long l;
    unsigned long v;
    cin >> l;
    for (int i = 0; i < l; ++i) {
        cin >> a[i];
    }
    for (int i = 0; i <= l; ++i) {
        for (int j = 0; j < 5; ++j) {
            dp[i][j] = numeric_limits<unsigned long>::max();
        }
    }
    for (int j = 0; j < 5; ++j) dp[0][j] = 0;
    for (int i = 0; i < l; ++i) {
        for (int j = 0; j < 5; ++j) {
            if (j > 0) {
                dp[i][j] = min(dp[i][j], dp[i][j - 1]);
            }
            dp[i + 1][j] = min(dp[i + 1][j], dp[i][j] + score(a[i], j));
        }
    }

    unsigned long minimum = numeric_limits<unsigned long>::max();
    for (int j = 0; j < 5; ++j) {
        minimum = min(minimum, dp[l][j]);
    }
    cout << minimum << endl;
}