#include <iostream>
#include <iomanip>

using namespace std;

int main() {
    int n;
    cin >> n;
    double d;
    for (int i = 0; i < n; ++i) {
        int a;
        cin >> a;
        d += 1.0 / a;
    }
    cout << setprecision(7) << 1.0 / d << endl;
}
