#include <iostream>
#include <iomanip>

using namespace std;

int main() {
    int n, k;
    double answer = 0.0;
    cin >> n >> k;
    for (int i = 0; i < n; ++i) {
        int score = i + 1;
        double d = 1.0 / n;
        while (score < k) {
            d /= 2.0;
            score <<= 1;
        }
        answer += d;
    }
    cout << fixed << setprecision(12) << answer << endl;
}
