#include <iostream>

using namespace std;

int main() {
    int n;
    double x, result = 0.0;
    string u;
    cin >> n;
    for (int i = 0; i < n; ++i) {
        cin >> x >> u;
        if (u == "JPY") {
            result += x;
        }
        if (u == "BTC") {
            result += x * 380000.0;
        }
    }
    cout << result << endl;
}