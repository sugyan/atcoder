#include <iostream>
#include <iomanip>
#include <cmath>

using namespace std;

int main() {
    int a, b, x;
    double answer;
    cin >> a >> b >> x;
    if (0.5 * a * a * b >= 1.0 * x) {
        answer = atan2(1.0 * a * b * b, 2.0 * x);
    } else {
        answer = atan2(2.0 * b - (2.0 * x / a / a), a);
    }
    cout << setprecision(10) << answer / M_PI * 180.0 << endl;
}
