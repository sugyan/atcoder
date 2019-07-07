#include <iostream>
#include <cmath>

using namespace std;

int main() {
    int l, r;
    cin >> l >> r;
    int answer = 0;
    if ((l / 2019) != (r / 2019)) {
        answer = 0;
    } else {
        answer = 2019;
        for (int i = l; i < r; ++i) {
            for (int j = i + 1; j <= r; ++j) {
                int mod = (long(i) * j) % 2019;
                answer = min(answer, mod);
            }
        }
    }
    cout << answer << endl;
}
