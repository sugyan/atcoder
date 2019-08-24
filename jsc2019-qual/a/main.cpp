#include <iostream>

using namespace std;

int main() {
    int m, d;
    cin >> m >> d;
    int answer;
    for (int i = 4; i <= m; ++i) {
        for (int j = 22; j <= d; ++j) {
            int d1 = j / 10, d2 = j % 10;
            if (d2 >= 2 && d1 * d2 == i) {
                ++answer;
            }
        }
    }
    cout << answer << endl;
}
